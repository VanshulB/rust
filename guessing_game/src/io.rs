pub fn run() {
    println!("Please Enter a Number: ");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Err(err) => eprintln!("{}", err),
        _ => (),
    };
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            eprintln!("{}", err);
            0
        }
    };
    let mut vec = Vec::new();
    for num in 1..=input {
        vec.push(num);
    }
    println!("{:?}", vec);
}
