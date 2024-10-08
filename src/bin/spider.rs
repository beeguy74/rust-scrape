use std::{io::copy, mem};
use reqwest::{self, blocking::Client};
use regex::Regex;
mod modules;


fn url_extension_searcher(html: &String, extension: &str) -> Vec<String> {
    let re = Regex::new(&format!(r#"(https?://[^\s]*?\.{}[^\s]*?)"#, extension)).unwrap();
    let mut res = Vec::new();
    for cap in re.captures_iter(html) {
        res.push(cap[1].to_string());
    }
    res
}

 fn download_to_file(url: &str, dir_path: &String) {
    let filename = dir_path.to_string() + "/" + url.split("/").last().unwrap();
    let mut file = std::fs::File::create(filename).unwrap();
    let resp = reqwest::blocking::get(url);
    if resp.is_err() {
        println!("Failed to download {}", url);
        return;
    }
    else {
        copy(&mut resp.unwrap().bytes().unwrap().as_ref(), &mut file).unwrap();
    }
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

 fn download_print(files: Vec<String>, dir_path: &String) {
    for file in files {
        download_to_file(&file, &dir_path);
    }
}

fn rec_download(client: &Client ,url: &String, dir_path: &String, iteration: i32) {
    if iteration == 0 {
        return ;
    }
    let resp;
    {
        let full_resp = client.get(url).send();
        if full_resp.is_err() {
            println!("Failed to download {}", url);
            return;
        }
        resp = full_resp.unwrap().text().unwrap();
    }

    download_print(url_extension_searcher(&resp, "jpg"), dir_path);
    download_print(url_extension_searcher(&resp, "png"), dir_path);
    download_print(url_extension_searcher(&resp, "jpeg"), dir_path);
    download_print(url_extension_searcher(&resp, "gif"), dir_path);
    download_print(url_extension_searcher(&resp, "bmp"), dir_path);

    let links = url_links_finder(&resp);
    mem::drop(resp);
    for link in links {
        println!("{} Downloading {}", iteration, link);
        rec_download(client, &link, dir_path, iteration - 1);
    }
}


 fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = modules::args::Args::default();
    args.parse_args()?;
    let client = reqwest::blocking::Client::new();
    println!("url: {}, dir_path: {}, deep: {}", args.url, args.dir_path, args.deep);

    rec_download(&client, &args.url, &args.dir_path, args.deep);

    Ok(())
}