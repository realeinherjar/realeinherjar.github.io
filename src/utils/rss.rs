use pulldown_cmark_frontmatter::FrontmatterExtractor;
use rss::{Item, ItemBuilder};

use serde::{Deserialize, Serialize};

use crate::{AUTHOR, URL};

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub date: String,
}

pub fn extract_markdown_metadata(input: &str) -> Item {
    let extractor = FrontmatterExtractor::from_markdown(input);
    let frontmatter = extractor.frontmatter.expect("frontmatter not detected");
    let code_block = frontmatter.code_block.expect("code block not detected");
    let title = frontmatter.title.unwrap();
    let attrs: Attributes = toml::from_str(&code_block.source).expect("invalid toml");
    let date = attrs.date;
    let mut item = ItemBuilder::default();
    item.title(Some(title.clone()));
    item.link(Some(format!("{URL}/blog/{title}")));
    item.author(Some(AUTHOR.to_string()));
    item.pub_date(Some(date));
    item.build()
}
