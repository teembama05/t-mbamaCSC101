//to find the roots of a quadratic equation

use std::io;

fn main() {
    println!("Please input a value for a");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("This is an invalid input");
   let mut a :f32= input1.trim().parse().expect("This is an invalid number");

   println!("Please input a value for b");
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("This is an invalid input");
   let mut b:f32 = input2.trim().parse().expect("This is an invalid number");

   println!("Please input a value for c");
   let mut input3 = String::new();
   io::stdin().read_line(&mut input3).expect("This is an invalid input");
   let mut c:f32 = input3.trim().parse().expect("This is an invalid number");

   //to find the discriminant
   let e:f32 = 4.0;
   let mut discriminant = (b*b)-(e*a*c);
   
   if discriminant > 0.0
   {
       println!("There are two distinct roots");
   }
   else if discriminant == 0.0
   {
       println!("There is exactly one real root");
   }
   else if discriminant < 0.0
   {
       println!("There are no real roots");
   }
   else {
       println!("Abnormal equation");
   }

   }