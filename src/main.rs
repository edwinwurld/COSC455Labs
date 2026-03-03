fn main() {
    println!("Rust Learning Curve - Chapters 1-5");

    // Variables and Data Types
    let x: i32 = 10;
    let y: f64 = 3.14;
    println!("x = {}, y = {}", x, y);

    // Mutable variable
    let mut z = 5;
    z += 2;
    println!("z = {}", z);

    // If / Else
    if z > 5 {
        println!("z is greater than 5");
    } else {
        println!("z is 5 or less");
    }

    // Loop
    for i in 1..4 {
        println!("i = {}", i);
    }

    // Vector (Collection)
    let numbers = vec![1, 2, 3, 4];
    for num in numbers {
        println!("Vector value: {}", num);
    }

    // Match statement
    let grade = 'A';

    match grade {
        'A' => println!("Excellent"),
        'B' => println!("Good"),
        _ => println!("Keep improving"),
    }

    greet("Edwin");
}

// Function
fn greet(name: &str) {
    println!("Hello, {}!", name);
}