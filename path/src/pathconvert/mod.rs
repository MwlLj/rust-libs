/*
** Absolute path to relative path
** @srcAbs Absolute path that needs to be converted
** @objAbs Target absolute path
*/
pub fn abs2rel(srcAbs: &str, objAbs: &str) -> String {
    let mut rel = String::new();
    let mut sp = '/';
    if cfg!(target_os="windows") {
        sp = '\\';
    }
    let mut srcVec: Vec<&str> = srcAbs.split(sp).collect();
    let mut objVec: Vec<&str> = objAbs.split(sp).collect();
    let srcLen = srcVec.len();
    let objLen = objVec.len();
    if srcLen > 0 && srcVec[srcLen-1].len() == 0 {
        srcVec.pop();
    }
    let mut equalIndex = 0;
    let mut isUnEqual = false;
    for (index, item) in srcVec.iter().enumerate() {
        match objVec.get(index) {
            Some(it) => {
                if it == item {
                } else {
                    equalIndex = index;
                    isUnEqual = true;
                    break;
                }
            },
            None => {
                // src > obj
                rel.push_str("..");
                rel.push(sp);
            }
        }
    }
    if isUnEqual {
        let len = srcLen - (equalIndex + 1) + 1;
        for i in 0..len {
            rel.push_str("..");
            rel.push(sp);
        }
        let v = objVec.split_off(equalIndex);
        for obj in v.iter() {
            if obj.len() > 0 {
                rel.push_str(obj);
                rel.push(sp);
            }
        }
    }
    if (isUnEqual == false) && (srcLen <= objLen) {
        // src <= obj
        rel.push('.');
        rel.push(sp);
    }
    rel
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn abs2relTest() {
        // let mut src = String::from(r#"D:\workspace\gitlab\cbb_utils_cpp\implement\curl_client\0.1.0"#);
        // let mut obj = String::from(r#"D:\workspace\gitlab\cpp_store\third\curl\7.55.1\include"#);
        let mut src = String::from(r#"D:\workspace\gitlab\cbb_utils_cpp\implement\curl_client\0.1.0\"#);
        let mut obj = String::from(r#"D:\workspace\gitlab"#);
        let rel = abs2rel(&src, &obj);
        println!("{:?}", &rel);
    }
}
