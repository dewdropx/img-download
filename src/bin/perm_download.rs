use error_chain::error_chain;
use std::io::Write;
use std::fs::File;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result <()> {
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target)
        .await?
        .error_for_status()?;
    
    let mut dest = File::create("rust-logo-512x512.png")?;

    let content = response.bytes().await?;
    dest.write_all(&content)?;

    println!("image downloaded in the current working directory successfully");

    Ok(())
}