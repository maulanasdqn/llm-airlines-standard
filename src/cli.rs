use std::ffi::OsString;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::adopt::{self, PROFILES};
use crate::build;
use crate::check::output::{compute_exit_code, write_report, Format};
use crate::check::{code, commit, prose};
use crate::error::Result;
use crate::install::selection::Console;
use crate::install::{self, format_listing};
use crate::spec::load_standard;

const USAGE_ERROR: u8 = 2;

#[derive(Parser)]
#[command(
    name = "ste",
    version,
    about = "Check prose and code against STE and STE-C, install the STE skills, and set up a repository."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Find breaks of the W rules and the C rules.")]
    Check {
        #[command(subcommand)]
        kind: CheckKind,
    },
    #[command(about = "Install the STE skills for Claude Code, Codex, Cursor, and other agents.")]
    Install(InstallArgs),
    #[command(
        about = "Set up STE and STE-C in a repository: AGENTS.md, CLAUDE.md, LEXICON.md, linters, and a commit hook."
    )]
    Adopt(AdoptArgs),
    #[command(about = "Generate the skills, the templates, and the linter configs from spec/.")]
    Build(BuildArgs),
}

#[derive(Subcommand)]
enum CheckKind {
    #[command(about = "Find breaks of the W rules in Markdown and text files.")]
    Prose(PathArgs),
    #[command(about = "Find breaks of the C rules in TypeScript, JavaScript, Python, Go, and Rust files.")]
    Code(PathArgs),
    #[command(about = "Find breaks of rule C9.4 and the W rules in a commit message.")]
    Commit(CommitArgs),
    #[command(about = "Write a draft LEXICON.md from the names in the code.")]
    Lexicon(PathArgs),
}

#[derive(Args)]
struct CommonArgs {
    #[arg(long, help = "A .ste.json file. The default is the nearest one.")]
    config: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = Format::Text, help = "The output format.")]
    format: Format,
    #[arg(long = "strict", help = "Fail on warnings as well as errors.")]
    is_strict: bool,
}

#[derive(Args)]
struct PathArgs {
    #[arg(default_value = ".", help = "Files or folders.")]
    paths: Vec<PathBuf>,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Args)]
struct CommitArgs {
    #[arg(help = "A commit message file, or - for the standard input.")]
    source: String,
    #[command(flatten)]
    common: CommonArgs,
}

#[derive(Args)]
struct InstallArgs {
    #[arg(long = "list", help = "List the skills and the targets, then stop.")]
    is_listing: bool,
    #[arg(long, help = "Skill names, separated by commas, or \"all\".")]
    skills: Option<String>,
    #[arg(long = "target", value_parser = ["claude-user", "claude-project", "agents-user", "agents-project"], help = "A target folder. You can give it more than once.")]
    targets: Vec<String>,
    #[arg(long = "dir", help = "Another skills folder. You can give it more than once.")]
    folders: Vec<PathBuf>,
    #[arg(long, default_value = ".", help = "The project folder for the project targets.")]
    project: PathBuf,
    #[arg(long = "force", help = "Replace a skill folder that has local changes.")]
    is_forced: bool,
    #[arg(long = "dry-run", help = "Show the changes. Change no file.")]
    is_dry_run: bool,
}

#[derive(Args)]
struct AdoptArgs {
    #[arg(long, value_parser = PROFILES, help = "The AGENTS.md profile. The default is the main language of the folder.")]
    profile: Option<String>,
    #[arg(long = "dir", default_value = ".", help = "The repository folder.")]
    folder: PathBuf,
    #[arg(long = "linters", help = "Also write the linter config of the profile.")]
    has_linters: bool,
    #[arg(long = "hook", help = "Also install the commit-msg hook.")]
    has_hook: bool,
    #[arg(long = "dry-run", help = "Show the changes. Change no file.")]
    is_dry_run: bool,
}

#[derive(Args)]
struct BuildArgs {
    #[arg(long = "check", help = "Report generated files that are out of date. Change no file.")]
    is_check: bool,
    #[arg(long, default_value = ".", help = "The root folder of the llm-airlines-standard repository.")]
    root: PathBuf,
}

fn write_lexicon(arguments: &PathArgs) -> Result<u8> {
    let standard = load_standard(arguments.common.config.as_deref())?;
    println!("{}", code::run_lexicon(&arguments.paths, &standard)?);
    Ok(0)
}

fn run_check(kind: CheckKind) -> Result<u8> {
    let (common, mut findings) = match kind {
        CheckKind::Lexicon(arguments) => return write_lexicon(&arguments),
        CheckKind::Prose(arguments) => {
            let standard = load_standard(arguments.common.config.as_deref())?;
            let findings = prose::run(&arguments.paths, &standard)?;
            (arguments.common, findings)
        }
        CheckKind::Code(arguments) => {
            let standard = load_standard(arguments.common.config.as_deref())?;
            let findings = code::run(&arguments.paths, &standard)?;
            (arguments.common, findings)
        }
        CheckKind::Commit(arguments) => {
            let standard = load_standard(arguments.common.config.as_deref())?;
            let findings = commit::run(&arguments.source, &standard)?;
            (arguments.common, findings)
        }
    };
    write_report(&mut findings, common.format)?;
    Ok(compute_exit_code(&findings, common.is_strict))
}

fn run_command(command: Command) -> Result<u8> {
    match command {
        Command::Check { kind } => run_check(kind),
        Command::Install(arguments) if arguments.is_listing => {
            println!("{}", format_listing());
            Ok(0)
        }
        Command::Install(arguments) => {
            let request = install::Request {
                skills: arguments.skills,
                targets: arguments.targets,
                folders: arguments.folders,
                project: arguments.project,
                is_forced: arguments.is_forced,
                is_dry_run: arguments.is_dry_run,
            };
            install::run(&request, Console { input: Box::new(BufReader::new(std::io::stdin())) })
        }
        Command::Adopt(arguments) => adopt::run(&adopt::Request {
            profile: arguments.profile,
            folder: arguments.folder,
            has_linters: arguments.has_linters,
            has_hook: arguments.has_hook,
            is_dry_run: arguments.is_dry_run,
        }),
        Command::Build(arguments) => build::run(&arguments.root, arguments.is_check),
    }
}

pub fn run<I: IntoIterator<Item = OsString>>(arguments: I) -> ExitCode {
    let cli = match Cli::try_parse_from(arguments) {
        Ok(cli) => cli,
        Err(error) => {
            let _ = error.print();
            return ExitCode::from(if error.use_stderr() { USAGE_ERROR } else { 0 });
        }
    };
    match run_command(cli.command) {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("ste: {error}");
            ExitCode::from(USAGE_ERROR)
        }
    }
}
