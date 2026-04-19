#[test]
fn all_tests_have_version_comment() -> std::io::Result<()> {
    let tests_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let errors = collect_errors(&tests_dir)?;
    assert!(
        errors.is_empty(),
        "Missing '// since vX.Y' comments:\n{}",
        errors.join("\n")
    );
    Ok(())
}

fn collect_errors(dir: &std::path::Path) -> std::io::Result<Vec<String>> {
    let entries = std::fs::read_dir(dir)?;
    let mut errors = Vec::new();
    let mut paths = entries.flatten().map(|e| e.path()).collect::<Vec<std::path::PathBuf>>();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            errors.extend(collect_errors(&path)?);
        } else if path.extension().map_or(false, |e| e == "rs") {
            errors.extend(check_file(&path)?);
        }
    }
    Ok(errors)
}

fn check_file(path: &std::path::Path) -> std::io::Result<Vec<String>> {
    if path.file_name() == Some(std::ffi::OsStr::new("lint.rs")) {
        return Ok(Vec::new());
    }

    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut errors = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("#[test]") || trimmed.starts_with("#[tokio::test]") {
            let start = i.saturating_sub(2);
            let has_version = lines[start..i]
                .iter()
                .any(|l| l.trim().starts_with("// since v"));
            if !has_version {
                errors.push(format!(
                    "{}:{}: missing '// since v' comment",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
    Ok(errors)
}
