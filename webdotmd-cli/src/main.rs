use libwebdotmd::{load_markdown_pages, load_templates};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = parse_args();
    let templates_path = std::path::Path::new("templates");
    let templates = load_templates(templates_path)?;
    dbg!(&templates);
    let pages_path = std::path::Path::new("pages");
    let md_pages = load_markdown_pages(pages_path)?;
    dbg!(&md_pages);
    // let md_pages = load_md_pages(&args.input)?;
    // let html_pages = render_html_pages(&md_pages, &templates)?;
    // write_html_pages(&html_pages, &args.output)?;
    Ok(())
}
