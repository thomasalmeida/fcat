#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn test_basic_functionality() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file1 = dir.path().join("test1.txt");
        let file2 = dir.path().join("test2.txt");

        std::fs::write(&file1, "Hello")?;
        std::fs::write(&file2, "World")?;

        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg(dir.path().to_str().unwrap())
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_ignore_patterns() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("file.log"), "Log")?;
        std::fs::write(dir.path().join("file.txt"), "Text")?;

        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg(dir.path())
            .arg("-i")
            .arg("*.log")
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_binary_files_ignored() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let binary_file = dir.path().join("test.bin");
        std::fs::write(&binary_file, &[0u8, 159, 146, 150])?;

        // Add a text file to ensure we have some output
        std::fs::write(dir.path().join("test.txt"), "Text content")?;

        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg(dir.path())
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_output_file_creation() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let out_file = dir.path().join("output.txt");
        std::fs::write(dir.path().join("test.txt"), "Content")?;

        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg(dir.path())
            .arg("-o")
            .arg(out_file.to_str().unwrap())
            .assert()
            .success();

        assert!(out_file.exists());
        Ok(())
    }

    #[test]
    fn test_invalid_path() -> anyhow::Result<()> {
        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg("/non/existent/path")
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn test_file_processing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "Hello World")?;

        let mut cmd = Command::cargo_bin("fcpy")?;
        cmd.arg(dir.path())
            .assert()
            .success();

        Ok(())
    }

    #[test]
    fn test_empty_file() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file = dir.path().

