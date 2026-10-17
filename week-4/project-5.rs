fn main() {
	let toshiba:f64 = 450_000.00;
	let mac:f64 = 1_500_000.00;
	let hp:f64 = 750_000.0;
	let deli:f64 = 2_850_000.00;
	let acer:f64 = 250_000.00;

	//calculating sum
	let sum= toshiba+mac+hp+deli+acer;
	println!("Total sum of sales {}",sum );
	let average = sum/5.0;
	println!("The average is {}",average );

}