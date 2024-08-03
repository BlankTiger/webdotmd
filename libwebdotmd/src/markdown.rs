use crate::utils::load_files_in_dir_to_string;
use crate::{Renderable, Template};
use std::collections::HashMap;
use std::path::Path;

pub struct MarkdownPage {
    /// starts from first line in the file and ends on line :content:
    metadata: Metadata,
    // content: Content,
}

impl Renderable for MarkdownPage {
    fn render(&self, templates: &HashMap<String, Template>) -> String {
        let mut rendered = String::new();
        // for el in &self.content.elements {
        //     rendered.push_str(&el.render(&templates));
        // }
        rendered
    }
}

#[derive(PartialEq, Debug)]
struct Metadata {
    /// each line is treated as a key, value pair in the form of key: value
    info: HashMap<String, String>,
}

struct Content {
    elements: Vec<Element>,
}

enum Element {
    Text(String),
}

impl Renderable for Element {
    fn render(&self, templates: &HashMap<String, Template>) -> String {
        use Element::*;
        match self {
            Text(text) => text.to_string(),
        }
    }
}

pub fn load_markdown_pages(pages_path: &Path) -> Result<HashMap<String, MarkdownPage>, std::io::Error> {
    let markdown_pages_content = load_files_in_dir_to_string(pages_path)?;
    let mut pages = HashMap::new();
    for (path, content) in markdown_pages_content {
        let page = parse_markdown_page(&content);

        pages.insert(path.to_str().unwrap().to_string(), page);
    }
    Ok(pages)
}

fn parse_markdown_page(content: &str) -> MarkdownPage {
    let metadata = parse_metadata(content);
    // let content = parse_content(content);
    MarkdownPage { metadata }
}

fn parse_metadata(content: &str) -> Metadata {
    let mut info = HashMap::new();
    for line in content.lines() {
        if line == ":content:" {
            break;
        }
        // allow breaks between pairs to allow grouping of metadata
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once(": ") else {
            panic!("Incorrect key: value pair in line: {line}");
        };
        info.insert(key.to_string(), value.to_string());
    }
    assert!(
        info.contains_key("template"),
        "Every page must specify template in metadata"
    );
    Metadata { info }
}

#[cfg(test)]
mod tests {
    use super::{parse_metadata, Metadata};
    use std::collections::HashMap;

    #[test]
    fn test_parse_metadata() {
        let content = "title: ayaya
template: template.html
author: blanktiger

key: value
:content:";
        let got = parse_metadata(content);
        let expected = Metadata {
            info: HashMap::from([
                ("title".to_string(), "ayaya".to_string()),
                ("template".to_string(), "template.html".to_string()),
                ("author".to_string(), "blanktiger".to_string()),
                ("key".to_string(), "value".to_string()),
            ]),
        };
        assert_eq!(expected, got);
    }

    #[test]
    #[should_panic]
    fn test_template_not_in_metadata() {
        let content = "title: ayaya
:content:";
        parse_metadata(content);
    }
}
