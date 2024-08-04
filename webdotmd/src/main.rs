use libwebdotmd::load_markdown_pages;
use webdotx::{load_templates, render};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = parse_args();
    let templates_path = std::path::Path::new("templates");
    let pages_path = std::path::Path::new("pages");
    // let output_path = std::path::Path::new("output");
    let templates = load_templates(templates_path, Some("html"))?;
    let md_pages = load_markdown_pages(pages_path)?;
    let html_pages = render(&md_pages, &templates);
    dbg!(&templates);
    dbg!(&md_pages);
    dbg!(&html_pages);
    // write_html_pages(&html_pages, output_path)?;
    Ok(())
}
