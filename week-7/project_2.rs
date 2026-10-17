use std::io;
	fn trapezium(height:f64,base1:f64,base2:f64)->f64{height/2.0*(base1+base2)}
	fn rhombus_area(diagonal1:f64,diagonal2:f64)->f64{0.5*diagonal1*diagonal2}
	fn parallelogram_area(base:f64,altitude:f64)->f64{base*altitude}
	fn cube_surface_area(s:f64)->f64{6.0*s*s}
	fn cylinder_volume(pi:f64,r:f64,h:f64)->f64{pi*r*r*h}
 fn main() {
 	let mut choice= String::new();
	println!("Welcome to my program calculator");

	println!("1.Trapezium_Area");
	println!("2.Rhombus_Area");
	println!("3.Parallelogram_Area");
	println!("4.cube_surface_Area");
	println!("5.Cylinder_volume");

	println!("\n Enter the shape of your choice");
	io::stdin().read_line(&mut choice).expect("Failed to read input");
	let choice:u32=choice.trim().parse().expect("Invalid number");


		match choice {
			1=> {
				let mut input = String::new();
				println!("Enter height:");
				io::stdin().read_line(&mut input).expect("Failed to read input ");
				let height:f64=input.trim().parse().expect("Invalid number");

				input.clear();
				println!("Enter base1:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let base1:f64=input.trim().parse().expect("Failed to read input");

				input.clear();
				println!("Enter base2:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let base2:f64=input.trim().parse().expect("Failed to read input");
				println!("Trapezium_Area={}",trapezium(height,base1,base2) );
				
			}
			2=> {
				let mut input = String::new();
				println!("Enter diagonal1:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let diagonal1: f64=input.trim().parse().expect("Failed to read input");

				input.clear();
				println!("Enter diagonal2:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let diagonal2: f64=input.trim().parse().expect("Failed to read input");
				println!("Rhombus_area={}",rhombus_area(diagonal1,diagonal2) ); 
			}

				3=> {
					let mut input = String::new();
				println!("Enter base:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let base: f64=input.trim().parse().expect("Failed to read input");

				input.clear();
				println!("Enter altitude:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let altitude: f64=input.trim().parse().expect("Failed to read input");
				println!("Parallelogram_area={}",parallelogram_area(base,altitude) );


				}
				4=> {
				let mut input = String::new();
				println!("enter cube side:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let side: f64=input.trim().parse().expect("Failed to read input");

				println!("cube_surface_area = {}",cube_surface_area(side) );

		

				}
				5=> {
				let mut input = String::new();
				println!("Enter cylinder radius");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let radius: f64=input.trim().parse().expect("Failed to read input");

				input.clear();
				println!("Enter cylinder height:");
				io::stdin().read_line(&mut input).expect("Failed to read input");
				let height: f64=input.trim().parse().expect("Failed to read input");
				println!("cylinder_volume={}",cylinder_volume(std::f64::consts::PI,radius,height, ));

				}
				_ => {
					println!("Invalid choice");
				}

			
		}
}



