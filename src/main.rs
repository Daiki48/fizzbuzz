use components::while_if::fb_whileif;
use components::for_range::fb_forrange;

// use std::env::args;

mod components;

fn main() {
    // let variety = args().nth(1).expect("Please specify an mode.");
    // println!("Variety is {}", &variety);

    println!("Which FizzBuzz process do you want to check?");
    println!("(1) while_if");
    println!("(2) for_range");
    let variety = get_variety();
    println!("variety is {}", variety);

    println!("How many times do you repeat?");
    let answer = get_entry();
    println!("answer is {}", answer);

    if variety == 1 {
        fb_whileif(answer);
    } else if variety == 2 {
        fb_forrange(answer);
    }
}

fn get_entry() -> u32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok();
    return num.trim().parse().ok().unwrap();
}

fn get_variety() -> u32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).ok();
    return num.trim().parse().ok().unwrap();
}
