use std::{env, io::Read, result};
use hex;
use endianness::{read_u32, ByteOrder::BigEndian};


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

    fn exract_png_metadata(&mut self){
        let mut i = 8; // Skip the magic number
        let mut buf: &mut [u8] = &mut [0; 4];
        let mut chunk_length = 0;
        let mut chunk_type: &[u8];
        let mut chunk_data: &[u8];
        let mut chunk_crc: &[u8];
        println!("LEN of png: {}", self.len());
        while i < self.len() {
            buf = &mut self.content[i..i+4];
            chunk_length = read_u32(buf, BigEndian).unwrap();
            println!("chunk length: {}", chunk_length);
            i += 4;
            chunk_type = &self.content[i..i+4];
            i += 4;
            chunk_data = &self.content[i..i+chunk_length as usize];
            i += chunk_length as usize;
            chunk_crc = &self.content[i..i+4];
            i += 4;
            if chunk_type == b"tEXt" || chunk_type == b"zTXt" || chunk_type == b"iTXt" {
                println!("Found text chunk");
                println!("Text: {}", String::from_utf8_lossy(chunk_data));
            }
            else {
                println!("Chunk type: {}", String::from_utf8_lossy(chunk_type));
            }
        }
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


fn launcher(image: &mut imageFile) {
    let hex_content = hex::encode(&image.content);
    let magic_number = &hex_content[0..8];
    println!("Magic Number: {}", magic_number);
    match image.extension.as_str() {
        "png" => {
            if magic_number == "89504e47" {
                println!("File is a PNG");
                image.exract_png_metadata();
            } else {
                println!("File is not a PNG");
            }
        }
        "jpg" | "jpeg" => {
            if magic_number == "ffd8ffe0" {
                println!("File is a JPEG");
            } else {
                println!("File is not a JPEG");
            }
        }
        "gif" => {
            if magic_number == "47494638" {
                println!("File is a GIF");
            } else {
                println!("File is not a GIF");
            }
        }
        "bmp" => {
            if magic_number == "424d" {
                println!("File is a BMP");
            } else {
                println!("File is not a BMP");
            }
        }
        _ => {
            println!("File extension not supported");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: {} <file>", args.nth(0).unwrap());
        return Err("Invalid number of arguments".into());
    }
    let mut image = open_file(args.nth(1).unwrap().as_str())?;
    launcher(&mut image);

    Ok(())
}