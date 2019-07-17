pub fn box2u64<T>(obj: Box<T>) -> u64 {
    let ptr = Box::into_raw(obj);
    ptr as u64
}

pub fn u642box<T>(addr: u64) -> Box<T> {
    let ptr = addr as *mut T;
    unsafe {
        Box::from_raw(ptr)
    }
}

#[test]
fn box2u64Test() {
    let v = Box::new(1);
    let addr = box2u64(v);
    println!("addr: {}", addr);
}

#[test]
fn u642boxTest() {
    let v = Box::new(1);
    let addr = box2u64(v);

    let result = u642box(addr);

    assert_eq!(v, result);
}
