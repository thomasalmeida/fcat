#[cfg(test)]
mod tests {
    use super::*;
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

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path().to_str().unwrap())
            .assert()
            .success()
            .stdout(predicate::str::contains("test1.txt"))
            .stdout(predicate::str::contains("test2.txt"));

        Ok(())
    }

    #[test]
    fn test_ignore_patterns() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("file.log"), "Log")?;
        std::fs::write(dir.path().join("file.txt"), "Text")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .arg("-i")
            .arg("*.log")
            .assert()
            .success()
            .stdout(predicate::str::contains("file.txt"))
            .stdout(predicate::str::contains("file.log").not());

        Ok(())
    }

    #[test]
    fn test_binary_files_ignored() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let binary_file = dir.path().join("test.bin");
        std::fs::write(&binary_file, &[0u8, 159, 146, 150])?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test.bin").not());

        Ok(())
    }

    #[test]
    fn test_output_file_creation() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let out_file = dir.path().join("output.txt");
        std::fs::write(dir.path().join("test.txt"), "Content")?;

        let mut cmd = Command::cargo_bin("fcat")?;
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
        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg("/non/existent/path")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Path not found"));

        Ok(())
    }

    #[test]
    fn test_file_processing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "Hello World")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test.txt"))
            .stdout(predicate::str::contains("Hello World"));

        Ok(())
    }

    #[test]
    fn test_multiple_ignore_patterns() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("file1.log"), "Log1")?;
        std::fs::write(dir.path().join("file2.tmp"), "Temp")?;
        std::fs::write(dir.path().join("data.txt"), "Text")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .arg("-i")
            .arg("*.log,*.tmp")
            .assert()
            .success()
            .stdout(predicate::str::contains("data.txt"))
            .stdout(predicate::str::contains("file1.log").not())
            .stdout(predicate::str::contains("file2.tmp").not());

        Ok(())
    }

    #[test]
    fn test_default_ignore_binary_files() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("image.jpg"), [0xff, 0xd8, 0xff])?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("image.jpg").not());

        Ok(())
    }

    #[test]
    fn test_empty_file_handling() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::File::create(dir.path().join("empty.txt"))?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .failure()
            .stderr(predicate::str::contains("No readable text files found"));

        Ok(())
    }

    #[test]
    fn test_basic_functionality() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file1 = dir.path().join("test1.txt");
        let file2 = dir.path().join("test2.txt");

        std::fs::write(&file1, "Hello")?;
        std::fs::write(&file2, "World")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path().to_str().unwrap())
            .assert()
            .success()
            .stdout(predicate::str::contains("test1.txt"))
            .stdout(predicate::str::contains("test2.txt"));

        Ok(())
    }

    #[test]
    fn test_ignore_patterns() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("file.log"), "Log")?;
        std::fs::write(dir.path().join("file.txt"), "Text")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .arg("-i")
            .arg("*.log")
            .assert()
            .success()
            .stdout(predicate::str::contains("file.txt"))
            .stdout(predicate::str::contains("file.log").not());

        Ok(())
    }

    #[test]
    fn test_binary_files_ignored() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let binary_file = dir.path().join("test.bin");
        std::fs::write(&binary_file, &[0u8, 159, 146, 150])?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test.bin").not());

        Ok(())
    }

    #[test]
    fn test_output_file_creation() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let out_file = dir.path().join("output.txt");
        std::fs::write(dir.path().join("test.txt"), "Content")?;

        let mut cmd = Command::cargo_bin("fcat")?;
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
        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg("/non/existent/path")
            .assert()
            .failure()
            .stderr(predicate::str::contains("Path not found"));

        Ok(())
    }

    #[test]
    fn test_file_processing() -> Result<(), Box<dyn std::error::Error>> {
        let dir = TempDir::new()?;
        let file_path = dir.path().join("test.txt");
        std::fs::write(&file_path, "Hello World")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path())
            .assert()
            .success()
            .stdout(predicate::str::contains("test.txt"))
            .stdout(predicate::str::contains("Hello World"));

        Ok(())
    }

    #[test]
    fn test_empty_file() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file = dir.path().join("empty.txt");
        std::fs::write(&file, "")?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path().to_str().unwrap())
            .assert()
            .failure()
            .stderr(predicate::str::contains("No readable text files found"));

        Ok(())
    }

    #[test]
    fn test_non_utf8_file() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let file = dir.path().join("non_utf8.txt");
        // Escreve uma sequência inválida em UTF-8 (0xC3 0x28 é inválida)
        std::fs::write(&file, &[0xC3, 0x28])?;

        let mut cmd = Command::cargo_bin("fcat")?;
        cmd.arg(dir.path().to_str().unwrap())
            .assert()
            .failure()
            .stderr(predicate::str::contains("Non-UTF8 content"));

        Ok(())
    }
}
