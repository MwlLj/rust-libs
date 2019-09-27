use std::path;

pub trait IWalk {
    fn on_dir(&mut self, path: &str, name: &str);
    fn on_file(&mut self, path: &str, name: &str);
}

struct CWalk {
}

impl CWalk {
    fn walk<F: IWalk>(&self, root: &str, f: &mut F) -> Result<(), &str> {
        let p = path::Path::new(root);
        let dirs = match p.read_dir() {
            Ok(d) => d,
            Err(err) => {
                // println!("read dir err: {}", err);
                return Err("read dir error");
            }
        };
        for entry in dirs {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    return Err("entry error");
                }
            };
            let fileType = match entry.file_type() {
                Ok(t) => t,
                Err(err) => {
                    return Err("file type error");
                }
            };
            let p = entry.path();
            let path = match p.to_str() {
                Some(p) => p,
                None => {
                    return Err("path to_str error");
                }
            };
            let name = match p.file_name() {
                Some(n) => n,
                None => {
                    return Err("file_name error");
                }
            };
            let name = match name.to_str() {
                Some(p) => p,
                None => {
                    return Err("path to_str error");
                }
            };
            if fileType.is_dir() {
                f.on_dir(path, name);
                self.walk(path, f);
            } else if fileType.is_file() {
                f.on_file(path, name);
            }
            // println!("{:?}, {:?}", entry.path().to_str(), entry.file_name());
        }
        Ok(())
    }
}

impl CWalk {
    pub fn new() -> CWalk {
        CWalk{}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct CTest {
    }

    impl IWalk for CTest {
        fn on_dir(&mut self, path: &str, name: &str) {
            println!("dir: path: {}, name: {}", path, name);
        }

        fn on_file(&mut self, path: &str, name: &str) {
            println!("file: path: {}, name: {}", path, name);
        }
    }

    #[test]
    fn walkTest() {
        let walk = CWalk::new();
        walk.walk(".", &mut CTest{});
    }
}

