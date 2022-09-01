pub fn fb_whileif(num: u32) {
    println!("while_if for fizz_buzz {}", num);

    let mut fb = 1;

    while fb <= num {
        if fb % 15 == 0 {
            println!("{} FizzBuzz", fb);
        } else if fb % 3 == 0 {
            println!("{} Fizz", fb);
        } else if fb % 5 == 0 {
            println!("{} Buzz", fb);
        } else {
            println!("{}", fb);
        }
        fb += 1;
    }
}
