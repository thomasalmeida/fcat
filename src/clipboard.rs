use anyhow::Result;
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(content: &str) -> Result<()> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to access clipboard (wl-copy required): {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        std::io::Write::write_all(&mut stdin, content.as_bytes())?;
    }

    child.wait()?;
    Ok(())
}
