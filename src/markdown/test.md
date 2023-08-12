# Hello World"

```toml
date = "2023-08-12" # date --rfc-3339=seconds
```

This is the first blog entry in the blog.

I am building this blog using [Dioxus](https://dioxuslabs.com) in Rust.
This is hosted on [Github](https://github.com/realeinherjar/realeinherjar.github.io).
Feel free to check the code.

Under the hood this blog is using the [`pulldown-cmark` crate](https://github.com/raphlinus/pulldown-cmark)
to parse the markdown files into HTML.
Then, the RSS feed is generated using the [`rss` crate](https://github.com/rust-syndication/rss).
The markdown files have an embedded front matter that contains metadata
that will be used to generate the RSS entry.
These are parsed using the [`pulldown-cmark-frontmatter`](https://github.com/khonsulabs/pulldown-cmark-frontmatter) crate,
which uses a code block instead of other markers,
allowing for most Markdown editing software to handle syntax highlighting,
errors, and more.

Technically speaking, Rust and Dioxus are straightforward to use.
However, [`tailwindcss`](https://tailwindcss.com/) is a bit of a pain to use.
Documentation sucks and the API is not very intuitive.
I welcome any PRs to improve it.
Honestly, I prefer to hammer my fingers than to have to deal with CSS.

For the Rustacean out there,
this personal site and blog is ready for you to fork and use.
You can add API endpoints with [`axum`](https://github.com/tokio-rs/axum) and [`sqlx`](https://github.com/jmoiron/sqlx).
The license for the code is MIT, and for the content is CC-BY-NC-SA.
