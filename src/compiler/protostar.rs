use anyhow::{anyhow, Result};
use execute::Execute;
use log::debug;
use std::process::Command;

pub fn compile(project_dir: &str, build_dir: &str) -> Result<()> {
    debug!(
        "Entering compile with args\nproject_dir: {}\nbuild_dir: {}",
        project_dir, build_dir
    );
    let mut compile_command = Command::new("protostar");
    compile_command.current_dir(project_dir);
    compile_command.arg("build");
    let output = compile_command.execute_output().unwrap();
    if let Some(exit_code) = output.status.code() {
        if exit_code != 0 {
            return Err(anyhow!("protostar build failed"));
        }
    } else {
        return Err(anyhow!("protostar build interrupted!"));
    }
    Ok(())
}
