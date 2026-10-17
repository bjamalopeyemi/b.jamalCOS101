fn main() {
	let hp:f64 = 600_000.00;
	let dell:f64 = 1_800_000.00;
	let lenovo:f64 = 1_200_000.00;
	let acer:f64 = 500_000.00;
	let mac:f64 = 1_700_000.00;

	//Calculating sum
	let sum = hp+dell+lenovo+acer+mac;
	println!("Total sum of sales is {}",sum );
	//calculating average
	let average = sum/5.0;
	println!("Total average is {}",average );
}