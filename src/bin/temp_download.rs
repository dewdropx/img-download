use error_chain::error_chain; // handle errors
use std::io::Write; // standard io write
use std::fs::File; // file system
use tempfile::Builder; // creating temp directory

// error handling
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// set up tokio run time for asynchronous main function
#[tokio::main]
async fn main() -> Result<()> {

    // send get request for image
    let tmp_dir = Builder::new().prefix("example").tempdir()?; // create temp directory
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target)
        .await?
        .error_for_status()?;

    // determine file name and create destination file
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty(){None} else {Some(name)})
            .unwrap_or("tmp.bin");
        
        println!("file to download: '{}'", fname);

        let fname = tmp_dir.path().join(fname);
        println!("file will be located under: '{:?}'", fname);
        File::create(fname)?;
    };

    // write contents from the response to destination
    let content = response.bytes().await?;
    dest.write_all(&content)?;

    Ok(())
}