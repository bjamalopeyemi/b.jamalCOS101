fn main() {
	let p:f64 = 500_000.00;
	let r:f64 = 8.0;
	let n:i32 = 4;

	//calculating value after 4 years
	let a = p*(1.0-(r/100.0)).powi(n);
	println!("depreciation is {}",a );
}