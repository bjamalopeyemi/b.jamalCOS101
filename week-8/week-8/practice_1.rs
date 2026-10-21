fn main() {
	//Using Vec::new()
	let v : Vec<i64> = Vec::new();

	//Printing the size of a vector
	println!("\nThe length of vec::new is :{}",v.len() );

	//using macro
	let v = vec! ["Grace","Effiong","Basil","Kareem","Susan"];

	//Printing the size of vector
	println!("\nThe length of vec macro is: { }",v.len() );
}