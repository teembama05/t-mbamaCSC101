use std::io;

 fn FacPub(){
        for i in 0..50{
            println!("What is the name of the paper the facaulty has published?");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("Invalid input");
            println!("How many papers has the facaulty published?");
            let mut number = String::new();
            io::stdin().read_line(&mut number).expect("Invalid input");
            let amount:i32 = number.trim().parse().expect("Invalid input");
            if amount >=3 && amount <=5{
                println!("The incentive of the facaulty is #500,000");
            }
            else if amount >=5 && amount <=10{
                println!("The incentive of the facaulty is #800,000");
            }
            else if amount >=10{
                println!("The incentive of the facaulty is #1,000,000");
            }
            else if amount <3{
                println!("The incentive of the facaulty is #100,000");
            }
            else{
                println!("Sorry, this cannot be calculated on this program");
            }
        }
}        
    
fn main(){
   
    FacPub();
}
