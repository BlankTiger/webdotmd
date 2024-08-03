use crate::utils::load_files_in_dir_to_string;
use crate::{Renderable, Template};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

pub struct MarkdownPage {
    /// starts from first line in the file and ends on line :content:
    metadata: Metadata,
    content: Content,
}

impl Renderable for MarkdownPage {
    fn render(&self, templates: &HashMap<String, Template>) -> String {
        let mut rendered = String::new();
        for el in &self.content.elements {
            rendered.push_str(&el.render(&templates));
        }
        rendered
    }
}

#[derive(PartialEq, Debug)]
struct Metadata {
    /// each line is treated as a key, value pair in the form of key: value
    info: HashMap<String, String>,
}

#[derive(PartialEq, Debug)]
struct Content {
    elements: Vec<Element>,
}

#[derive(PartialEq, Debug)]
enum Element {
    Text(String),
    Break,
    Header { level: usize, elements: Vec<Element> },
    Link { text: String, link: String },
}

impl Renderable for Element {
    fn render(&self, templates: &HashMap<String, Template>) -> String {
        use Element::*;
        match self {
            Text(text) => text.to_string(),
            Break => "<br />".to_string(),
            Header { level, elements } => todo!(),
            Link { text, link } => todo!(),
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
    let content = parse_content(content);
    MarkdownPage { metadata, content }
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

fn parse_content(content: &str) -> Content {
    let start_of_content = content.find(":content:\n").unwrap() + ":content:\n".len();
    let content = &content[start_of_content..];
    let mut elements = vec![];
    let blocks = content.split("\n\n");
    for block in blocks {
        let block_elements = parse_block(block);
        elements.extend(block_elements);
        elements.push(Element::Break);
    }
    elements.pop();
    Content { elements }
}

static LINK_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[(.*?)\]\((.*?)\)").expect("Failed to compile link pattern"));

fn parse_block(block: &str) -> Vec<Element> {
    let mut elements = vec![];
    use Element::*;
    if block.starts_with('#') {
        let (level, text) = block.split_once(' ').unwrap();
        let header = Header {
            level: level.len(),
            elements: parse_block(text),
        };
        elements.push(header);
    } else if LINK_PATTERN.is_match(block) {
        let link_capture = LINK_PATTERN.captures(block).unwrap();
        let link_match = link_capture.get(0).unwrap();
        let (link_start_idx, link_end_idx) = (link_match.range().start, link_match.range().end);
        if link_start_idx > 0 {
            elements.push(Text(block[..link_start_idx].to_string()));
        }
        elements.push(Link {
            text: link_capture.get(1).unwrap().as_str().to_string(),
            link: link_capture.get(2).unwrap().as_str().to_string(),
        });
        if link_end_idx != block.len() - 1 {
            let rest_of_block = &block[link_end_idx..];
            elements.extend(parse_block(rest_of_block));
        }
    } else {
        elements.push(Text(block.to_string()));
    }
    elements
}

#[cfg(test)]
mod tests {
    use super::{parse_content, parse_metadata, Content, Element, Metadata};
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

    #[test]
    fn test_parse_content_text() {
        let content = ":content:
Some random text.";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![Element::Text("Some random text.".to_string())],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_blocks_of_text() {
        let content = ":content:
Some random text.

Some other random text.";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![
                Element::Text("Some random text.".to_string()),
                Element::Break,
                Element::Text("Some other random text.".to_string()),
            ],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_header() {
        let content = ":content:
# Header

## Header

### Header

#### Header";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![
                Element::Header {
                    level: 1,
                    elements: vec![Element::Text("Header".to_string())],
                },
                Element::Break,
                Element::Header {
                    level: 2,
                    elements: vec![Element::Text("Header".to_string())],
                },
                Element::Break,
                Element::Header {
                    level: 3,
                    elements: vec![Element::Text("Header".to_string())],
                },
                Element::Break,
                Element::Header {
                    level: 4,
                    elements: vec![Element::Text("Header".to_string())],
                },
            ],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_text_with_link() {
        let content = ":content:
Some text with a link: [link text](coolpage.com). Cool.";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![
                Element::Text("Some text with a link: ".to_string()),
                Element::Link {
                    text: "link text".to_string(),
                    link: "coolpage.com".to_string(),
                },
                Element::Text(". Cool.".to_string()),
            ],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_header_with_link() {
        let content = ":content:
# Some text with a link: [link text](coolpage.com). Cool.";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![Element::Header {
                level: 1,
                elements: vec![
                    Element::Text("Some text with a link: ".to_string()),
                    Element::Link {
                        text: "link text".to_string(),
                        link: "coolpage.com".to_string(),
                    },
                    Element::Text(". Cool.".to_string()),
                ],
            }],
        };
        assert_eq!(expected, got);
    }
}
