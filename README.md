# Personal Website using Dioxus

I am using the [Dioxus](https://dioxuslabs.com) which
is a full stack library in Rust.

The RSS feed is generated using the [`rss` crate](https://github.com/rust-syndication/rss).

For the Rustacean out there,
this personal site and blog is ready for you to fork and use.
You can add API endpoints with [`axum`](https://github.com/tokio-rs/axum) and [`sqlx`](https://github.com/jmoiron/sqlx).
The license for the code is MIT, and for the content is CC-BY-NC-SA.

## Development

1. Install Dioxus: `cargo install dioxus-cli`
1. Install `npm`: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
1. Install `tailwindcss`: <https://tailwindcss.com/docs/installation>
1. Run the following command in the root of the project to start
   the tailwind CSS compiler:

   ```bash
   npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
   ```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

Open the browser to http://localhost:8080

## Converting Markdown to HTML

Once you have a blog post in Markdown,
you can convert to HTML
using [Pandoc](https://pandoc.org/):

```bash
pandoc sample_readme.md -t html -o sample_readme.html
```

## Generating the RSS feed

To generate the RSS feed for the blog,
you'll need to run with `cargo run`.
This will crate a file at `public/feed.xml`.
