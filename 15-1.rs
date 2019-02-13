// Rustコンパイラには[#derive]アトリビュートをプレフィックスとしてつけることで、特定のトレイトに対して
// 標準的な実装を提供する機能がある

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);
    // println!("One second look like: {}", _one_second);

    let foot = Inches(21);
    println!("One foot = {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = {
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        }
    };
    println!("One foot is {} than one meter", cmp)
}