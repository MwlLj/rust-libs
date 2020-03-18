pub struct FnMutDefer<F: FnMut()>(Option<F>);

impl<F: FnMut()> std::ops::Drop for FnMutDefer<F> {
	fn drop(&mut self) {
        self.0.take().map(|mut f| f());
	}
}

pub fn mut_defer<F>(f: F) -> impl std::ops::Drop
    where F: FnMut() {
    FnMutDefer(Some(f))
}

pub struct FnOnceDefer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> std::ops::Drop for FnOnceDefer<F> {
    fn drop(&mut self) {
        self.0.take().map(|mut f| f());
    }
}

pub fn once_defer<F>(f: F) -> impl std::ops::Drop
    where F: FnOnce() {
    FnOnceDefer(Some(f))
}

#[test]
fn dropTest() {
    use std::collections::HashMap;
    let mut values = HashMap::new();
    {
        let key = "1";
        values.insert(key, "test");
        assert_eq!(values.len(), 1);
        once_defer(|| {
            values.remove(key);
        });
    }
    assert_eq!(values.len(), 0);
}
