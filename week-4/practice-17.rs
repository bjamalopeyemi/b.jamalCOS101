//Rust program to find user age

use std::io;
fn main() {
	let mut age= String::new();

	println!("\nPlease enter your age");
	io::stdin().read_line(&mut age).expect("Not a valid string");
	let age:i32 = age.trim().parse().expect("Not a valid number");

	if age < 0 || age > 120 {
		println!("Invalid age");
	}else if age >=0 && age <=12 {
		println!("Child");
	}else if age >=13 && age <=17{
	   println!("Teenager");
	}else if age >=18 && age <=59{
		println!("Adult");
	}else if age >=60 && age <=120{
		println!("Senior");

	println!("You are a/an {}",age );
	}
}