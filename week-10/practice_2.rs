fn main() {
    let v = vec![10, 20, 30];
    // vector v owns the object in the heap

    let v2 = v; // moves ownership to v2

    display(&v2); // lend v2 instead of giving it away

    println!("In main {:?}", v2); // v2 still works
}

fn display(v: &Vec<i32>) {
    println!("inside display {:?}", v);
}