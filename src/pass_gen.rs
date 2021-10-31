use rand::Rng;

pub fn pass_gen() {
    loop {
        println!("Enter password length: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            // if not an integer, skip and ask again
            Err(_) => continue,
        };
        let answer = generate(input);
        println!("generated password: {}", answer);
    }
}

fn generate(length: i32) -> String {
    let mut rng = rand::thread_rng();
    return (0..length)
        .map(|_| rng.gen_range(33..127) as u8 as char)
        .collect();
}
