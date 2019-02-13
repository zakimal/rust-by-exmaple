// letを介してデストラクタやパターンマッチングを行う時、refキーワードを用いて構造体やタプルのフィールドへのリファレンス
// を得ることができる

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y:i32
}

fn main() {
    let x = 'Q';
    let ref ref_c1 = x;
    let ref_c2 = &x;
    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point {
        x: 0,
        y: 0
    };

    // デストラクタ
    let _copy_of_x = {
        let Point {
            x: ref ref_to_x,
            y: _
        } = point;
        *ref_to_x
    };

    let mut mutable_point = point; // mutableなコピー

    {
        // デストラクタ
        let Point {
            x: _,
            y: ref mut mut_ref_to_y
        } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2;
    }
    println!("tuple is {:?}", mutable_tuple);
}