 //Rust program for a resturant menu
 use std::io;


fn main() {
	//menu
	println!("Welcome to Jamal's jungle");
	println!("What would you like to have today?");
	println!("Here are a list of all our available items");


	println!("P-Poundo Yam/ Edinkaiko Soup -N3,200");
	println!("F- Fried rice & Chicken -N3,000");
	println!("A- Amala & Ewedu soup - N2,500");
	println!("E- Eba & Egusi Soup - N2,000");
	println!("W- White Rice & Stew - N2,500");

	//Ask for food choice
	println!("\n Enter your food choice(P/F/A/E/W)");

	let mut food = String::new();
	io::stdin().read_line(&mut food).unwrap();
	let food = food.trim();

	//ask for quantity
	println!("\n Enter quantity of your choice");

	let mut quantity_input = String::new();
	io::stdin().read_line(&mut quantity_input).expect("Failed to read input");

	let quantity:i32 = quantity_input.trim().parse().expect("Please enter a valid number");


	//Determine food price
	let price;

	if food == "P"{
		price = 3200;
	}else if food== "F" {
		price = 3000;
	}else if food == "A"{
		price = 2500;
	}else if food == "E"{
		price = 2000;
	}else if food == "W"{
		price = 2500;
	}else {
		println!("Invalid food choice");
		return;
	}

    println!("\nOrder selected");
    println!("Food choice:{}",food );
    println!("Price per item:{}",price);

    let subtotal = price * quantity;
    println!("Quantity: {}", quantity);
    println!("Subtotal: {}",subtotal );

    let mut discount = 0;

if subtotal > 10000 {
    discount = subtotal * 5 / 100;
    println!("\nYou qualify for a 5% discount!");
    println!("Discount: ₦{}", discount);
} else {
    println!("\nNo discount applied.");
}
    let final_total = subtotal-discount;

    println!("Final amount:{}",final_total );
    println!("Thank you for ordering");
    println!("Enjoy your meal and come again");
}
