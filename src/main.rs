use components::while_if::fb_whileif;

use std::env::args;

mod components;

fn main() {
    let variety = args().nth(1).expect("Please specify an mode.");
    println!("Variety is {}", &variety);

    println!("How many times do you repeat?");
    let answer = get_entry();
    println!("answer is {}", answer);

    if variety == "while_if" {
        println!("{:?}", fb_whileif(answer));
    }
}

fn get_entry() -> u32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok();
    return num.trim().parse().ok().unwrap();
}
