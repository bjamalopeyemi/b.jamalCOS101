fn main () {
	let name = "Aisha Lawal";
	let uni:&str = "Pan atlantic university";
	let addr:&str = "Km 52 Lekki-Epe expressway,Ibeju-lekki,lagos";
	println!("Name : {}",name );
	println!("University:{}, \nAddress : {}",uni,addr);

	let department:& 'static str = "Computer science";
	let school:& 'static str = "School of science and technology";
	println!("Department : {}, \nSchool : {}",department,school);
}

