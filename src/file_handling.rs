use anyhow::{Context, Result};
use encoding_rs::UTF_8;
use std::{fs::File, io::Read, path::Path};
use walkdir::WalkDir;

pub fn process_files(paths: &[String], ignore_patterns: &[String]) -> Result<String> {
    let mut output = String::new();
    let globset = crate::config::build_ignore_set(ignore_patterns)?;

    for path in paths {
        let base_path = Path::new(path);
        if !base_path.exists() {
            return Err(anyhow::anyhow!("Path not found: {}", path));
        }

        for entry in WalkDir::new(base_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            if entry.file_type().is_file()
                && !globset.is_match(entry_path)
                && is_readable_text(entry_path)?
            {
                match std::fs::read(entry_path) {
                    Ok(content) => {
                        let (text, _, _) = UTF_8.decode(&content);
                        output += &format!("==== {} ====\n{}\n\n", entry_path.display(), text);
                    }
                    Err(e) => eprintln!("Skipping {}: {}", entry_path.display(), e),
                }
            }
        }
    }

    if output.is_empty() {
        Err(anyhow::anyhow!("No readable text files found"))
    } else {
        Ok(output)
    }
}

fn is_readable_text(path: &Path) -> Result<bool> {
    let metadata = path.metadata()?;
    if metadata.len() == 0 || metadata.permissions().readonly() {
        return Ok(false);
    }

    let mut buffer = [0; 1024];
    let mut file = File::open(path).context("Failed to open file")?;
    let bytes_read = file.read(&mut buffer).context("Failed to read file")?;

    if bytes_read == 0 || buffer[..bytes_read].contains(&0) {
        return Ok(false);
    }

    let sample = &buffer[..bytes_read.min(512)];
    let text = std::str::from_utf8(sample).map_err(|_| anyhow::anyhow!("Non-UTF8 content"))?;

    let printable = text
        .chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .count();

    Ok((printable as f32 / text.len() as f32) > 0.85)
}
