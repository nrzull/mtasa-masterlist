pub fn get_safe_string(mut source: Vec<u8>) -> String {
    let mut safe_string = String::from("");

    while let Some(v) = source.pop() {
        if let Ok(a) = std::str::from_utf8(&vec![v]) {
            safe_string.push_str(a);
        }
    }

    let safe_string: String = safe_string.chars().rev().collect();
    safe_string
}
