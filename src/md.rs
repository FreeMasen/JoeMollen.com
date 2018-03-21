use pulldown_cmark::html::push_html;
use pulldown_cmark::Parser;

pub fn md_to_html(html: &str) -> String {
    let parser = Parser::new(html);
    let mut buf = String::new();
    push_html(&mut buf, parser);
    println!("html:\n{}\n", buf);
    buf
}