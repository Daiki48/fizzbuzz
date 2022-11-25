pub fn fb_match(_num: u32) {
    for fb in 1 ..=_num {
        match (fb % 3, fb % 5) {
            (0, 0) => println!("{} FizzBuzz", &fb),
            (0, _) => println!("{} Fizz", &fb),
            (_, 0) => println!("{} Buzz", &fb),
            _ => println!("{}", &fb),
        };
    }
}
