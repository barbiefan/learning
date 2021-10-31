pub fn fibb() {
    loop {
        println!("What nth fibb number?:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            // if not an integer, skip and ask again
            Err(_) => continue,
        };
        let ending = match input % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        let answer: i32 = nthfibb(input);
        println!("{}{} fibb number is {}", input, ending, answer);
    }
}

fn nthfibb(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return nthfibb(n - 1) + nthfibb(n - 2);
    }
}
