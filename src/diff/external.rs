pub fn use_external_diff_tool(
    diff: String,
    diff_tool: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(diff_tool)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(diff.as_bytes())?;
    }

    let status = child.wait()?;
    if !status.success() {
        eprintln!(
            "Error: Diff tool '{}' exited with status: {}",
            diff_tool, status
        );
    }

    Ok(())
}
