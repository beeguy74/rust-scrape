use std::future::Future;
use std::io::copy;
use std::env;

use reqwest;
use regex::Regex;

struct Args {
    url: String,
    dir_path: String,
    deep: i32,
}

impl Args {
    fn default() -> Self {
        Args {
            url: "https://www.osnews.com".to_string(),
            dir_path: "./data".to_string(),
            deep: 5,
        }
    }
}

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

fn parse_args() -> Args {
    let mut args = Args::default();
    let mut i = 1;
    while i < env::args().len() {
        match env::args().nth(i).unwrap().as_str() {
            "-r" => {
                args.url = env::args().nth(i + 1).unwrap();
                i += 2;
            }
            "-p" => {
                args.dir_path = env::args().nth(i + 1).unwrap();
                i += 2;
            }
            "-l" => {
                args.deep = env::args().nth(i + 1).unwrap().parse::<i32>().unwrap();
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }
    args
}

fn url_links_finder(html: &str) -> Vec<String> {
    // i need a regex to find all <a href> tags and extract the links
    let re = Regex::new(r#"<a[^>]*?href="([^"]*?)"[^>]*?>"#).unwrap();
    let mut res = Vec::new();
    for cap in re.captures_iter(html) {
        res.push(cap[1].to_string());
    }
    res
}

async fn download_print(files: Vec<String>, dir_path: &str) {
    for file in files {
        println!("{}", file);
        download_to_file(&file, dir_path).await;
    }
}

async fn rec_download(url: &str, dir_path: &str, iteration: i32) -> Box<dyn Future<Output = ()>> {
    if iteration == 0 {
        return Box::new(async {});
    }
    let full_resp = reqwest::get(url).await;
    let resp = full_resp.unwrap().text().await.unwrap();

    let links = url_links_finder(&resp);
    download_print(url_extension_searcher(&resp, "jpg"), dir_path).await;
    download_print(url_extension_searcher(&resp, "png"), dir_path).await;
    download_print(url_extension_searcher(&resp, "jpeg"), dir_path).await;
    download_print(url_extension_searcher(&resp, "gif"), dir_path).await;
    download_print(url_extension_searcher(&resp, "bmp"), dir_path).await;
    for link in links {
        println!("Downloading {}", link);
        rec_download(&link, dir_path, iteration - 1).await;
    }
    return Box::new(async {});
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = parse_args();
    println!("url: {}, dir_path: {}, deep: {}", args.url, args.dir_path, args.deep);

    rec_download(&args.url, &args.dir_path, args.deep).await;




    Ok(())
}