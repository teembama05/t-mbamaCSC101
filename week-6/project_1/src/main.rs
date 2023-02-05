use std::io;


fn main() {
    println!("Please input all values to 1 decimal place");
   
    println!("What would you like to calculate? (1 =Area of trapezium, 2 = Area of rhombus, 3 = Area of parallelogram, 4 = Area of cube , 5 = Volume of cylinder )");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read inout");
    let answer: i32 = answer.trim().parse().expect("Invalid input");

    if answer == 1{
        area_of_trapezium();
    }
    else if answer == 2{
        area_of_rhombus();
    }
    else if answer == 3{
        area_of_parallelogram();
    }
    else if answer == 4{
        area_of_cube();
    }
    else if answer == 5{
        volume_of_cylinder();
    }
    else{
        println!("Sorry this cannot be calculated using this program.");
    }
}
fn area_of_trapezium(){
    println!("Please input the height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f32 = height.trim().parse().expect("Invalid input");

    println!("Please input the first base");
    let mut base1 = String::new();
    io::stdin().read_line(&mut base1).expect("Failed to read inout");
    let base1: f32 = base1.trim().parse().expect("Invalid input");

    println!("Please input the second base");
    let mut base2 = String::new();
    io::stdin().read_line(&mut base2).expect("Failed to read inout");
    let base2: f32 = base2.trim().parse().expect("Invalid input");

    //to calculate area
    let area:f32 = (height/2.0)*(base1+base2);
    println!("The area of the trapezium is {}", area);
}
fn area_of_rhombus(){
    println!("Please input the first diagonal");
    let mut diagonal1 = String::new();
    io::stdin().read_line(&mut diagonal1).expect("Failed to read inout");
    let diagonal1: f32 = diagonal1.trim().parse().expect("Invalid input");

    println!("Please input the second diagonal");
    let mut diagonal2 = String::new();
    io::stdin().read_line(&mut diagonal2).expect("Failed to read inout");
     let diagonal2: f32 = diagonal2.trim().parse().expect("Invalid input");

    let area:f32 = (1.0/2.0)*diagonal1*diagonal2;
     println!("The area of the rhombus is {}", area);
} 
fn area_of_parallelogram(){
    println!("Please input the base");
    let mut base = String::new();
    io::stdin().read_line(&mut base).expect("Failed to read inout");
    let base: f32 = base.trim().parse().expect("Invalid input");

    println!("Please input the altitude");
    let mut altitude = String::new();
    io::stdin().read_line(&mut altitude).expect("Failed to read inout");
    let altitude: f32 = altitude.trim().parse().expect("Invalid input");

    let area:f32 = base*altitude;
    println!("The area of the parallelogram is {}", area);
}
fn area_of_cube(){
    println!("Please input the length of the side");
    let mut length_of_the_side = String::new();
    io::stdin().read_line(&mut length_of_the_side).expect("Failed to read inout");
    let length_of_the_side: f32 = length_of_the_side.trim().parse().expect("Invalid input");

    let area:f32 = 6.0*(length_of_the_side*length_of_the_side);
     println!("The area of the cube is {}", area);
}
fn volume_of_cylinder(){
    println!("Please input the radius");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("Failed to read inout");
    let radius: f32 = radius.trim().parse().expect("Invalid input");

    println!("Please input the height");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read inout");
    let height: f32 = height.trim().parse().expect("Invalid input");

    let mut pi:f32 = 3.142;

    let area:f32 = pi*(radius*radius)*height;
    println!("The volume of the cylinder is {}", area);
}
    
