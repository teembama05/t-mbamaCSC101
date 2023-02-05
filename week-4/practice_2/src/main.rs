use std::io;

// fn main() {
//    let empty_string = String::new();
//    println!("Length of empty_string is {}",empty_string.len());

//    let content_string = String::from("ComputerScience");
//    println!("Length of content_string is {}",content_string.len());
// }

fn main(){
    let mut check:bool = true;

    while check == true{
        println!("Hello");

        let mut ans = String::new();
        println!("Enter condition(true or false)");
        io::stdin().read_line(&mut ans).expect("This is an invalid input");
        let mut ans:bool = ans.trim().parse().expect("This is an invalid number");
        if ans == false{
            check = false;

        }
    }
}
