use std::{collections::HashMap, path::Path};

use libwebdotmd::{load_markdown_pages, write_html_pages};
use webdotx::{load_templates, render, FuncMap};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = parse_args();
    let templates_path = Path::new("templates");
    let pages_path = Path::new("pages");
    let output_path = Path::new("output");
    let templates = load_templates(templates_path, Some("html"))?;
    let md_pages = load_markdown_pages(pages_path)?;
    let autofill_funcs = create_autofill_funcs();
    let html_pages = render(&md_pages, &templates, &Some(autofill_funcs));
    clear_output_directory(None)?;
    write_html_pages(&html_pages, pages_path, output_path)?;
    let assets = Path::new("assets");
    copy_files_from_dir_to_dir(assets, output_path)?;
    Ok(())
}

fn create_autofill_funcs() -> FuncMap {
    let mut autofill_funcs: FuncMap = HashMap::new();
    autofill_funcs.insert("navbar", &create_navbar);
    autofill_funcs.insert("footer", &create_footer);
    autofill_funcs.insert("blog_entry_list", &create_blog_entry_list);
    autofill_funcs
}

fn create_navbar() -> String {
    "<nav>Navbar</nav>".to_string()
}

fn create_footer() -> String {
    // TODO: think on how to resolve placeholders in autofill_funcs
    "<footer>© Maciej Urban 2024</footer>".to_string()
}

fn create_blog_entry_list() -> String {
    let pages_path = Path::new("pages");
    let md_pages = load_markdown_pages(pages_path).unwrap();
    let mut list = String::new();
    list.push_str("<ul>\n");
    for (name, page) in md_pages {
        if let Some(template_name) = page.get_metadata("template") {
            if template_name == "templates/blog_entry.html" {
                let (_, href) = name.split_once('/').unwrap();
                let href = href.replace(".md", ".html");
                let page_title = page.get_metadata("title").unwrap();
                let entry = format!("<li><a href='{href}'>{page_title}</a></li>");
                list.push_str(&entry);
            }
        }
    }
    list.push_str("</ul>\n");
    list
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
