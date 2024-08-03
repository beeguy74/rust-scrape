use std::io::copy;

use reqwest;
use regex::Regex;

fn url_extension_searcher(html: &str, extension: &str) -> Vec<String> {
    let re = Regex::new(&format!(r#"(https?://[^\s]*?\.{}[^\s]*?)"#, extension)).unwrap();
    let mut res = Vec::new();
    for cap in re.captures_iter(html) {
        res.push(cap[1].to_string());
    }
    res
}

async fn download_to_file(url: &str, dir_path: &str) {
    let filename = dir_path.to_string() + "/" + url.split("/").last().unwrap();
    let mut file = std::fs::File::create(filename).unwrap();
    let mut resp = reqwest::get(url).await;
    if resp.is_err() {
        println!("Failed to download {}", url);
        return;
    }
    else {
        copy(&mut resp.unwrap().bytes().await.unwrap().as_ref(), &mut file).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.osnews.com";
    let resp = reqwest::get(url).await?.text().await?;
    let jpgs: Vec<String> = url_extension_searcher(&resp, "jpg");
    let pngs = url_extension_searcher(&resp, "png");
    let jpegs: Vec<String> = url_extension_searcher(&resp, "jpeg");
    let gifs: Vec<String> = url_extension_searcher(&resp, "gif");
    let bmps: Vec<String> = url_extension_searcher(&resp, "bmp");



    for png in pngs {
        println!("{}", png);
        download_to_file(&png, "./images").await;
    }
    for jpg in jpgs {
        println!("{}", jpg);
    }
    for jpeg in jpegs {
        println!("{}", jpeg);
    }
    for gif in gifs {
        println!("{}", gif);
    }
    for bmp in bmps {
        println!("{}", bmp);
    }

    Ok(())
}