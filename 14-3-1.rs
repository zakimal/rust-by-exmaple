// mutableなデータは&mut Tで読み書きのできる形で借用することができる。
// immutableなデータはどうあがいてもmutableに借用できない。

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str, // このプログラムが走っている間はメモリ上にread-onlyで保持される文字列への参照
    title: &'static str,
    year: u32
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {}", book.title, book.year);
}

fn main() {
    let immutable_book = Book {
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };
    let mut mutable_book = immutable_book;
    borrow_book(&immutable_book);
    borrow_book(&mutable_book);
    new_edition(&mut mutable_book);
    // new_edition(&mut immutable_book);
}