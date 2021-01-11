use ferris_says::say;
use std::io::{stdout, BufWriter};

fn foo_bar() {
    println!("Another function?");
}

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Rust, are you compiling? Does this change the width? Reader: Yes, it does");
    let width = message.chars().count();
    println!("{}", width);

    foo_bar();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
