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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = parse_args();
    println!("url: {}, dir_path: {}, deep: {}", args.url, args.dir_path, args.deep);
    let resp = reqwest::get(args.url).await?.text().await?;

    let jpgs: Vec<String> = url_extension_searcher(&resp, "jpg");
    let pngs: Vec<String> = url_extension_searcher(&resp, "png");
    let jpegs: Vec<String> = url_extension_searcher(&resp, "jpeg");
    let gifs: Vec<String> = url_extension_searcher(&resp, "gif");
    let bmps: Vec<String> = url_extension_searcher(&resp, "bmp");



    for png in pngs {
        println!("{}", png);
        download_to_file(&png, &args.dir_path).await;
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