use dioxus::core::ScopeState;
use pulldown_cmark::html;
use pulldown_cmark_frontmatter::FrontmatterExtractor;

pub fn use_markdown(cx: &ScopeState) -> &dyn Fn(String) -> String {
    cx.use_hook(|| {
        move |content: String| {
            let str = &content;
            let mut extractor = FrontmatterExtractor::from_markdown(str);

            let mut output = String::new();
            html::push_html(&mut output, &mut extractor);

            output
        }
    })
}
