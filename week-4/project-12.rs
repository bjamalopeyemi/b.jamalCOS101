//rust program for student grde evaluator
use std::io;

fn main() {
	//inputs score
	let mut name = String::new();
	let mut score1 = String::new();
	let mut score2 = String::new();
	let mut score3 = String::new();

//students name
println!("\nEnter students name:");
io::stdin().read_line(&mut name).expect("Not a valid string");
let n:String = name.trim().parse().expect("Not a valid input");


println!("Enter value for score1:");
io::stdin().read_line(&mut score1).expect("Not a valid string");
let score1:f32 =score1.trim().parse().expect("Not a valid number");

println!("Enter value for score2:");
io::stdin().read_line(&mut score2).expect("Not a valid string");
let score2:f32 =score2.trim().parse().expect("Not a valid number");

println!("Enter value for score3:");
io::stdin().read_line(&mut score3).expect("Not a valid string");
let score3:f32 =score3.trim().parse().expect("Not a valid number");

//calculating average
let average = (score1+score2+score3) /3.0;

//determining the average
let grade = if average >=70.0 && average <=100.0{"A"

}else if average >=60.0 && average <70.0 {"B"

}else if average >=50.0 && average <60.0{"C"

}else if average >=45.0 && average <50.0 {"D"

}else {"F"};

println!("Dear {}",n);
println!("Your average is {}", average);
println!("You got a/an {} ",grade);
}
