use std::io;

fn StudentCouncil_VoteX(){
        for i in  0..15{
            //To check if the person is a class rep or not
            println!("You are a class rep (true or false)");
            let mut class_rep = String::new();
            io::stdin().read_line(&mut class_rep).expect("Failed to read input");
            let class_rep:bool = class_rep.trim().parse().expect("Invalid input");
        

            //To check if the person is in 100 level 
            println!("You are in 100 level (true or false)",);
            let mut year = String::new();
            io::stdin().read_line(&mut year).expect("Failed to read input");
            let year:bool = year.trim().parse().expect("Invalid input");

            //To ascertain if the persons CGPA is above 4.0
            println!("Your CGPA is above 4.0 (true or false)");
            let mut cgpa = String::new();
            io::stdin().read_line(&mut cgpa).expect("Failed to read input");
            let cgpa:bool = cgpa.trim().parse().expect("Invalid input");

            if class_rep == true && year == false && cgpa ==true {
                println!("Please enter your name");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read input");
                println!("Please enter your email");
                let mut email = String::new();
                io::stdin().read_line(&mut email).expect("Failed to read input");
                println!("Please enter your department");
                let mut department = String::new();
                io::stdin().read_line(&mut department).expect("Failed to read input");
                println!("Please enter your State of Origin");
                let  mut state = String::new();
                io::stdin().read_line(&mut state).expect("Failed to read input");
                println!("YOU CAN VOTE!");
            }
            else{
            println!("SORRY YOU ARE NOT ELIGIBLE TO VOTE!");
            }
        }
}
fn main(){
    StudentCouncil_VoteX();
    
}
