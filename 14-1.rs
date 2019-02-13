// Rustの変数は、単にデータをスタックに保持するものであるとは捉えず、変数はメモリ上のある領域を保有している
// と考える。RustはResource Acquisition Is Initializationを強制し、その変数のスコープを抜けたら
// 即座にデストラクタがメモリ領域を解放する。この挙動はメモリリークを防ぐのに役立つ。

fn create_box() {
    // 32bitの整数3をヒープ上に確保
    let _box1 = Box::new(3i32);
    // ここで_box1は廃棄される
}

fn main() {
    let _box2 = Box::new(5i32);
    {
        let _box3 = Box::new(4i32);
    }
    for _ in 0u32..1_000 {
        create_box();
    }
}