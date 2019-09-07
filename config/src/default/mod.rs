use std::path::Path;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

pub trait IDefault {
    fn default(&self) -> Result<String, &str>;
}

pub struct CConfig {
    content: String
}

impl CConfig {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_move(self) -> String {
        self.content
    }
}

impl CConfig {
    pub fn load_by_str(path: &str, default: String) -> Option<CConfig> {
        let mut content = String::new();
        if !Path::new(path).exists() {
            let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .open(path) {
                Ok(f) => f,
                Err(err) => {
                    println!("open file error: {}", err);
                    return None;
                }
            };
            if let Err(err) = file.write_all(default.as_bytes()) {
                println!("write_all error: {}", err);
                return None;
            };
            if let Err(err) = file.flush() {
                println!("flush error: {}", err);
                return None;
            };
            content = default;
        } else {
            let mut file = match File::open(path) {
                Ok(f) => f,
                Err(err) => {
                    println!("open file error: {}", err);
                    return None;
                }
            };
            if let Err(err) = file.read_to_string(&mut content) {
                println!("read_to_string error: {}", err);
                return None;
            };
        }
        Some(CConfig{
            content: content
        })
    }

    pub fn load<T>(path: &str, t: &T) -> Option<CConfig>
        where T: IDefault {
        let default = match t.default() {
            Ok(d) => d,
            Err(err) => {
                println!("default() error: {}", err);
                return None;
            }
        };
        CConfig::load_by_str(path, default)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct CDefaultConfig;
    impl IDefault for CDefaultConfig {
        fn default(&self) -> Result<String, &str> {
            Ok(r#"{
    "ip": "127.0.0.1",
    "port": 8080
}
            "#.to_owned())
        }
    }

    #[test]
    fn defaultConfigTest() {
        let config = match CConfig::load("config.json", &CDefaultConfig{}) {
            Some(c) => c,
            None => {
                assert!(false);
                return;
            }
        };
        println!("{}", config.content());
    }
}
