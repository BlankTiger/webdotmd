use crate::utils::load_files_in_dir_to_string;
use std::{collections::HashMap, path::Path};

/// Fundamental data structure of webdotx, holds template content and parsed placeholders.
/// Placeholder always starts with `{{ $`, and ends with `$ }}`, name of the placeholder is between
/// `$`. Example template:
///
/// ```html
/// <html>
///
/// <head>
///     <title>{{ $title$ }}</title>
/// </head>
///
/// <body>
///     <h1>{{ $header$ }}</h1>
///     {{ $content$ }}
/// </body>
///
/// </html>
/// ```
///
/// For that template, `Template` struct internally would look like this:
/// ```rust ignore
/// let template = Template {
///     content: "<html>\n\n<head>\n    <title>{{ $title$ }}</title>\n</head>\n\n<body>\n    <h1>{{ $header$ }}</h1>\n    {{ $content$ }}\n</body>\n\n</html>\n".to_string(),
///     placeholders: [
///         Placeholder {
///             name: "title",
///             position: Position {
///                 from: 26,
///                 to: 38,
///             },
///         },
///         Placeholder {
///             name: "header",
///             position: Position {
///                 from: 33,
///                 to: 46,
///             },
///         },
///         Placeholder {
///             name: "content",
///             position: Position {
///                 from: 10,
///                 to: 24,
///             },
///         },
///     ],
/// };
/// ```
///
/// Notice that `position.from` is the index of the first character of the placeholder counting from
/// the beginning of the template content string if it's the first placeholder, or from the end of
/// the previous placeholder if it's not the first one.
#[derive(Debug)]
pub struct Template {
    content: String,
    placeholders: Vec<Placeholder>,
}

#[derive(Debug, PartialEq)]
struct Placeholder {
    /// Name is between '$'.
    name: String,
    /// Position is relative, first one is always from the beginning of template content, second
    /// one is from the end of the first placeholder, third one is from the end of the second
    /// placeholder. This sidesteps calculations of offsets during template rendering, because we
    /// don't need to change position when a placeholder is filled with a str of different length
    /// than the placeholder itself.
    position: Position,
}

#[derive(Debug, PartialEq)]
struct Position {
    from: usize,
    to: usize,
}

/// Loads templates from the provided directory with provided extension. If extension is `None`, then
/// all files in the directory are loaded.
///
/// Returns a map of template names (paths) to `Template` structs.
///
/// Example use:
/// ```rust
/// use std::collections::HashMap;
/// use webdotx::{load_templates, Template};
///
/// let templates_path = std::path::Path::new("templates");
/// let templates = load_templates(templates_path, Some("html"))/* .unwrap() */;
/// // let template = templates.get("template.html").unwrap();
/// ```
pub fn load_templates(
    templates_path: &Path,
    extension: Option<&str>,
) -> Result<HashMap<String, Template>, std::io::Error> {
    let template_strings = load_files_in_dir_to_string(templates_path, extension)?;
    let mut templates = HashMap::new();
    for (template_path, template_content) in template_strings {
        let mut template = Template {
            content: template_content.clone(),
            placeholders: Vec::new(),
        };
        let placeholders = parse_placeholders(&template_content);
        template.placeholders.extend(placeholders);
        templates.insert(template_path.to_str().unwrap().to_string(), template);
    }
    Ok(templates)
}

fn parse_placeholders(template_content: &str) -> Vec<Placeholder> {
    let mut placeholders = vec![];
    let mut content = template_content;
    while !content.is_empty() {
        let placeholder_start = content.find("{{ $");
        let placeholder_end = content.find("$ }}");
        let (Some(placeholder_start), Some(placeholder_end)) = (placeholder_start, placeholder_end) else {
            return placeholders;
        };

        let name = content[placeholder_start + 4..placeholder_end].to_string();
        let position = Position {
            from: placeholder_start,
            to: placeholder_end + 3,
        };
        content = &content[position.to + 1..];
        let placeholder = Placeholder { name, position };
        placeholders.push(placeholder);
    }

    placeholders
}

impl Template {
    /// Fills placeholders in the template with the provided values. If value for a placeholder is
    /// missing, then it returns an error.
    pub fn fill_template(&self, filled_placeholders: HashMap<String, String>) -> String {
        let mut rendered = String::new();
        let mut content = self.content.as_str();
        for placeholder in &self.placeholders {
            rendered.push_str(&content[..placeholder.position.from]);
            let filled_placeholder = filled_placeholders.get(&placeholder.name).unwrap().as_str();
            rendered.push_str(filled_placeholder);
            content = &content[placeholder.position.to + 1..];
        }
        rendered.push_str(content);
        rendered
    }
}

/// Renderable allows for rendering of data structures into a string using provided templates.
///
/// Example use:
///
/// ```rust
/// use std::collections::HashMap;
/// use webdotx::{Renderable, Template};
///
/// struct Page {
///    name: String,
///    content: String,
///    test_offset: String,
///    template_name: String,
///    // other fields
/// }
///
/// impl Renderable for Page {
///    fn render(&self, templates: &HashMap<String, Template>) -> String {
///        let filled_placeholders = HashMap::from([
///            ("name".to_string(), self.name.clone()),
///            ("content".to_string(), self.content.clone()),
///            ("test_offset".to_string(), self.test_offset.clone()),
///        ]);
///        let template = templates.get(&self.template_name).unwrap();
///        template.fill_template(filled_placeholders)
///    }
/// }
/// ```
pub trait Renderable {
    fn render(&self, templates: &HashMap<String, Template>) -> String;
}

/// Renders all renderables using provided templates. Returns a map of renderable names to rendered
/// strings.
///
/// Example use:
/// ```rust ignore
/// use std::collections::HashMap;
/// use webdotx::{render, Renderable, Template};
///
/// struct Page {
///   name: String,
///   content: String,
///   test_offset: String,
///   template_name: String,
///   // other fields
/// }
///
/// impl Renderable for Page {
///     fn render(&self, templates: &HashMap<String, Template>) -> String {
///         let filled_placeholders = HashMap::from([
///             ("name".to_string(), self.name.clone()),
///             ("content".to_string(), self.content.clone()),
///             ("test_offset".to_string(), self.test_offset.clone()),
///             // other fields
///         ]);
///         let template = templates.get(&self.template_name).unwrap();
///         template.fill_template(filled_placeholders)
///     }
/// }
///
/// let named_renderables = HashMap::from([
///     (
///         "page1".to_string(),
///         Page {
///             name: "page1".to_string(),
///             content: "some content".to_string(),
///             test_offset: "tested".to_string(),
///             template_name: "template1".to_string(),
///         },
///     ),
///     (
///         "page2".to_string(),
///         Page {
///             name: "page2".to_string(),
///             content: "some other content".to_string(),
///             test_offset: "tested".to_string(),
///             template_name: "template2".to_string(),
///         },
///     ),
/// ]);
///
/// let templates = load_templates(std::path::Path::new("templates")).unwrap();
/// let rendered = render(&named_renderables, &templates);
/// ```
pub fn render(
    named_renderables: &HashMap<String, impl Renderable>,
    templates: &HashMap<String, Template>,
) -> HashMap<String, String> {
    let mut rendered = HashMap::new();
    // TODO: probably could parallelize
    for (page_name, page) in named_renderables {
        let rendered_page = page.render(templates);
        rendered.insert(page_name.to_string(), rendered_page);
    }
    rendered
}

#[cfg(test)]
mod tests {
    use super::{parse_placeholders, Placeholder, Position, Template};
    use std::collections::HashMap;

    #[test]
    fn test_parse_placeholders() {
        let content = "some text, {{ $name$ }}, other text {{ $content$ }}{{ $test_offset$ }}";
        let got = parse_placeholders(content);
        let expected = vec![
            Placeholder {
                name: "name".into(),
                position: Position { from: 11, to: 22 },
            },
            Placeholder {
                name: "content".into(),
                position: Position { from: 13, to: 27 },
            },
            Placeholder {
                name: "test_offset".into(),
                position: Position { from: 0, to: 18 },
            },
        ];
        assert_eq!(expected, got);
    }

    #[test]
    fn test_render() {
        let filled_placeholders = HashMap::from_iter([
            ("name".to_string(), "blanktiger".to_string()),
            ("content".to_string(), "some interesting text".to_string()),
            ("test_offset".to_string(), "tested".to_string()),
        ]);
        let template = Template {
            content: "some text, {{ $name$ }}, other text {{ $content$ }}{{ $test_offset$ }}".to_string(),
            placeholders: vec![
                Placeholder {
                    name: "name".into(),
                    position: Position { from: 11, to: 22 },
                },
                Placeholder {
                    name: "content".into(),
                    position: Position { from: 13, to: 27 },
                },
                Placeholder {
                    name: "test_offset".into(),
                    position: Position { from: 0, to: 18 },
                },
            ],
        };
        let got = template.fill_template(filled_placeholders);
        let expected = "some text, blanktiger, other text some interesting texttested".to_string();
        assert_eq!(expected, got);
    }
}
