struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // cost of buying 3 laptops of this brand
    fn cost(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    let hp = Laptop { brand: String::from("HP"), price: 650_000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755_000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550_000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850_000 };

    println!("{}: N{}", hp.brand, hp.cost());
    println!("{}: N{}", ibm.brand, ibm.cost());
    println!("{}: N{}", toshiba.brand, toshiba.cost());
    println!("{}: N{}", dell.brand, dell.cost());

    let total = hp.cost() + ibm.cost() + toshiba.cost() + dell.cost();
    println!("Total cost: N{}", total);
}