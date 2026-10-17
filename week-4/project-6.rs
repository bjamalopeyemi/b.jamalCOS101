fn main() {
	let p:f64 = 450_000_000.00;
	let r:f64 = 9.0;
	let n:i32 = 5;

	// calculating amount
	let a = p*(1.0+(r/100.0)).powi(n);
	println!("The amount is {}",a );

	// calculating compound interest
	let ci = a-p;
	println!("The compound interest is {}",ci );

}