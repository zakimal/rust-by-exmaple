#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux!");
}

fn main() {
    are_you_on_linux();
    if cfg!(target_os = "linux") {
        println!("yes it is definitely linux");
    } else {
        println!("yes it is definitely not linux");
    }
}