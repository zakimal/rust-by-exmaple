// Dropトレイトはただ一つのメソッドdropのみを提供し、オブジェクトがスコープから抜けた時には自動的に呼ばれて、
// オブジェクトが所有しているメモリ領域を解放する。

struct Droppable {
    name: &'static str
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting the block B");
        }
        println!("Just exited the block B");
        println!("Exiting the block A");
    }
    println!("Just exited the block A");
    drop(_a);
    println!("end of the main function");
}