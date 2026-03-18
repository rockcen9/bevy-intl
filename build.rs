use std::error::Error;
use std::{ fs, path::Path, path::PathBuf };
use serde_json::{ Value, Map };
use anyhow::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let messages_dir = find_messages_directory("messages")?;
    let out_path = Path::new(&std::env::var("OUT_DIR")?).join("all_translations.json");

    // Always create the file, even if empty, so include_str! works
    if !messages_dir.exists() {
        println!("cargo:warning=No messages/ folder found in consuming project");
        println!("cargo:warning=This is normal when building bevy-intl itself");
        fs::write(out_path, "{}")?;
        return Ok(());
    }

    let translations = build_translations(&messages_dir)?;
    fs::write(out_path, serde_json::to_string_pretty(&translations)?)?;

    println!("cargo:rerun-if-changed=messages");
    Ok(())
}

fn build_translations(messages_dir: &Path) -> Result<Value> {
    let mut translations = Map::new();

    for lang_entry in fs::read_dir(messages_dir)? {
        let lang_dir = lang_entry?;
        if !lang_dir.file_type()?.is_dir() {
            continue;
        }

        let lang_code = lang_dir.file_name().to_string_lossy().to_string();
        let mut translation_files = Map::new();

        for file_entry in fs::read_dir(lang_dir.path())? {
            let file = file_entry?;
            let file_path = file.path(); // Store the path to extend its lifetime

            if let Some("json") = file_path.extension().and_then(|e| e.to_str()) {
                let file_stem = file_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let content = fs::read_to_string(&file_path)?;
                let json: Value = serde_json::from_str(&content)?;
                translation_files.insert(file_stem.to_string(), json);
            }
        }
        translations.insert(lang_code, Value::Object(translation_files));
    }

    Ok(Value::Object(translations))
}

fn find_messages_directory(folder_name: &str) -> Result<PathBuf> {
    // Walk up from OUT_DIR — it's always rooted inside the consuming project's
    // target/ folder, so ascending will eventually reach the project root.
    // e.g. .../bevy_fishing_horror_jam/target/debug/build/bevy-intl-xxx/out/
    if let Ok(out_dir) = std::env::var("OUT_DIR") {
        let mut current = PathBuf::from(out_dir);
        while current.pop() {
            let candidate = current.join(folder_name);
            if candidate.exists() && candidate.is_dir() {
                return Ok(candidate);
            }
        }
    }

    // Fallback: folder relative to CWD (works when building bevy-intl itself)
    Ok(PathBuf::from(folder_name))
}
