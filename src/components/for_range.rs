pub fn fb_forrange(_num: u32) {
    for fb in 1 .. _num + 1 {
        if fb % 15 == 0 {
            println!("{} FizzBuzz", fb);
        } else if fb % 3 == 0 {
            println!("{} Fizz", fb);
        } else if fb % 5 == 0 {
            println!("{} Buzz", fb);
        } else {
            println!("{}", fb);
        }
    };
}
