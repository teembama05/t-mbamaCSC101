//food menu and discount determiner

use std::io;

fn main() {
    println!("Menu                                      Price");
    println!("Poundo Yam/Edinkaiko Soup                 -#3,200");
    println!("Fried Rice & Chicken                      -#3,000");
    println!("Amala & Ewedu Soup                        -#2,500");
    println!("Eba & Egusi Soup                          -#2,000");
    println!("White Rice & Stew                         -#2,500");
    println!(" input '1' for pounded yam/Edinkaiko Soup \n input '2' for fried Rice & Chicken \n input '3' for Amala & Ewedu Soup \n input '4' for Eba & Egusi Soup Soup \n input '5' for White Rice & Stew. ");
    println!("What would you like to order?");
    println!("ORDERS ABOVE #10 000 GET 5% DISCOUNT!!!!");

    let mut p:f32 = 3_200.0;
    let mut f:f32 = 3_000.0;
    let mut a:f32 = 2_500.0;
    let mut e:f32 = 2_000.0;
    let mut w:f32 = 2_500.0;
    let mut price:f32 = 0.0;

     //To know what is to be ordered
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("This is an invalid input");
    let mut order:i32 = order.trim().parse().expect("This is an invalid number");

    if order == 1 {
        //To know the quantity to be ordered
        println!("How many would you like to order?");
        let mut required = String::new();
        io::stdin().read_line(&mut required).expect("This is a invalid input");
        let mut quantity:f32 = required.trim().parse().expect("This is an invalid number");

        let price = p * quantity;
        println!("Your actual amount is {}",price);
        let discount: f32 = (price * 5.0)/100.0;
            if price > 10_000.0{
             let amount: f32 = price-discount;
             println!("Your discounted amount is {}",amount);
             println!("Enjoy your meal!!");
            }
        
        }
        else if order == 2 {
            println!("How many would you like to order?");
        let mut required = String::new();
        io::stdin().read_line(&mut required).expect("This is a invalid input");
        let mut quantity:f32 = required.trim().parse().expect("This is an invalid number");

        let price = f * quantity;
        println!("Your actual amount is {}", price );
        let discount: f32 = (price * 5.0)/100.0;
            if price > 10_000.0{
                 let amount: f32 = price-discount;
                println!("Your discounted amount is {}",amount);
                println!("Enjoy your meal!!");
            }
        }
        else if order == 3{
            println!("How many would you like to order?");
        let mut required = String::new();
        io::stdin().read_line(&mut required).expect("This is a invalid input");
        let mut quantity:f32 = required.trim().parse().expect("This is an invalid number");

        let price = a * quantity;
        println!("Your actual amount is {}", price);
        let discount: f32 = (price * 5.0)/100.0;
           if price > 10_000.0{
            let amount: f32 = price-discount;
               println!("Your discounted amount is {}",amount);
               println!("Enjoy your meal!!");
            }

        }
        else if order == 4{
            println!("How many would you like to order?");
        let mut required = String::new();
        io::stdin().read_line(&mut required).expect("This is a invalid input");
        let mut quantity:f32 = required.trim().parse().expect("This is an invalid number");

        let price = e * quantity;
        println!("Your actual amount is {}", price);
        let discount: f32 = (price * 5.0)/100.0;
            if price > 10_000.0{
            let amount: f32 = price-discount;
              println!("Your discounted amount is {}",amount);
              println!("Enjoy your meal!!");
            }
        }
        else if order == 5{
            println!("How many would you like to order?");
        let mut required = String::new();
        io::stdin().read_line(&mut required).expect("This is a invalid input");
        let mut quantity:f32 = required.trim().parse().expect("This is an invalid number");

        let price = w * quantity;
        println!("Your actual amount is {}", price);
        let discount: f32 = (price * 5.0)/100.0;
            if price > 10_000.0{
                let amount: f32 = price-discount;
               println!("Your discounted amount is {}",amount);
               println!("Enjoy your meal!!");
            }
        }
        else{
            println!("Not on the menu, sorry.");
        }
       


    }
  