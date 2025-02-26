use globset::{Glob, GlobSet, GlobSetBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Invalid glob pattern: {0}")]
    InvalidGlobPattern(#[from] globset::Error),
}

pub fn build_ignore_set(patterns: &[String]) -> Result<GlobSet, ConfigError> {
    let mut builder = GlobSetBuilder::new();

    let default_ignores = &[
        "*.exe", "*.dll", "*.so", "*.dylib", "*.bin", "*.dat", "*.jar", "*.o", "*.a", "*.lib",
        "*.zip", "*.tar*", "*.gz", "*.bz2", "*.xz", "*.7z", "*.rar", "*.deb", "*.rpm", "*.jpg",
        "*.jpeg", "*.png", "*.gif", "*.bmp", "*.svg", "*.ico", "*.webp", "*.psd", "*.tiff",
        "*.mp3", "*.wav", "*.ogg", "*.m4a", "*.flac", "*.aac", "*.mp4", "*.avi", "*.mkv", "*.mov",
        "*.wmv", "*.flv", "*.webm", "*.ppt*", "*.ods", "*.odp", "*.sys", "*.dmp", "*.pak", "*.cab",
    ];

    for pattern in default_ignores {
        builder.add(Glob::new(pattern)?);
    }

    for user_pattern in patterns {
        let clean_pattern = user_pattern.trim();
        if !clean_pattern.is_empty() {
            // Add the pattern as provided by the user.
            builder.add(Glob::new(clean_pattern)?);
            // If the pattern does not contain a slash, add a recursive variant.
            if !clean_pattern.contains('/') {
                // This variant matches the pattern in any subdirectory.
                builder.add(Glob::new(&format!("**/{}", clean_pattern))?);
                // For literal patterns (without wildcards), also ignore the directory and its contents.
                if !clean_pattern.contains('*')
                    && !clean_pattern.contains('?')
                    && !clean_pattern.contains('[')
                {
                    builder.add(Glob::new(&format!("**/{}/**", clean_pattern))?);
                }
            }
        }
    }

    Ok(builder.build()?)
}
