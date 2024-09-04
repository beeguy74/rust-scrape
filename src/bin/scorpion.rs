use std::{env, io::Read, result};
use hex;


struct imageFile {
    content: Vec<u8>,
    extension: String,
}

impl imageFile {
    fn len(&self) -> usize {
        self.content.len()
    }

    fn set_content(&mut self, buf: &mut std::io::BufReader<std::fs::File>) {
        buf.read_to_end(&mut self.content);
    }
}

fn open_file(file_path: &str) -> Result<imageFile, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(file_path)?;
    let mut buf_reader = std::io::BufReader::new(file);
    let mut result = imageFile {
        content: Vec::new(),
        extension: file_path.split('.').last().unwrap().to_string(),
    };
    result.set_content(&mut buf_reader);
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: {} <file>", args.nth(0).unwrap());
        return Err("Invalid number of arguments".into());
    }
    let image = open_file(args.nth(1).unwrap().as_str())?;
    // print first line of file
    println!("Content: {:?}", &image.content[0..8]);
    let hex_content = hex::encode(&image.content);
    // Print the hexadecimal content
    println!("Hex Content: {}", hex_content);

    Ok(())
}