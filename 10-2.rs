mod my {
    // 任意の型Tをcontentsフィールドに収めるWhiteBoxという構造体を定義
    pub struct WhiteBox<T> {
        pub contents: T,
    }
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T, // private
    }
    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox{
                contents,
            }
        }
    }
}

fn main() {
    let white_box = my::WhiteBox{contents: "public information"};
    println!("The white box contains: {}", white_box.contents);
    let _black_box = my::BlackBox::new("classified information");
}