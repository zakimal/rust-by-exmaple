fn main() {
    let reference = &4; // 'i32'型の値4へのリファレンスを'reference'にバインド
    match reference {
        &val => println!("Got ad value via destructuring: {:?}", val), // 'i32'型の変数'val'
    }
    match *reference {
        val => println!("Got a value via de-referencing: {:?}", val), // de-referencingしている
    }
    let _not_a_reference = 3;
    let ref _is_a_reference = 3; // こっちは要素へのリファレンスが作成されて束縛される
    let value = 5;
    let mut mut_value = 5;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => { // mutableなreferenceをとってde-referencingしている
            *m += 10;
            println!("We added 10. 'mut_value': {:?}", m);
        },
    }
}