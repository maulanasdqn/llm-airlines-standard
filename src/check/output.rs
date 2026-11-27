use std::io::Write;

use super::finding::{sort_findings, Finding, Severity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Format {
    Text,
    Json,
}

fn find_severity_name(finding: &Finding) -> &'static str {
    finding.severity.map_or("warning", Severity::name)
}

pub fn format_text(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|finding| {
            let severity = find_severity_name(finding);
            format!(
                "{}:{}:{}: {severity} {}: {}",
                finding.path, finding.line, finding.column, finding.rule, finding.message
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn format_summary(findings: &[Finding]) -> String {
    let count = |severity: Severity| findings.iter().filter(|finding| finding.severity == Some(severity)).count();
    format!(
        "ste check: {} errors, {} warnings, {} infos",
        count(Severity::Error),
        count(Severity::Warning),
        count(Severity::Info)
    )
}

pub fn compute_exit_code(findings: &[Finding], is_strict: bool) -> u8 {
    let is_failure = |finding: &Finding| match finding.severity {
        Some(Severity::Error) => true,
        Some(Severity::Warning) => is_strict,
        _ => false,
    };
    u8::from(findings.iter().any(is_failure))
}

pub fn write_report(findings: &mut [Finding], format: Format) -> std::io::Result<()> {
    sort_findings(findings);
    let report = match format {
        Format::Json => serde_json::to_string_pretty(&findings).unwrap_or_else(|_| String::from("[]")),
        Format::Text => format_text(findings),
    };
    let mut output = std::io::stdout().lock();
    if !report.is_empty() {
        writeln!(output, "{report}")?;
    }
    eprintln!("{}", format_summary(findings));
    Ok(())
}
