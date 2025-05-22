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
    autofill_funcs.insert("code_highlighting", &get_code_highlighting);
    // TODO: add an ability to specify features for pages via tagging in metadata, for example:
    // `features: math, code`
    autofill_funcs.insert("mathjax", &get_mathjax);
    autofill_funcs.insert("outline_highlighting", &get_outline_highlighting);
    autofill_funcs.insert("tw_classes_push_footer", &get_classes_to_push_footer_down);
    autofill_funcs.insert("link_classes", &get_link_classes);
    autofill_funcs.insert("body_classes", &get_body_classes);
    autofill_funcs.insert("default_theme", &get_default_theme);
    autofill_funcs.insert("main_opening", &main_opening);
    autofill_funcs.insert("main_closing", &main_closing);

    autofill_funcs
}

fn get_outline_highlighting() -> &'static str {
    r##"<script src="OutlineHighlighter.js"></script>"##
}

fn get_mathjax() -> &'static str {
    r##"
    <script>
        MathJax = {
            chtml: {
                scale: 1,
                minScale: 1,
                exFactor: 1,
                matchFontHeight: true
            },
            tex: {
                inlineMath: [['$', '$'], ['\\(', '\\)']]
            },
            loader: { load: ["input/tex", "output/chtml"] }
        };
    </script>
    <script type="text/javascript" id="MathJax-script" async
        src="https://cdn.jsdelivr.net/npm/mathjax@3.0.0/es5/tex-chtml.js">
    </script>
    "##
}

fn get_code_highlighting() -> &'static str {
    r##"
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Fira+Code:wght@300..700&family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap" rel="stylesheet">

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/styles/tokyo-night-dark.css">
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/highlight.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/go.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/rust.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/bash.min.js"></script>
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/json.min.js"></script>
<!-- <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/toml.min.js"></script> -->
<script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.10.0/languages/lua.min.js"></script>
<script src="./HLJSLanguageDisplayPlugin.js"></script>
<script>
hljs.addPlugin(new HLJSLanguageDisplayPlugin());
hljs.highlightAll();
</script>
"##
}

fn main_opening() -> &'static str {
    r##"<main class="w-full flex justify-center text-justify py-4 text-base leading-relaxed h-fit">
<div class="flex flex-col justify-start items-center max-w-prose min-w-[40%] bg-l-bg-secondary dark:bg-d-bg-secondary rounded-xl border">
<div class="w-5/6">
"##
}

fn main_closing() -> &'static str {
    "</div>
</div>
</main>
"
}

fn create_navbar() -> &'static str {
    let link_classes = get_link_classes();
    let navbar = Box::from(format!(
        r##"<nav class="flex justify-between items-center p-4 px-8 max-h-16 bg-l-bg-accent dark:bg-d-bg-accent">
<a href="https://maciejurban.dev"><img src="website-logo.svg" class="object-contain w-20"/></a>
<div class="flex justify-between gap-6">
    <a href="/" class="{link_classes}">Home</a>
    <a href="articles.html" class="{link_classes}">Articles</a>
    <a href="https://github.com/BlankTiger" class="{link_classes}">GitHub</a>
    
    <div>
        <label class="inline-flex items-center cursor-pointer">
          <input id="themeToggle" type="checkbox" value="" class="sr-only peer" checked>
          <div class="relative w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600"></div>
        </label>
    </div>

    <script src="ThemeToggle.js"></script>
    <script src="PresetTheme.js" strategy="beforeInteractive"></script>
</div>
</nav>
<hr/>
"##
    ));
    Box::leak(navbar)
}

fn get_link_classes() -> &'static str {
    "text-l-accent 
    hover:text-l-accent-secondary 
    dark:text-d-accent 
    dark:hover:text-d-accent-secondary"
}

fn create_footer() -> &'static str {
    // TODO: think on how to resolve placeholders in autofill_funcs
    // TODO: fix how footer is positioned with css grid
    r#"
<footer class="w-full bg-l-bg-accent dark:bg-d-bg-accent">
<hr/>
<div class="p-4 px-8">© Maciej Urban 2024</div>
</footer>"#
}

struct CardWithDate {
    card: String,
    date: chrono::NaiveDate,
}

fn create_article_entry_list() -> &'static str {
    // let templates_path = Path::new("templates");
    // let templates = load_templates(templates_path, Some("html")).unwrap();
    let card_template = load_template(Path::new("templates/elements/article_card.html")).unwrap();
    let pages_path = Path::new("pages");
    let md_pages = load_markdown_pages(pages_path).unwrap();
    let mut list: Vec<CardWithDate> = Vec::new();
    for (name, page) in md_pages {
        if let Some(template_name) = page.get_metadata("template") {
            if template_name == "templates/article_entry.html" {
                if let Some(hidden) = page.get_metadata("hidden") {
                    if hidden == "true" {
                        continue;
                    }
                }
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
    let list = Box::from(list.iter().map(|a| a.card.clone()).collect::<String>());
    Box::leak(list)
}

fn parse_year_month_date_from_str(date: &str) -> (i32, u32, u32) {
    let (year, rest) = date.split_once("-").unwrap();
    let (month, day) = rest.split_once("-").unwrap();
    (year.parse().unwrap(), month.parse().unwrap(), day.parse().unwrap())
}

fn create_favicon_trash() -> &'static str {
    r##"<link rel="apple-touch-icon" sizes="180x180" href="apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="favicon-32x32.png">
<link rel="icon" type="image/png" sizes="16x16" href="favicon-16x16.png">
<link rel="manifest" href="site.webmanifest">
"##
}

fn get_classes_to_push_footer_down() -> &'static str {
    "grid min-h-[100dvh] grid-rows-[auto_1fr_auto]"
}

fn get_body_classes() -> &'static str {
    "bg-l-bg text-l-text dark:bg-d-bg dark:text-d-text"
}

fn get_default_theme() -> &'static str {
    "dark"
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
