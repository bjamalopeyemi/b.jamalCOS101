//Rust program to check APS level
use std::io;
fn main() {
	println!("\n Welcome to the APS level checker");

	let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
	let academic =vec!["Research assistant","Phd candidate","Post-Doc Researcher","Senior lecturer","Dean"];
	let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate(1-2 years)","Senior Associate(3-4 years)","Partner"];
	let teacher = vec!["Placement","Classroom Teacher","Senior teacher","Leading teacher","Deputy principal","principal"];
	let public_servant= vec!["APS(1-2)","APS(3-5)","APS(5-8)","EL1(8-10)","EL2(10-13)","SES"];

	//find out user field
	loop {
		println!("\n These are the available fields:
			1.Office Admin 
			2.Academic 
			3.lawyer
			4.teacher");

		let mut input1=String::new();
		println!("what field do you work in (Enter from 1-4)");
		io::stdin().read_line(&mut input1).expect("Failed to read input");
		let mut field:u8 = input1.trim().parse().expect("Please enter a valid number");

	//find out user job 
	if field==1 {
		loop{
			println!("\n You have choosen Office Admin");
			for i in 0..office_admin.len() {
				println!("{}.{}",i+1,office_admin[i]);
			}
			println!("\n Which of the above jobs do you do(Select from 1 to 6)?");

		let mut input2=String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let mut job:usize = input2.trim().parse().expect("Please enter a valid number");

		if job<1 || job>6 {
			println!("\n Please select from 1-6");

		}else {
			println!("\n Your Aps level is {}",public_servant[job-1] );
			break;
		}

		}
		break;
	}else if field==2 {
		loop{
			println!("\n You have choosen Academic");
			for i in 0..academic.len() {
				println!("{}.{}",i+1,academic[i]);
			}
			println!("\n Which of the above jobs do you do(Select from 1 to 6)?");

		let mut input2=String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let mut job:usize = input2.trim().parse().expect("Please enter a valid number");

		if job<1 || job>6 {
			println!("\n Please select from 1-6");

		}else {
			println!("\n Your Aps level is {}",public_servant[job-1] );
			break;
		}
	}
	break;

} else if field==3 {
		loop{
			println!("\n You have chosen Lawyer");
			for i in 0..lawyer.len() {
				println!("{}.{}",i+1,lawyer[i]);
			}
			println!("\n Which of the above jobs do you do(Select from 1 to 6)?");

		let mut input2=String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let mut job:usize = input2.trim().parse().expect("Please enter a valid number");

		if job<1 || job>6 {
			println!("\n Please select from 1-6");

		}else {
			println!("\n Your Aps level is {}",public_servant[job-1] );
			break;
		}

}
break;
} else if field==4 {
		loop{
			println!("\n You have choosen Teacher");
			for i in 0..teacher.len() {
				println!("{}.{}",i+1,teacher[i]);
			}
			println!("\n Which of the above jobs do you do(Select from 1 to 6)?");

		let mut input2=String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let mut job:usize = input2.trim().parse().expect("Please enter a valid number");

		if job<1 || job>6 {
			println!("\n Please select from 1-6");

		}else {
			println!("\n Your Aps level is {}",public_servant[job-1] );
			break;
		}
	}
	break;
}else {
	println!("\nSelect from 1 to 4.");
}
}
}

