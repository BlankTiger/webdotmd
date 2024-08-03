use libwebdotmd::load_templates;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let args = parse_args();
    let path = std::path::Path::new("templates");
    let templates = load_templates(path)?;
    dbg!(templates);
    // let md_pages = load_md_pages(&args.input)?;
    // let html_pages = render_html_pages(&md_pages, &templates)?;
    // write_html_pages(&html_pages, &args.output)?;
    Ok(())
}
