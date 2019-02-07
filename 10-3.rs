use deeply::nested::function as other_function;
fn function() {
    println!("called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called deeply::nested::function()");
        }
    }
}

fn main() {
    other_function(); // useしてるからこれで呼べる
    println!("Entering block");
    {
        use deeply::nested::function; // ブロック内でfunction()をシャドーウィング
        function(); // deeply::nested::functionの方が呼ばれる
        println!("leaving block");
    }
    function();
}