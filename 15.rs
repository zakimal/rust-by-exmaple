// traitは、任意の型となりうるSelfに対して定義されたメソッドの集合のこと。
// 同一トレイト内で定義されたメソッド同士は互いにアクセスできる。

struct Sheep {
    naked: bool,
    name: &'static str
}

trait Animal {
    // スタティックメソッド
    fn new(name: &'static str) -> Self;

    // インスタンスメソッド
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // traitで具体的な実装を示すと、デフォルト実装として扱われる。
    fn talk(&self) {
        print!("{} says {}", self.name(), self.noise());
    }
}

// Sheep構造体の持っているメソッドたち
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haricut!", self.name);
            self.naked = true;
        }
    }
}

// AnimalトレイトをSheep構造体に対して実装する。→Sheep構造体がAnimal（トレイトを満たしている）型としてとして扱える
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep{
            name: name,
            naked: false
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "Baaaaaaah?"
        } else {
            "Baaaaaaah!"
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
}