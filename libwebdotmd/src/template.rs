use crate::utils::load_files_in_dir_to_string;
use std::{collections::HashMap, path::Path};

#[derive(Debug)]
pub struct Template {
    content: String,
    placeholders: Vec<Placeholder>,
}

/// Placeholder always starts with '{{ $', and ends with '$ }}'.
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

pub fn load_templates(templates_path: &Path) -> Result<HashMap<String, Template>, std::io::Error> {
    let template_strings = load_files_in_dir_to_string(templates_path)?;
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

pub fn fill_template(template: Template, filled_placeholders: HashMap<String, String>) -> String {
    let mut rendered = String::new();
    let mut content = template.content.as_str();
    for placeholder in template.placeholders {
        rendered.push_str(&content[..placeholder.position.from]);
        let filled_placeholder = filled_placeholders.get(&placeholder.name).unwrap().as_str();
        rendered.push_str(filled_placeholder);
        content = &content[placeholder.position.to + 1..];
    }
    rendered.push_str(content);
    rendered
}

pub trait Renderable {
    fn render(&self, templates: &HashMap<String, Template>) -> String;
}

#[cfg(test)]
mod tests {
    use super::{fill_template, parse_placeholders, Placeholder, Position, Template};
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
        let got = fill_template(template, filled_placeholders);
        let expected = "some text, blanktiger, other text some interesting texttested".to_string();
        assert_eq!(expected, got);
    }
}
