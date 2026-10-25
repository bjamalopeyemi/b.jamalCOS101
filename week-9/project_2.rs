use std::fs::File;
use::std::io::Write;

fn main() {
	let students_name=vec!["Oluchi mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Bianca Edemoh"];
	let matric_number=vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001"];
	let department=vec!["Accounting","Economics","Computer","Electrical","Mechanical"];
	let level=vec!["300","100","200","200","100"];

	let mut file = std::fs::File::create("data.txt").expect("create failed");

		file.write_all("Welcome to PAU's Student Management Information system\n"
		.as_bytes()).expect("Write failed");

	 println!("Student Name | Matric. Number | Department | Level");
    file.write_all("Student_Name | Matric_Number | Department | Level\n".as_bytes()).expect("write failed");

    	

    // one row per student
    for i in 0..students_name.len() {
        // 1. display on screen
        println!("{} | {} | {} | {}", students_name[i], matric_number[i], department[i], level[i]);

	file.write_all(students_name[i].as_bytes()).expect("write failed");
	        file.write_all(" | ".as_bytes()).expect("write failed");

	file.write_all(matric_number[i].as_bytes()).expect("write failed");
	        file.write_all(" | ".as_bytes()).expect("write failed");

	file.write_all(department[i].as_bytes()).expect("write failed");
	        file.write_all(" | ".as_bytes()).expect("write failed");

	file.write_all(level[i].as_bytes()).expect("write failed");
	        file.write_all("\n".as_bytes()).expect("write failed");
}
	println!("\n Data written to file.");



	
}