use std::io;

fn main() {
    println!("How many siblings do you have?");
    let mut siblings = String::new();
    io::stdin().read_line(&mut siblings).expect("Failed to read input");
    let mut siblings: i32 = siblings.trim().parse().expect("Invalid input");

    while siblings > 1{
        println!("Enter the first name of your sibling");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid input");
        println!("Enter the age of your sibling");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
        let age: i32 = age.trim().parse().expect("Invalid input");

        if age > 18 {
            age_greater_than_18();
        }
        else{
            age_less_than_18();
        }
    }
}
fn age_greater_than_18(){
    println!("Your sibling is married?(true or false)");
    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed to read input");
    let status:bool = status.trim().parse().expect("Invalid input");
    if status == true{
        married();
    }
    else{
        single();
    }
}
fn single(){
    println!("What university does this sibling attend?");
    let mut uni = String::new();
    io::stdin().read_line(&mut uni).expect("Failed to read input");

    println!("What course is this sibling studying?");
    let mut course = String::new();
    io::stdin().read_line(&mut course).expect("Failed to reaad input");
}
fn married(){
    println!("Does this sibling have children?");
    let mut child = String::new();
    io::stdin().read_line(&mut child).expect("Failed to read input");

    println!("What city does this sibling live in?");
    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read input");
}
fn age_less_than_18(){
    println!("This sibling has written WAEC(true or false)");
    let mut waec = String::new();
    io::stdin().read_line(&mut waec).expect("Failed to read input");
    let waec:bool = waec.trim().parse().expect("Invalid input");
    if waec == true{
        println!("What secondary school did this sibling attend?");
        let mut school = String::new();
        io::stdin().read_line(&mut school).expect("Invalid input");
    }
    else{
        println!("What class is this sibling in currently");
        let mut class = String::new();
        io::stdin().read_line(&mut class).expect("Invalid input");
    }
}
