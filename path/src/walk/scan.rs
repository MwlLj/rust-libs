use std::path;

pub trait IWalk {
    fn on_dir(&mut self, path: &str, name: &str) -> bool;
    fn on_file(&mut self, path: &str, name: &str) -> bool;
}

pub struct CWalk {
}

impl CWalk {
    pub fn walk<F: IWalk>(&self, root: &str, f: &mut F) -> Result<(), &str> {
        let p = path::Path::new(root);
        let dirs = match p.read_dir() {
            Ok(d) => d,
            Err(err) => {
                // println!("read dir err: {}", err);
                return Err("read dir error");
            }
        };
        let mut ds = Vec::new();
        for entry in dirs {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    // return Err("entry error");
                    continue;
                }
            };
            let fileType = match entry.file_type() {
                Ok(t) => t,
                Err(err) => {
                    // return Err("file type error");
                    continue;
                }
            };
            let p = entry.path();
            let path = match p.to_str() {
                Some(p) => p,
                None => {
                    // return Err("path to_str error");
                    continue;
                }
            };
            let name = match p.file_name() {
                Some(n) => n,
                None => {
                    // return Err("file_name error");
                    continue;
                }
            };
            let name = match name.to_str() {
                Some(p) => p,
                None => {
                    // return Err("path to_str error");
                    continue;
                }
            };
            if fileType.is_dir() {
                if !f.on_dir(path, name) {
                    return Ok(())
                }
                ds.push(path.to_string());
            } else if fileType.is_file() {
                if !f.on_file(path, name) {
                    return Ok(())
                }
            }
            // println!("{:?}, {:?}", entry.path().to_str(), entry.file_name());
        }
        for dir in ds {
            self.walk(&dir, f);
        }
        Ok(())
    }
}

impl CWalk {
    pub fn new() -> CWalk {
        CWalk{}
    }
}

struct CDefault<'b, DirF: FnMut(&str, &str) -> bool, FileF: FnMut(&str, &str) -> bool> {
    dirF: &'b mut DirF,
    fileF: &'b mut FileF
}

impl<'b, DirF, FileF> IWalk for CDefault<'b, DirF, FileF>
    where DirF: FnMut(&str, &str) -> bool
    , FileF: FnMut(&str, &str) -> bool {
    fn on_dir(&mut self, path: &str, name: &str) -> bool {
        (self.dirF)(path, name)
    }

    fn on_file(&mut self, path: &str, name: &str) -> bool {
        (self.fileF)(path, name)
    }
}

pub fn walk<'a, DirF, FileF>(root: &'a str, dirF: &mut DirF, fileF: &mut FileF) -> Result<(), &'a str>
    where DirF: FnMut(&str, &str) -> bool
    , FileF: FnMut(&str, &str) -> bool {
    let mut default = CDefault{
        dirF: dirF,
        fileF: fileF
    };
    let walker = CWalk::new();
    if let Err(err) = walker.walk(root, &mut default) {
        return Err("walk error");
    };
    Ok(())
}

pub enum Type {
    Dir,
    File
}

struct COneFnDefault<'a, F: FnMut(&str, &str, Type) -> bool> {
    f: &'a mut F
}

impl<'a, F> IWalk for COneFnDefault<'a, F>
    where F: FnMut(&str, &str, Type) -> bool {
    fn on_dir(&mut self, path: &str, name: &str) -> bool {
        (self.f)(path, name, Type::Dir)
    }

    fn on_file(&mut self, path: &str, name: &str) -> bool {
        (self.f)(path, name, Type::File)
    }
}

pub fn walk_one_fn<'a, F>(root: &'a str, f: &mut F) -> Result<(), &'a str>
    where F: FnMut(&str, &str, Type) -> bool {
    let mut default = COneFnDefault{
        f: f
    };
    let walker = CWalk::new();
    if let Err(err) = walker.walk(root, &mut default) {
        return Err("walk error");
    };
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    struct CTest {
    }

    impl IWalk for CTest {
        fn on_dir(&mut self, path: &str, name: &str) -> bool {
            println!("dir: path: {}, name: {}", path, name);
            true
        }

        fn on_file(&mut self, path: &str, name: &str) -> bool {
            println!("file: path: {}, name: {}", path, name);
            true
        }
    }

    #[test]
    #[ignore]
    fn walkTest() {
        let walk = CWalk::new();
        walk.walk(".", &mut CTest{});
    }
}

