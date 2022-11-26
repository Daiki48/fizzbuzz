use components::while_if::fb_whileif;
use components::for_range::fb_forrange;
use components::pattn_match::fb_match;

mod components;

fn main() {
    println!("Which FizzBuzz process do you want to check?");
    println!("(1) while_if");
    println!("(2) for_range");
    println!("(3) for_match");
    
    let variety = get_variety();
    match variety {
        1 => println!("variety is while_if"),
        2 => println!("variety is for_range"),
        3 => println!("variety is for_match"),
        _ => println!("Not Found"),
    };

    println!("How many times do you repeat?");
    let answer = get_entry();
    println!("answer is {}\n", answer);

    match variety {
        1 => fb_whileif(answer),
        2 => fb_forrange(answer),
        3 => fb_match(answer),
        _ => println!("Not Found"),
    };
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
