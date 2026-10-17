fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let n:i32 = 5;

	//calculating amount
	let a =p*(1.0 + (r/100.0)).powi(n);
	println!("Amount is {}", a);
	// calculating compound interested
	let ci = a-p;
	println!("Compound interest is {}",ci );
}