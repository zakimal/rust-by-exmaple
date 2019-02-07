fn main() {
    // クロージャ
    fn function (i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 {i + 1};
    let closure_inferred = |i| i + 1;
    let i = 1;
    println!("Function call: {}", function(i));
    println!("Annotated closure: {}", closure_annotated(i));
    println!("Inferred closure: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

    // 外部の環境にある変数を参照することができる
    let professor_x = "Charles Xavier";
    let print = || println!("Professor X's name is {}", professor_x);
    print(); // クロージャ呼び出し
}