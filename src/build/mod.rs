pub mod outputs;
pub mod plan;
pub mod skills;
pub mod templating;
pub mod views;

use std::path::Path;

use crate::error::Result;
use outputs::find_drift;
use plan::build_outputs;

pub fn run(root: &Path, is_check: bool) -> Result<u8> {
    let outputs = build_outputs(root)?;
    let problems = skills::validate_all(root);
    for problem in &problems {
        eprintln!("{problem}");
    }
    if !is_check {
        for line in outputs::write(root, &outputs)? {
            println!("{line}");
        }
        return Ok(u8::from(!problems.is_empty()));
    }
    let drift = find_drift(root, &outputs);
    for line in &drift {
        eprintln!("{line}");
    }
    if !drift.is_empty() {
        eprintln!("ste build: generated files are out of date. Run ste build.");
    }
    Ok(u8::from(!drift.is_empty() || !problems.is_empty()))
}
