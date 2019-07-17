use pointer::convert::box_u64;

fn box2u64Test() {
    let v = Box::new(1);
    let addr = box_u64::box2u64(v);
    println!("addr: {}", addr);
}

fn u642boxTest() {
    let v = Box::new(1);
    let addr = box_u64::box2u64(v);
    let result = box_u64::u642box::<u32>(addr);
    println!("{:?}", result);
}

fn main() {
    box2u64Test();
    u642boxTest();
}
