pub fn run() {
    println!("Hello from the print.rs file");
    println!("Number: {}", 1);
    println!("Hey this is {} from {}", "Vanshul", "Gwalior");
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Vanshul", "Gwalior", "code"
    );
    println!(
        "{name} likes to play {activity}",
        name = "Vanshul",
        activity = "Cricket"
    );
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    println!("{:?}", (12, true, "Hello"));
    println!("10 + 10 = {}", 10 + 10);
}
