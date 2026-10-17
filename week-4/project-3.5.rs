fn main() {
	let p:f64 = 210_000;
	let r:f64 = 5.0;
	let n:i32 = 3;

	//calculating amount
	a =p*(1.0-(r/100.0)).powi(n);
	println!("Amount is {}",a);
}