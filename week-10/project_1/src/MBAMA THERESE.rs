use std::io;
use std::io::Write;

fn main() {
     //To determine the person to be called
    println!("WELCOME TO ERNST & YOUNG GLOBAL LIMITED!!!");
    println!{"WHOSE FILE WOULD YOU LIKE TO ACCESS?\n"};
    println!("Enter 7 for Aigbona Juliet and Akpevwe Iloka");
    println!("Enter 8 for Adamu Sagamu and Gbenga Daniels ");
    println!("Enter 9 for Maria Akinsola and Ehis Ero");
    
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let input1:i32 = input1.trim().parse().expect("Failed to read input");

   if input1 == 7{
    code_7();
   }
   else if input1 == 8{
    code_8();
   }
   else if input1 == 9{
    code_9();
   }
   else{
    println!("This cannot run in this program");
   }
}

fn code_7(){
    let consult = vec!["Analytics consulting services", "Customer experience", "Cybersecurity strategy risk compliance and resilience",
        "Digital transformation", "Risk consulting services", "Supply chain and operations", "Technology transformation"];
    
    let assurance = vec!["Audit services", "Climate change and Sustainability services", "Financial accounting advisory services",
        "Forensic and intergrity services", "Private client audit experience", "Accounting Link", "Assurance"];
    
    let name1 = "Aigbona Juliet";
    let name2 = "Akpevwe Iloka";
    let qual1 = "B.Sc.";
    let qual2 = "HND";
    let dept1 = "Consulting";
    let dept2 = "Assurance";
    
    let mut file = std::fs::File::create("Aigbona_Juliet.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name1,qual1,dept1,consult);
    println!("{}'s file has been created!", name1);

    let mut file = std::fs::File::create("Akpevwe_Iloka.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name2,qual2,dept2,assurance);
    println!("{}'s file has been created!", name2);
}

fn code_8(){
    let tax = vec!["Tax planning", "Tax function operations", "Tax policy and controversy", "Global trade", 
        "Tax accounting", "Tax compliance", "Transaction tax"];
    let people_and_workforce = vec!["Change management and experience", "HR transformation", "Integrated workforce mobility", "Learning and development consulting",
        "Recognition and reward advisory", "Workforce analytics", "People and workforce"];

    let name1 = "Adamu Sagamu";
    let name2 = "Gbenga Daniels";
    let qual1 = "B.Sc.";
    let qual2 = "HND";
    let dept1 = "Tax";
    let dept2 = "People and workforce";

    let mut file = std::fs::File::create("Adamu_Sagamu.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name1,qual1,dept1,tax);
    println!("{}'s file has been created!", name1);

    let mut file = std::fs::File::create("Gbenga_Daniels.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name2,qual2,dept2,people_and_workforce);
    println!("{}'s file has been created!", name2);
}

fn code_9(){
    let strat = vec!["Strategy conculting", "Corporate and growth strategy", "Transaction strategy and execution",
        "Restructuring and turnaround strategy", "Industry strategy", "Digital business building", "Commercial strategy"];
    
    let transactions_and_corporate = vec!["Corporate finance", "Divestments and carve-outs", "Sustainability and ESG Service", "M&A advisory",
        "M&A integration", "M&A technology tools", "M&A advanced analytics"];

    let name1 = "Maria Akinsola";
    let name2 = "Ehis Ero";
    let qual1 = "M.Sc.";
    let qual2 = "M.Sc.";
    let dept1 = "Transaction and corporate finance";
    let dept2 = "Strategy";

    let mut file = std::fs::File::create("Maria_Akinsola.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name1,qual1,dept1,transactions_and_corporate);
    println!("{}'s file has been created!", name1);

    let mut file = std::fs::File::create("Ehis_Ero.txt").expect("failed to create");
    writeln!(&mut file, "This file belongs to {}\nWith a qualification of {}\nShe is in {} department\nHer jpb descriptions are {:?}"
        ,name2,qual2,dept2,strat);
    println!("{}'s file has been created!", name2);
}