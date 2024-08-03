mod markdown;
mod template;
mod utils;

pub use markdown::{load_markdown_pages, MarkdownPage};
pub use template::{load_templates, Renderable, Template};
