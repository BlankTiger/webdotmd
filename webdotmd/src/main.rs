use std::path::Path;

use libwebdotmd::{load_markdown_pages, write_html_pages};
use webdotx::{load_templates, render};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = parse_args();
    let templates_path = Path::new("templates");
    let pages_path = Path::new("pages");
    let output_path = Path::new("output");
    let templates = load_templates(templates_path, Some("html"))?;
    let md_pages = load_markdown_pages(pages_path)?;
    let html_pages = render(&md_pages, &templates);
    clear_output_directory(None)?;
    write_html_pages(&html_pages, pages_path, output_path)?;
    let assets = Path::new("assets");
    copy_files_from_dir_to_dir(assets, output_path)?;
    Ok(())
}

fn clear_output_directory(output_dir: Option<&Path>) -> Result<(), std::io::Error> {
    let output_dir = output_dir.unwrap_or(Path::new("output"));
    assert!(output_dir.is_dir());
    std::fs::remove_dir_all(output_dir)?;
    std::fs::create_dir(output_dir)?;
    Ok(())
}

pub fn copy_files_from_dir_to_dir(source_dir: &Path, target_dir: &Path) -> Result<(), std::io::Error> {
    let source_prefix = source_dir.to_str().unwrap();
    for entry in std::fs::read_dir(source_dir)? {
        let source_path = entry?.path();
        let path = source_path.strip_prefix(source_prefix).unwrap();
        let target_path = target_dir.join(path);
        if target_path.is_dir() {
            std::fs::create_dir(target_path)?;
            continue;
        }
        if !source_path.is_dir() {
            let source_bytes = std::fs::read(&source_path)?;
            std::fs::write(target_path, source_bytes)?;
        }
    }
    Ok(())
}
