fn main() {
   let name1 = "Ayomide Adesokan";
   println!("My name is {}",name1);

   //find and replace
   let name2 = name1.replace("Ayomide","Adebare");
   println!("You can also call me {}",name2);
   let facaulty = "Faculty of Science and Technology";

   //find amd replace
   let school = facaulty.replace("Faculty", "School");
   println!("I am a student of the {}", school);

}
