use crate::{
    components::blog::{BlogPost, POST_HELLO},
    AUTHOR, URL,
};
use rss::{Category, Channel, ChannelBuilder, Item, ItemBuilder, Source};
use std::fs::OpenOptions;

pub const RSS_ITEMS: &[(BlogPost, &str)] = &[(POST_HELLO, "Sun, 13 Aug 2023 18:10:00 GMT")];

fn create_rss_item(post: &BlogPost, date: &str) -> Item {
    let mut item = ItemBuilder::default();

    item.title(Some(post.title.to_string()));
    item.link(Some(format!( "{URL}/blog/{0}", post.link ).to_string()));
    item.author(Some(AUTHOR.to_string()));
    item.pub_date(Some(date.to_string()));
    item.description(Some(post.description.to_string()));
    item.content(Some(post.content.to_string()));
    item.categories(vec![ Category { name: post.category.to_string(), domain: None } ]);
    item.source(Some(Source {
        url: URL.to_string(),
        title: Some("Einherjar".to_string()),
    }));
    item.build()
}

fn create_rss(rss_items: &[(BlogPost, &str)]) -> Channel {
    let mut channel = ChannelBuilder::default();
    channel.title("Einherjar".to_string());
    channel.link(URL.to_string());
    channel.description("Einherjar's Blog".to_string());
    channel.items(
        rss_items
            .iter()
            .map(|(post, date)| create_rss_item(post, date))
            .collect::<Vec<Item>>(),
    );
    channel.language(Some("english".to_string()));
    channel.build()
}

pub fn write_rss_file() {
    let rss = create_rss(RSS_ITEMS);
    let rss_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("./public/feed.xml")
        .unwrap();
    rss.write_to(&rss_file).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rss::validation::Validate;
    #[test]
    fn test_rss() {
        let rss = create_rss(RSS_ITEMS);
        rss.validate().unwrap();
    }
}
