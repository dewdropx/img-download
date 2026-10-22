use error_chain::error_chain; // handle errors
use std::io::Write; // standard io write
use std::fs::File; // file system

// error handling
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// set up tokio run time for the asynchronous main function
#[tokio::main]
async fn main() -> Result <()> {

    // send get request for image
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target)
        .await?
        .error_for_status()?;
    
    // create destination file in the current working directory
    let mut dest = File::create("rust-logo-512x512.png")?;

    // write the contents from the response to the destination
    let content = response.bytes().await?;
    dest.write_all(&content)?;

    println!("image downloaded in the current working directory successfully");

    Ok(())
}