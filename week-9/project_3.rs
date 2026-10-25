use std::fs::File;
use::std::io::Write;

fn main() {
    let commissioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zone = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = File::create("merged.txt").expect("create failed");

    
    file.write_all("EFCC Merged File of Convicted Ministers\n".as_bytes()).expect("write failed");
    println!("Commissioner | Ministry | Zone");
    file.write_all("Commissioner | Ministry | Zone\n".as_bytes()).expect("write failed");

    for i in 0..commissioner.len() {

        // display on screen
        println!("{} | {} | {}", commissioner[i], ministry[i], zone[i]);

        
        file.write_all(commissioner[i].as_bytes()).expect("write failed");
	        file.write_all(" | ".as_bytes()).expect("write failed");

	file.write_all(ministry[i].as_bytes()).expect("write failed");
	        file.write_all(" | ".as_bytes()).expect("write failed");

	file.write_all(zone[i].as_bytes()).expect("write failed");
	        file.write_all(" \n".as_bytes()).expect("write failed");

}
	println!("\n Data written to file.");
}