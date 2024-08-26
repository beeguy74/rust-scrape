use std::env;

pub struct Args {
    pub url: String,
    pub dir_path: String,
    pub deep: i32,
}

impl Args {
    pub fn default() -> Self {
        Args {
            url: "https://www.osnews.com".to_string(),
            dir_path: "./data".to_string(),
            deep: 1,
        }
    }

    pub fn parse_args(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut i = 1;
        while i < env::args().len() {
            match env::args().nth(i).unwrap().as_str() {
                "-r" => {
                    self.url = env::args().nth(i + 1).unwrap();
                    i += 2;
                }
                "-p" => {
                    self.dir_path = env::args().nth(i + 1).unwrap();
                    i += 2;
                }
                "-l" => {
                    self.deep = env::args().nth(i + 1).unwrap().parse::<i32>().unwrap();
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }
        Ok(())
    }
}