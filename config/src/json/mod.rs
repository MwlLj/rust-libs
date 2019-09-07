use crate::default;

use serde::{Serialize, Deserialize};

pub struct CConfig {
    content: String
}

impl CConfig {
    pub fn load<T>(path: &str, t: &T) -> Option<CConfig>
        where T: Serialize {
        let default = match serde_json::to_string_pretty(t) {
            Ok(d) => d,
            Err(_) => {
                return None;
            }
        };
        let config = match default::CConfig::load_by_str(path, default) {
            Some(c) => c,
            None => {
                return None;
            }
        };
        Some(CConfig{
            content: config.content_move()
        })
    }

    pub fn decode<'b, D>(&'b self) -> Option<D>
        where D: Deserialize<'b> {
        let d = match serde_json::from_str(&self.content) {
            Ok(d) => d,
            Err(_) => {
                return None;
            }
        };
        Some(d)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[macro_use]
    extern crate serde_derive;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    struct CNet {
        ip: String,
        port: u16
    }

    #[test]
    fn configTest() {
        let config = CConfig::load("config.json", &CNet{
            ip: "127.0.0.1".to_string(),
            port: 80
        }).unwrap();
        let d = config.decode().unwrap();
        println!("{:?}", d);
    }
}

/*
pub fn load<'a, T, D>(path: &'a str, t: &T) -> Option<D>
    where T: Serialize, D: Deserialize<'a> {
    let default = match serde_json::to_string_pretty(t) {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };
    let config = match default::CConfig::load_by_str(path, default) {
        Some(c) => c,
        None => {
            return None;
        }
    };
    let d = match serde_json::from_str(config.content()) {
        Ok(d) => d,
        Err(_) => {
            return None;
        }
    };
    Some(d)
}
*/
