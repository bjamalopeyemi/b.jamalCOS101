//Rust program to determine grades of a student
use std::io;

fn main() {
	let mut score = String::new();

	println!("\nPlease enter your score out of 100");
	io::stdin().read_line(&mut score).expect("Not a valid string");
	let score:f32 = score.trim().parse().expect("Not a valid number");

 if score > 100.0 || score < 0.0 {
  println!("Invalid score");

 } else if score >=70.0 && score <=100.0 {
 	println!("Excellent");
 }else if score >=50.0 && score < 70.0{
 	println!("pass");
 }else if score >=40.0 && score <50.0{
 	println!("Resit");
 }else {
 	println!("fail");
 }

  }  
