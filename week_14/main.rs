use std::fs;

fn display_file(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();
    println!("{}", contents);
}

fn administrator() {
    display_file("globacom_db.sql");
}

fn project_manager() {
    display_file("project_tb.sql");
}

fn employee() {
    display_file("staff_tb.sql");
}

fn customer() {
    display_file("customer_tb.sql");
}

fn vendor() {
    display_file("dataplan_tb.sql");
}

fn main() {
    println!("SELECT YOUR ROLE");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut role = String::new();

    std::io::stdin()
        .read_line(&mut role)
        .unwrap();

    match role.trim() {
        "1" => administrator(),
        "2" => project_manager(),
        "3" => employee(),
        "4" => customer(),
        "5" => vendor(),
        _ => println!("Invalid role"),
    }
}