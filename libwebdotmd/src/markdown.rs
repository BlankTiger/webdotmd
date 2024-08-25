use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use webdotx::utils::load_files_in_dir_to_string;
use webdotx::{FuncMap, Renderable, Template};

#[derive(Debug)]
pub struct MarkdownPage {
    /// starts from first line in the file and ends on line :content:
    metadata: Metadata,
    content: Content,
}

impl MarkdownPage {
    pub fn get_metadata(&self, name: &str) -> Option<&String> {
        self.metadata.info.get(name)
    }
}

fn header_to_text(header: &str) -> String {
    // dbg!(header);
    // let (_, header) = header.split_once("</a>").unwrap();
    // println!("{header}");
    header
        .replace(">", "")
        .replace("<", "")
        .replace("\n", "")
        .replace("\"", "'")
}

impl Renderable for MarkdownPage {
    fn render(&self, templates: &HashMap<String, Template>, autofill_funcs: &Option<FuncMap>) -> String {
        let mut content = String::new();
        let mut outline = String::from(r#"<ul class="list-disc list-inside">"#);
        for el in &self.content.elements {
            let rendered = &el.render(templates, autofill_funcs);
            content.push_str(rendered);
            if let Element::Header { level, elements } = el {
                let content = elements
                    .iter()
                    .map(|el| el.render(templates, autofill_funcs))
                    .collect::<String>();
                let text = &header_to_text(&content);
                outline.push_str("<li>");
                outline.push_str("<a href=\"#");
                outline.push_str(text);
                outline.push_str("\">");
                outline.push_str(text);
                outline.push_str("</a></li><br>\n");
            }
            println!("{outline}");
        }
        outline.push_str("</ul>");
        let mut filled_placeholders = self.metadata.info.clone();
        filled_placeholders.insert("content".to_string(), content);
        filled_placeholders.insert("outline".to_string(), outline);
        templates
            .get(&self.metadata.info["template"])
            .expect("Template to be found")
            .fill_template(filled_placeholders, autofill_funcs)
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
    Header {
        level: usize,
        elements: Vec<Element>,
    },
    Link {
        text: String,
        link: String,
    },
    List {
        list_type: ListType,
        items: Vec<Vec<Element>>,
    },
    Code {
        lang: String,
        code: String,
    },
}

#[derive(PartialEq, Debug)]
enum ListType {
    Ordered { symbol: String },
    Unordered { symbol: String },
}

impl Renderable for Element {
    fn render(&self, templates: &HashMap<String, Template>, autofill_funcs: &Option<FuncMap>) -> String {
        use Element::*;
        match self {
            Text(text) => format!("{text}"),
            Break => r#"<div class="py-2"></div>"#.to_string(),
            Header { level, elements } => {
                let content = elements
                    .iter()
                    .map(|el| el.render(templates, autofill_funcs))
                    .collect::<String>();
                let text = header_to_text(&content);
                let rendered = templates
                    .get("templates/elements/header.html")
                    .expect("Header template not found")
                    .fill_template(
                        HashMap::from([
                            ("level".to_string(), level.to_string()),
                            ("content".to_string(), content.to_string()),
                            ("text".to_string(), content),
                        ]),
                        autofill_funcs,
                    );
                rendered
            }
            Link { text, link } => templates
                .get("templates/elements/link.html")
                .expect("Link template not found")
                .fill_template(
                    HashMap::from([
                        ("text".to_string(), text.to_string()),
                        ("link".to_string(), link.to_string()),
                    ]),
                    autofill_funcs,
                ),
            List { list_type, items } => {
                let mut rendered = String::new();
                for item in items {
                    let mut item_content = String::new();
                    for el in item {
                        item_content.push_str(&el.render(templates, autofill_funcs).replace("\n", ""));
                    }
                    let item_template = templates.get("templates/elements/list_item.html").unwrap();
                    let item_rendered = item_template
                        .fill_template(HashMap::from([("item".to_string(), item_content)]), autofill_funcs);
                    rendered.push_str(&item_rendered);
                }
                match list_type {
                    ListType::Ordered { symbol } => {
                        let list_template = templates.get("templates/elements/ordered_list.html").unwrap();
                        let list_type = html_list_type_from(symbol);
                        list_template.fill_template(
                            HashMap::from([("items".to_string(), rendered), ("list_type".to_string(), list_type)]),
                            autofill_funcs,
                        )
                    }
                    ListType::Unordered { symbol } => {
                        let list_template = templates.get("templates/elements/unordered_list.html").unwrap();
                        let list_type = html_list_type_from(symbol);
                        list_template.fill_template(
                            HashMap::from([("items".to_string(), rendered), ("list_type".to_string(), list_type)]),
                            autofill_funcs,
                        )
                    }
                }
            }
            Code { lang, code } => {
                let code_template = templates.get("templates/elements/code.html").unwrap();
                code_template.fill_template(
                    HashMap::from([
                        ("lang".to_string(), lang.to_string()),
                        ("code".to_string(), code.to_string()),
                    ]),
                    autofill_funcs,
                )
            }
        }
    }
}

fn html_list_type_from(symbol: &str) -> String {
    match symbol {
        "-" => "list-disc".to_string(),
        "+" => "list-[circle]".to_string(),
        "1." => "list-decimal".to_string(),
        "a)" => "list-[lower-roman]".to_string(),
        _ => panic!("Invalid list type"),
    }
}

pub fn load_markdown_pages(pages_path: &Path) -> Result<HashMap<String, MarkdownPage>, std::io::Error> {
    let markdown_pages_content = load_files_in_dir_to_string(pages_path, Some("md"))?;
    let mut pages = HashMap::new();
    for (path, content) in &markdown_pages_content {
        let metadata = parse_metadata(content);
        let content = parse_content(content);
        let page = MarkdownPage { metadata, content };
        pages.insert(path.to_str().unwrap().to_string(), page);
    }
    Ok(pages)
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
    let content = content.replace(" -- ", " — ");
    let mut elements = vec![];
    let blocks = content.split("\n\n");
    let mut _blocks = Vec::new();
    let mut _block = String::new();
    let mut in_code_block = false;
    for block in blocks {
        if in_code_block && block.ends_with("```") {
            _block.push_str("\n\n");
            _block.push_str(block);
            _blocks.push(_block);
            _block = String::new();
            in_code_block = false;
            continue;
        } else if in_code_block {
            _block.push_str("\n\n");
            _block.push_str(block);
            continue;
        }
        if block.starts_with("```") && block.ends_with("```") {
        } else if block.starts_with("```") {
            in_code_block = true;
            _block.push_str(block);
            continue;
        }

        _blocks.push(block.to_string());
    }
    for block in _blocks {
        let block_elements = parse_block(&block);
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

    // NOTE: ORDER IS IMPORTANT, matching links first breaks matching list items that have links
    if block.starts_with('#') {
        let (level, text) = block.split_once(' ').unwrap();
        let header = Header {
            level: level.len(),
            elements: parse_block(text),
        };
        elements.push(header);
    } else if is_code(block) {
        // BUG: make it so that a code block is parsed entirely as a single element, otherwise
        // there cannot be a line break in a code block
        let block = block.trim();
        let (_, lang) = block.lines().next().unwrap().split_once("```").unwrap();
        let code = block
            .lines()
            .skip(1)
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .skip(1)
            .rev()
            .map(|s| {
                let mut s = s.to_string();
                s.push('\n');
                s
            })
            .collect::<String>();
        let lang = lang.to_string();
        let code = code.trim().to_string();
        let code = Code { lang, code };
        elements.push(code);
    } else if is_a_list(block) {
        let mut items = vec![];
        let mut nested_list_lines = vec![];
        for line in block.trim().lines() {
            // parse nested list
            if is_indented(line) && is_a_list(line) {
                let mut line = line.to_string();
                line.push('\n');
                nested_list_lines.push(line);
                continue;
            }
            if !nested_list_lines.is_empty() {
                let nested_list_block = nested_list_lines.iter().map(|s| s.trim_start()).collect::<String>();
                items.push(parse_block(&nested_list_block));
                nested_list_lines = vec![];
            }
            let (_list_symbol, item) = line.split_once(' ').unwrap();
            items.push(parse_block(item.trim()));
        }
        let list_type = parse_list_type(block);
        let list = List { list_type, items };
        elements.push(list);
    } else if LINK_PATTERN.is_match(block) {
        let link_capture = LINK_PATTERN.captures(block).unwrap();
        let link_match = link_capture.get(0).unwrap();
        let (link_start_idx, link_end_idx) = (link_match.range().start, link_match.range().end);
        if link_start_idx > 0 {
            elements.push(Text(block[..link_start_idx].to_string()));
        }
        let link = Link {
            text: link_capture.get(1).unwrap().as_str().to_string(),
            link: link_capture.get(2).unwrap().as_str().to_string(),
        };
        elements.push(link);
        if link_end_idx != block.len() {
            let rest_of_block = &block[link_end_idx..];
            elements.extend(parse_block(rest_of_block));
        }
    } else {
        elements.push(Text(block.to_string()));
    }
    elements
}

fn is_code(block: &str) -> bool {
    let block = block.trim();
    block.starts_with("```") && block.ends_with("```")
}

const LIST_TYPES: &[&str] = &["-", "+", "1.", "a)"];
const UNORDERED_LIST_TYPES: &[&str] = &["-", "+"];
const ORDERED_LIST_TYPES: &[&str] = &["1.", "a)"];

fn is_a_list(block: &str) -> bool {
    let lines = block.lines();
    for line in lines {
        let Some((line_start, _)) = line.trim().split_once(' ') else {
            return false;
        };
        if !LIST_TYPES.contains(&line_start) {
            return false;
        }
    }
    true
}

fn is_indented(s: &str) -> bool {
    s.starts_with("    ")
}

fn parse_list_type(s: &str) -> ListType {
    let s = s.trim();
    for u_type in UNORDERED_LIST_TYPES {
        if s.starts_with(u_type) {
            return ListType::Unordered {
                symbol: u_type.to_string(),
            };
        }
    }
    for o_type in ORDERED_LIST_TYPES {
        if s.starts_with(o_type) {
            return ListType::Ordered {
                symbol: o_type.to_string(),
            };
        }
    }
    eprintln!("Must always return a valid ListType, didn't for: {}", s);
    panic!();
}

pub fn write_html_pages(
    html_pages: &HashMap<String, String>,
    source_path: &Path,
    output_path: &Path,
) -> Result<(), std::io::Error> {
    let mut source_path = source_path.to_str().unwrap().to_string();
    source_path.push('/');
    for (path, html) in html_pages {
        let path = path.trim_start_matches(&source_path);
        let mut path = output_path.join(path);
        path.set_extension("html");
        std::fs::write(path, html)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{is_a_list, parse_content, parse_list_type, parse_metadata, Content, Element, ListType, Metadata};
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

    #[test]
    fn test_is_a_list() {
        let block = "Some text.";
        let block_is_a_list = is_a_list(block);
        assert!(!block_is_a_list);

        let block = "- Item 1
- Item 2";
        let block_is_a_list = is_a_list(block);
        assert!(block_is_a_list);

        let block = "+ Item 1
+ Item 2";
        let block_is_a_list = is_a_list(block);
        assert!(block_is_a_list);

        let block = "1. Item 1
1. Item 2";
        let block_is_a_list = is_a_list(block);
        assert!(block_is_a_list);

        let block = "a) Item 1
a) Item 2";
        let block_is_a_list = is_a_list(block);
        assert!(block_is_a_list);
    }

    #[test]
    fn test_parse_list_type() {
        let line = "- Item 1";
        let got = parse_list_type(line);
        let expected = ListType::Unordered {
            symbol: "-".to_string(),
        };
        assert_eq!(expected, got);

        let line = "+ Item 1";
        let got = parse_list_type(line);
        let expected = ListType::Unordered {
            symbol: "+".to_string(),
        };
        assert_eq!(expected, got);

        let line = "1. Item 1";
        let got = parse_list_type(line);
        let expected = ListType::Ordered {
            symbol: "1.".to_string(),
        };
        assert_eq!(expected, got);

        let line = "a) Item 1";
        let got = parse_list_type(line);
        let expected = ListType::Ordered {
            symbol: "a)".to_string(),
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_list() {
        let content = ":content:
- item 1
- item 2

+ item 1
+ item 2

1. item 1
1. item 2

a) item 1
a) item 2

a) item with a link [text](link.com), hurray!
a) item 2

- [text](link.com)";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![
                Element::List {
                    list_type: ListType::Unordered {
                        symbol: "-".to_string(),
                    },
                    items: vec![
                        vec![Element::Text("item 1".to_string())],
                        vec![Element::Text("item 2".to_string())],
                    ],
                },
                Element::Break,
                Element::List {
                    list_type: ListType::Unordered {
                        symbol: "+".to_string(),
                    },
                    items: vec![
                        vec![Element::Text("item 1".to_string())],
                        vec![Element::Text("item 2".to_string())],
                    ],
                },
                Element::Break,
                Element::List {
                    list_type: ListType::Ordered {
                        symbol: "1.".to_string(),
                    },
                    items: vec![
                        vec![Element::Text("item 1".to_string())],
                        vec![Element::Text("item 2".to_string())],
                    ],
                },
                Element::Break,
                Element::List {
                    list_type: ListType::Ordered {
                        symbol: "a)".to_string(),
                    },
                    items: vec![
                        vec![Element::Text("item 1".to_string())],
                        vec![Element::Text("item 2".to_string())],
                    ],
                },
                Element::Break,
                Element::List {
                    list_type: ListType::Ordered {
                        symbol: "a)".to_string(),
                    },
                    items: vec![
                        vec![
                            Element::Text("item with a link ".to_string()),
                            Element::Link {
                                text: "text".to_string(),
                                link: "link.com".to_string(),
                            },
                            Element::Text(", hurray!".to_string()),
                        ],
                        vec![Element::Text("item 2".to_string())],
                    ],
                },
                Element::Break,
                Element::List {
                    list_type: ListType::Unordered {
                        symbol: "-".to_string(),
                    },
                    items: vec![vec![Element::Link {
                        text: "text".to_string(),
                        link: "link.com".to_string(),
                    }]],
                },
            ],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_nested_list() {
        let content = ":content:
- item 1
- item 2:
    a) item with a link [text](link.com), hurray!
    a) item 2
- [text](link.com)";
        // TODO: work on infinitely nested lists
        let got = parse_content(content);
        let expected = Content {
            elements: vec![Element::List {
                list_type: ListType::Unordered {
                    symbol: "-".to_string(),
                },
                items: vec![
                    vec![Element::Text("item 1".to_string())],
                    vec![Element::Text("item 2:".to_string())],
                    vec![Element::List {
                        list_type: ListType::Ordered {
                            symbol: "a)".to_string(),
                        },
                        items: vec![
                            vec![
                                Element::Text("item with a link ".to_string()),
                                Element::Link {
                                    text: "text".to_string(),
                                    link: "link.com".to_string(),
                                },
                                Element::Text(", hurray!".to_string()),
                            ],
                            vec![Element::Text("item 2".to_string())],
                        ],
                    }],
                    vec![Element::Link {
                        text: "text".to_string(),
                        link: "link.com".to_string(),
                    }],
                ],
            }],
        };
        assert_eq!(expected, got);
    }

    #[test]
    fn test_parse_content_code() {
        let content = ":content:
```rust
fn hello_world() -> ! {
    while true {}
}
```";
        let got = parse_content(content);
        let expected = Content {
            elements: vec![Element::Code {
                lang: "rust".to_string(),
                code: "fn hello_world() -> ! {
    while true {}
}"
                .to_string(),
            }],
        };
        assert_eq!(expected, got);
    }
}
