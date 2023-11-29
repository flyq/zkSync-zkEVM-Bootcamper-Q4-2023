fn main() {
    println!("Welcome, zkevmer!");
    fizz_buzz();
}

fn fizz_buzz() {
    let mut amount = 0;
    for i in 0..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            amount += 1;
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        }
    }
    println!("Amount of fizz buzz: {}", amount);
}
