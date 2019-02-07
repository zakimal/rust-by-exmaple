fn main() {
    let number = Some(7); // 'Some'は値をラップしているだけ
    let letter: Option<i32> = None; // Option<T>は「型Tの値をラップした'Some'もしくは'None'。値が存在するもしくは値が存在しないと言うことを表現する
    let emotion: Option<i32> = None;
    if let Some(i) = number {
        println!("Matched: {:?}!", i);
    }
    if let Some(i)  = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    let i_like_letters = false;
    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }
}