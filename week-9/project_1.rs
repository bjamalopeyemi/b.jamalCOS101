use std::io::Write;
fn main () {


	let mut file=std::fs::File::create("brewries.txt").expect("create failed");
	file.write_all("Lager:33 Export,Desperados,Goldberg,Gulder,Heineken,Star\n"
		.as_bytes()).expect("Write failed");
	file.write_all("Stout:Legend,Turbo king,Williams\n".as_bytes()).expect("write failed");
	file.write_all("Nonalcholic:Maltina,Amstel Malta,Malta Gold,Fayrouz\n".as_bytes()).expect("write failed");
	println!("\n Data written to file.");

}