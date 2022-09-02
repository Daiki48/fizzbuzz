use components::while_if::fb_whileif;
use components::for_range::fb_forrange;

mod components;

fn main() {
    println!("Which FizzBuzz process do you want to check?");
    println!("(1) while_if");
    println!("(2) for_range");
    let variety = get_variety();
    if variety == 1 {
        println!("variety is while_if");
    } else if variety == 2 {
        println!("variety is for_range");
    }

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
