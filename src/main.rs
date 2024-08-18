use std::io::copy;
use reqwest;
use regex::Regex;
mod modules;


fn url_extension_searcher(html: &str, extension: &str) -> Vec<String> {
    let re = Regex::new(&format!(r#"(https?://[^\s]*?\.{}[^\s]*?)"#, extension)).unwrap();
    let mut res = Vec::new();
    for cap in re.captures_iter(html) {
        res.push(cap[1].to_string());
    }
    res
}

 fn download_to_file(url: &str, dir_path: String) {
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

 fn download_print(files: Vec<String>, dir_path: String) {
    for file in files {
        println!("{}", file);
        download_to_file(&file, dir_path.clone());
    }
}

fn rec_download(url: String, dir_path: String, iteration: i32) {
    if iteration == 0 {
        return ;
    }
        let full_resp = reqwest::blocking::get(url.clone());
        if full_resp.is_err() {
            println!("Failed to download {}", url.clone());
            return;
        }
        let resp = full_resp.unwrap().text().unwrap();

        let links = url_links_finder(&resp);
        for link in links {
            println!("Downloading {}", link);
            rec_download(link.clone(), dir_path.clone(), iteration - 1);
        }
        download_print(url_extension_searcher(&resp, "jpg"), dir_path.clone());
        download_print(url_extension_searcher(&resp, "png"), dir_path.clone());
        download_print(url_extension_searcher(&resp, "jpeg"), dir_path.clone());
        download_print(url_extension_searcher(&resp, "gif"), dir_path.clone());
        download_print(url_extension_searcher(&resp, "bmp"), dir_path.clone());
}


 fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = modules::args::Args::default();
    args.parse_args()?;
    println!("url: {}, dir_path: {}, deep: {}", args.url, args.dir_path, args.deep);

    rec_download(args.url.clone(), args.dir_path.clone(), args.deep);

    Ok(())
}