use reqwest;
use tokio;

#[tokio::main]
async fn main() {
    let url = "http://example.com";
    match fetch_html(url).await {
        Ok(html) => println!("{}", html),
        Err(err) => eprintln!("Error fetching HTML: {}", err),
    }
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
