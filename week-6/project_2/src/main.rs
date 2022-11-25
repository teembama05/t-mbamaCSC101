use std::io;

fn main() {
    println!("How many siblings do you have?");
    let siblings = String::new();
    io::stdin().read_line(&mut siblings).expect("Failed to read input");
    let siblings: i32 = siblings.trim().parse().expect("Invalid input");

    for siblings >= 1{
        println!("Enter the name of your sibling");
        let name = String::new();
        println!("Enter the age of your sibling");
        let age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age: i32 = age.trim().parse().expect("Invalid input");
    }
   

}