use std::io;


fn main() {
    println!("Please input all values to 1 decimal place");
    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();
    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();
    let mut base = String::new();
    let mut altitude = String::new();
    let mut length_of_the_side = String::new();
    let mut radius = String::new();
    let mut height = String::new();
    let mut pi:f32 = 3.142;
    println!("What would you like to calculate? (1 =Area of trapezium, 2 = Area of rhombus, 3 = Area of parallelogram, 4 = Area of cube , 5 = Volume of cylinder ");
    let answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read inout");
    let answer: i32 = answer.trim().parse().expect("Invalid input");


    //area_of_trapezium();
    //area_of_rhombus();
    //();
    //area_of_cube();
    //volume_of_cylinder();
    if answer == 1{
        fn area_of_trapezium(){
            io::stdin().read_line(&mut height).expect("Failed to read input");
            let height: f32 = height.trim().parse().expect("Invalid input");

            io::stdin().read_line(&mut base1).expect("Failed to read inout");
            let base1: f32 = base1.trim().parse().expect("Invalid input");

             io::stdin().read_line(&mut base2).expect("Failed to read inout");
            let base2: f32 = base2.trim().parse().expect("Invalid input");

            //to calculate area
            let area:f32 = (height/2.0)*(base1+base2);
            println!("The area of the trapezium is {}", area);
        }
        area_of_trapezium();
    }
    else if answer == 2{
        fn area_of_rhombus(){
            io::stdin().read_line(&mut diagonal1).expect("Failed to read inout");
            let diagonal1: f32 = diagonal1.trim().parse().expect("Invalid input");

            io::stdin().read_line(&mut diagonal2).expect("Failed to read inout");
            let diagonal2: f32 = diagonal2.trim().parse().expect("Invalid input");

            let area:f32 = (1.0/2.0)*diagonal1*diagonal2;
            println!("The area of the rhombus is {}", area);
        } 
        area_of_rhombus();   
    }
    else if answer == 3{
        fn area_of_parallelogram(){
            io::stdin().read_line(&mut base).expect("Failed to read inout");
            let base: f32 = base.trim().parse().expect("Invalid input");

            io::stdin().read_line(&mut altitude).expect("Failed to read inout");
            let altitude: f32 = altitude.trim().parse().expect("Invalid input");

            let area:f32 = base*altitude;
            println!("The area of the parallelogram is {}", area);
        }
        area_of_parallelogram();
    }
    else if answer == 4{
        fn area_of_cube(){
            io::stdin().read_line(&mut length_of_the_side).expect("Failed to read inout");
            let length_of_the_side: f32 = length_of_the_side.trim().parse().expect("Invalid input");

            let area:f32 = 6.0*(length_of_the_side*length_of_the_side);
            println!("The area of the cube is {}", area);
        }
        area_of_cube();
    }
    else if answer == 5{
        fn volume_of_cylinder(){
            io::stdin().read_line(&mut radius).expect("Failed to read inout");
            let radius: f32 = radius.trim().parse().expect("Invalid input");

            io::stdin().read_line(&mut height).expect("Failed to read inout");
            let height: f32 = height.trim().parse().expect("Invalid input");

            let area:f32 = pi*(radius*radius)*height;
            println!("The volume of the cylinder is {}", area);
        }
        volume_of_cylinder();
    }
    else{
        println!("Sorry this program cannot calculate that");
    }
}
