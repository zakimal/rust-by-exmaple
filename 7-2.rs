fn main() {
    // rustには'loop'というキーワードがある。これは無限ループを作るために使用する。
    // 'while true'と基本的には動作は同じだが、いちいち条件を評価しないので若干高速。
    let mut count = 0u32;
    println!("Let's count until infinity");
    loop {
        count += 1;
        if count == 3 {
            println!("Three! HAHA!!");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough!");
            break;
        }
    }
}