use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use markup5ever_rcdom::{RcDom, Handle};
use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let url = "http://example.com";
    match fetch_html(url).await {
        Ok(html) => {
            let dom = parse_html(&html);
            println!("HTML parsed successfully.");
            print_dom(dom.document, 0);
        },
        Err(err) => eprintln!("Error fetching HTML: {}", err),
    }
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

fn parse_html(html: &str) -> RcDom {
    let parser = parse_document(RcDom::default(), Default::default());
    parser.one(html)
}

fn print_dom(handle: Handle, depth: usize) {
    let node = handle;
    for _ in 0..depth {
        print!("  ");
    }
    match &node.data {
        markup5ever_rcdom::NodeData::Document => println!("Document"),
        markup5ever_rcdom::NodeData::Doctype { name, .. } => println!("Doctype: {}", name),
        markup5ever_rcdom::NodeData::Text { contents } => println!("Text: {:?}", contents.borrow()),
        markup5ever_rcdom::NodeData::Element { ref name, .. } => println!("Element: <{}>", name.local),
        _ => println!("Other"),
    }
    for child in node.children.borrow().iter() {
        print_dom(child.clone(), depth + 1);
    }
}
