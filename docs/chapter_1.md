# Chapter 1: Project Start and Basic Setup

## 1. Set up Rust development environment
If you haven't already installed Rust, you can do so by running the following command:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You'll also need to install `pkg-config` and OpenSSL for some of the dependencies:
- On Ubuntu, you can install them with the following command:
  ```sh
  sudo apt-get update
  sudo apt-get install -y pkg-config libssl-dev
  ```
- On Fedora:
  ```sh
  sudo dnf install pkg-config openssl-devel
  ```
- On macOS, OpenSSL can be installed via Homebrew:
  ```sh
  brew install openssl pkg-config
  ```

## 2. Create a new Rust project using Cargo
Open a terminal and run:
```sh
cargo new browser
cd browser
```

## 3. Project Directory Structure
The directory structure of the project will look like this:
```
browser/
├── Cargo.toml
└── src/
    └── main.rs
```
The `main.rs` file is the entry point of our Rust application. It will contain our browser logic.

## 4. Making an HTTP Request to Fetch a Webpage
We'll start by making an HTTP request to fetch a webpage.

### Update `Cargo.toml` to Include Dependencies
We'll need `reqwest` for making HTTP requests and `tokio` for async support. Open `Cargo.toml` and add the following dependencies:
```toml
[dependencies]
reqwest = "0.11"
tokio = { version = "1", features = ["full"] }
```

### Code to Make an HTTP Request
Update `src/main.rs` with the following code:

```rust
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
```

## 5. Running the Program
To run the program, use the following command:
```sh
cargo run
```
This will compile the project, make an HTTP request to the specified URL, and print the HTML response.

## 6. Planning the Project Structure
We'll eventually split our browser into several modules:
- **Network Layer**: to handle HTTP requests
- **HTML Parser**: to parse the HTML into a DOM tree
- **CSS Parser**: to parse and apply styles
- **Layout and Rendering**: to display content visually

This modular approach will help in organizing the code as we move forward.