// データは一度に幾つでもimmutableに借用することができるが、誰かたimmutableに借用している間は、オリジナルのデータを
// mutableに借用することは許されない。
// mutableな借用は一度に一つしかできないが、オリジナルのデータをもう一度借用するためには、mutableな参照がスコープを
// 抜けた後でないといけない。

struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let mut point = Point {
        x: 0, y: 0, z: 0
    };
    {
        let borrowed_point = &point;
        let another_borrowed_point = &point;
        println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x, another_borrowed_point.y, point.z
        );
        // let mutable_borrow = &mut point; // 他の人が借りているのでmutableには借用できない
    }
    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;
        // let y = &point.y; // mutable_borrowがpointをmutableに借りているのでyがimmutableに借りることは許されない
        println!("Point Z coordinate is {}", point.z); // これもダメ。
    }
    println!("Point now has coordinates ({}, {}, {})", point.x, point.y, point.z);
}