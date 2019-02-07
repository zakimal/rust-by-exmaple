#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    println!("what color is it?");
    match color {
        Color::Red => println!("The color is Red"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r, g, b) =>
            println!("RED: {}, GREEN: {}, BLUE: {}", r, g, b),
        Color::HSV(h, s, v) =>
            println!("HUE: {}, SATURATION: {}, VALUE: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("HUE: {}, SATURATION: {}, LIGHTNESS: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("CYAN: {}, MAGENTA: {}, YELLOW: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("CYAN: {}, MAGENTA: {}, YELLOW: {}, KEY (BLACK): {}!",
                     c, m, y, k),
    }
}