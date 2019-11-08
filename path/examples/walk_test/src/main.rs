use path::walk;
use rust_parse::cmd::CCmd;

fn main() {
    let mut cmdHandler = CCmd::new();
    let root = cmdHandler.register("-root", "root");
    cmdHandler.parse();

    let root = root.borrow();

    walk::walk_one_fn(&*root, &mut |path: &str, name: &str, t: walk::Type| -> bool {
        match t {
            walk::Type::Dir => {
                println!("{:?}", path);
                true
            },
            walk::Type::File => {
                println!("{:?}", path);
                true
            }
        }
    });
}
