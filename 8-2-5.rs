fn main() {
    // クロージャを返す関数を定義することもできる
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    fn_plain();
    fn_mut();
}

fn create_fn() -> Box<Fn()> {
    let text = "Fn".to_owned();
    Box::new(move || println!("This is a {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = "FnMut".to_owned();
    Box::new(move || println!("This is a {}", text))
}