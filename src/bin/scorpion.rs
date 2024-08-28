use std::{env, io::Read};

fn open_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content);
    Ok(content)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: {} <file>", args.nth(0).unwrap());
        return Ok(());
    }
    let content = open_file(args.nth(1).unwrap().as_str())?;
    // print first line of file
    println!("Content: {}", &content[0..content.find('\n').unwrap()]);

    Ok(())
}