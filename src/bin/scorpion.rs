use std::{env, io::Read, result};
use hex;
use endianness::{read_u32, ByteOrder::BigEndian};
use flate2::read::ZlibDecoder;
use std::collections::HashMap;


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

    fn parse_ihdr_chunk(data: &[u8]) {
        let width = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        let height = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let bit_depth = data[8];
        let color_type = data[9];
        let compression_method = data[10];
        let filter_method = data[11];
        let interlace_method = data[12];
    
        println!("Width: {}", width);
        println!("Height: {}", height);
        println!("Bit Depth: {}", bit_depth);
        println!("Color Type: {}", color_type);
        println!("Compression Method: {}", compression_method);
        println!("Filter Method: {}", filter_method);
        println!("Interlace Method: {}", interlace_method);
    }

    fn parse_ztxt_chunk(data: &[u8]) {
        let null_pos = data.iter().position(|&b| b == 0).unwrap();
        let keyword = &data[..null_pos];
        let compression_method = data[null_pos + 1];
        let compressed_text = &data[null_pos + 2..];

        if compression_method == 0 {
            let mut decoder = ZlibDecoder::new(compressed_text);
            let mut decompressed_text = String::new();
            decoder.read_to_string(&mut decompressed_text).unwrap();
            println!("Keyword: {}", String::from_utf8_lossy(keyword));
            println!("Decompressed Text: {:?}", decompressed_text);

            // let key_value_map = imageFile::parse_hex_key_value(&decompressed_text);
            // for (key, value) in key_value_map {
            //     println!("{}: {}", key, value);
            // }
        } else {
            println!("Unknown compression method: {}", compression_method);
        }
    }

    fn parse_hex_key_value(data: &String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let lines: Vec<&str> = data.split('\n').collect();
    
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    
        map
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
            i += 4;
            chunk_type = &self.content[i..i+4];
            i += 4;
            chunk_data = &self.content[i..i+chunk_length as usize];
            i += chunk_length as usize;
            chunk_crc = &self.content[i..i+4];
            i += 4;
            if chunk_type == b"IDAT" {
            }
            else if chunk_type == b"IHDR" {
                println!("Found IHDR chunk");
                imageFile::parse_ihdr_chunk(chunk_data);
            }
            else if chunk_type == b"zTXt" {
                println!("Found zTXt chunk");
                imageFile::parse_ztxt_chunk(chunk_data);
            }
            else {
                println!("Found {} chunk", String::from_utf8_lossy(chunk_type));
                println!("Length: {}", chunk_length);
                // println!("Text: {}", String::from_utf8(chunk_data));
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