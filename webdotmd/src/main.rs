use std::{collections::HashMap, path::Path};

use libwebdotmd::{load_markdown_pages, write_html_pages};
use webdotx::{load_template, load_templates, render, FuncMap};

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
    autofill_funcs.insert("article_entry_list", &create_article_entry_list);
    autofill_funcs.insert("favicon_trash", &create_favicon_trash);
    autofill_funcs.insert("tw_classes_push_footer", &get_classes_to_push_footer_down);
    autofill_funcs.insert("link_classes", &get_link_classes);
    autofill_funcs.insert("body_classes", &get_body_classes);
    autofill_funcs.insert("default_theme", &get_default_theme);
    autofill_funcs.insert("main_opening", &main_opening);
    autofill_funcs.insert("main_closing", &main_closing);

    autofill_funcs
}

fn main_opening() -> String {
    r##"<main class="w-full flex justify-center text-justify py-4 text-base leading-relaxed h-fit">
<div class="flex flex-col justify-start items-center max-w-prose min-w-[40%] bg-l-bg-secondary dark:bg-d-bg-secondary rounded-xl border">
<div class="w-5/6">
"##.to_string()
}

fn main_closing() -> String {
    "</div>
</div>
</main>
"
    .to_string()
}

fn create_navbar() -> String {
    let link_classes = get_link_classes();
    format!(
        r##"<nav class="flex justify-between items-center p-4 px-8 max-h-16 bg-l-bg-accent dark:bg-d-bg-accent">
<a href="https://maciejurban.dev"><img src="website-logo.svg" class="object-contain w-20"/></a>
<div class="flex justify-between w-60">
    <a href="index.html" class="{link_classes}">Home</a>
    <a href="articles.html" class="{link_classes}">Articles</a>
    <a href="https://github.com/BlankTiger" class="{link_classes}">GitHub</a>
</div>
</nav>
<hr/>
"##
    )
}

fn get_link_classes() -> String {
    "text-l-accent 
    hover:text-l-accent-secondary 
    dark:text-d-accent 
    dark:hover:text-d-accent-secondary"
        .to_string()
}

fn create_footer() -> String {
    // TODO: think on how to resolve placeholders in autofill_funcs
    // TODO: fix how footer is positioned with css grid
    r#"
<footer class="w-full bg-l-bg-accent dark:bg-d-bg-accent">
    <hr/>
    <div class="p-4 px-8">© Maciej Urban 2024</div>
</footer>"#
        .to_string()
}

struct CardWithDate {
    card: String,
    date: chrono::NaiveDate,
}

fn create_article_entry_list() -> String {
    // let templates_path = Path::new("templates");
    // let templates = load_templates(templates_path, Some("html")).unwrap();
    let card_template = load_template(Path::new("templates/elements/article_card.html")).unwrap();
    let pages_path = Path::new("pages");
    let md_pages = load_markdown_pages(pages_path).unwrap();
    let mut list: Vec<CardWithDate> = Vec::new();
    for (name, page) in md_pages {
        if let Some(template_name) = page.get_metadata("template") {
            if template_name == "templates/article_entry.html" {
                let (_, href) = name.split_once('/').unwrap();
                let href = href.replace(".md", ".html");
                let page_title = page.get_metadata("title").unwrap();
                let date = page.get_metadata("date").unwrap().to_string();
                let filled_placeholders = HashMap::from([
                    ("title".to_string(), page_title.to_string()),
                    ("summary".to_string(), page.get_metadata("summary").unwrap().to_string()),
                    ("date".to_string(), date.clone()),
                    (
                        "time_to_read".to_string(),
                        page.get_metadata("time_to_read").unwrap().to_string(),
                    ),
                    ("link".to_string(), href),
                ]);
                let autofill_funcs = create_autofill_funcs();
                let entry = card_template.fill_template(filled_placeholders, &Some(autofill_funcs));
                let (year, month, day) = parse_year_month_date_from_str(&date);
                list.push(CardWithDate {
                    card: entry,
                    date: chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap(),
                });
            }
        }
    }
    list.sort_by(|a, b| b.date.cmp(&a.date));
    list.iter().map(|a| a.card.clone()).collect::<String>()
}

fn parse_year_month_date_from_str(date: &str) -> (i32, u32, u32) {
    let (year, rest) = date.split_once("-").unwrap();
    let (month, day) = rest.split_once("-").unwrap();
    (year.parse().unwrap(), month.parse().unwrap(), day.parse().unwrap())
}

fn create_favicon_trash() -> String {
    String::from(
        r##"<link rel="apple-touch-icon" sizes="180x180" href="apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="favicon-32x32.png">
<link rel="icon" type="image/png" sizes="16x16" href="favicon-16x16.png">
<link rel="manifest" href="site.webmanifest">
"##,
    )
}

fn get_classes_to_push_footer_down() -> String {
    String::from("grid min-h-[100dvh] grid-rows-[auto_1fr_auto]")
}

fn get_body_classes() -> String {
    String::from("bg-l-bg text-l-text dark:bg-d-bg dark:text-d-text")
}

fn get_default_theme() -> String {
    String::from("dark")
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
