use std::io;

fn main() {
    println!("Enter the number: ");

    let mut largest = String::new();
    io::stdin().read_line(&mut largest)
        .expect("Failed to read line");

    let largest = largest.trim().parse::<i32>()
        .expect("Please type a number!");

    for i in (1..=largest).rev() {
        for _j in 1..=i {
            print!("*");
        }
        println!();
    }
}
