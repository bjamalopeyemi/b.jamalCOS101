//Rust program to determine the annual incentive of employees
use std::io;

fn main() {

//input
let mut input1 = String::new();
let mut input2 = String::new();



 //incentive Amounts
 let tier1 = "N1,560,000";
 let tier2 = "N1,480,000";
 let tier3 = "N1,300,000";
 let tier4 = "N100,000";

 //Experienced choice
 println!("\n Are you an experienced employee?(true/false)");
 io::stdin().read_line(&mut input2).expect("Not a valid string");
 let exp:bool = input2.trim().parse().expect("Not a valid input,Input true/false");

if exp == true{
 //Age input
 println!("\nPlease input your age");
 io::stdin().read_line(&mut input1).expect("Not a valid string");
 let age:i8 = input1.trim().parse().expect("Not a valid age");


 //Incentive  choice 
 if exp == true {
 	if age >= 40{
 		println!("Your annual incentive is {}",tier1 );
 	}
 	else if age>= 30 && age < 40{
 	println!("Your annual incentive is {}",tier2 ); 
 }
     else if age < 28 {
     	println!("Your annual incentive is {}",tier3);
     }

 }
}
 else {
 	println!("Your annual incentive is: {}",tier4 );
 }
}


