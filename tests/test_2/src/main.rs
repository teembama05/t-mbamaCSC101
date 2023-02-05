use std::io;

fn main() {
    println!("Welcome to the Public Service Division");
    Pub_Service();
}
fn Pub_Service(){
    println!("What kind of staff are you? (input 1 for Office administartor, 2 for Academic , 3 for lawyer , 4 for teacher)", );
    let mut staff = String::new();
    io::stdin().read_line(&mut staff).expect("Failed to read input");
    let mut staff:i32 = staff.trim().parse().expect("Invalid input");

    println!("How  many years of experience do you have?");
    let mut experience = String::new();
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let mut experience:i32 = experience.trim().parse().expect("Invalid input");

    if staff == 1 && experience >=1 && experience <=2{
        println!("You are an Intern");
    }
    else if staff ==1 && experience >=3 && experience <=5{
        println!("You are an Administrator");
    }
    else if staff ==1 && experience >=5 && experience <=8{
        println!("You are a Senior Administrator");
    }
    else if staff ==1 && experience >=8 && experience <=10{
        println!("You are an Office Manager");
    }
    else if staff ==1 && experience >=10 && experience <=13{
        println!("You are a Director");
    }else if staff ==1 && experience ==14{
        println!("You are a CEO");
    }
    else if staff == 2 && experience >=1 && experience <=2{
        println!("Sorry you do not have a position");
    }
     else if staff ==2 && experience >=3 && experience <=5{
        println!("You are a Research Assistant");
    }
    else if staff ==2 && experience >=5 && experience <=8{
        println!("You are a PhD Candidate");
    }
    else if staff ==2 && experience >=8 && experience <=10{
        println!("You are a Post-Doc Researcher");
    }
    else if staff ==2 && experience >=10 && experience <=13{
        println!("You are a Senior Lecturer");
    }
    else if staff ==2 && experience ==14{
        println!("You are a Dean");
    }
    else if staff ==3 && experience >=1 && experience <=2{
        println!("You are a Paralegal");
    }
    else if staff ==3 && experience >=3 && experience <=5{
        println!("You are a Junior Associate");
    }
    else if staff ==3 && experience >=5 && experience <=8{
        println!("You are an Associate");
    }
    else if staff ==3 && experience >=8 && experience <=10{
        println!("You are a Senior Associate 1-2");
    }
    else if staff ==3 && experience >=10 && experience <=13{
        println!("You are a senior Associate 3-4");
    }
    else if staff ==3 && experience ==14{
        println!("You are a Partner");
    }
    else if staff ==4 && experience >=1 && experience <=2{
        println!("You are a Placement staff");
    }
    else if staff ==4 && experience >=3 && experience <=5{
        println!("You are a Classroom Teacher");
    }
    else if staff ==4 && experience >=5 && experience <=8{
        println!("You are a Snr Teacher");
    }
    else if staff ==4 && experience >=8 && experience <=10{
        println!("You are a Leading teacher");
    }
    else if staff ==4 && experience >=10 && experience <=13{
        println!("You are a Deputy Principal", );
    }
    else if staff ==4 && experience ==14{
        println!("You are a Principal");
    }
    else{
        println!("Sorry this cannot be determined on this program");
    }


}