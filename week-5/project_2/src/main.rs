//to determine the annual incentive of a staff 
//based on age and experience
use std::io;

fn main() {
    //for experience
    println!("The employee is experienced(Enter true or false)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("This is an invalid input");
    let mut experience:bool = input1.trim().parse().expect("This is an invalid digit");
    

    //for age
    println!("How old is the employee"); 
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("This is an invalid input");
    let mut age:i32 = input2.trim().parse().expect("This is an invalid digit");
    

    if experience == true && age >= 40
    {
        println!("The incentive of the employee is #1,560,000 per month.");
    }
    else if experience == true && age >=30 && age < 40
    {
        println!("The incentive of the employee is #1,480,000 per month.");
    }
    else if experience == true && age < 28
    {
        println!("The incentive of the employee is #1,300,000 per month.");

    }
    else if experience == false
    {
        println!("The incentive of the employee is #100,000 per month.");

    }
    else {
        println!("Abnormal situation");
    }
}
