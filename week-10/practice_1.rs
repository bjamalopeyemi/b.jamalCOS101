fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in the heap

    let v2 = v;
    // ownership moves to v2, so v is finished

    println!("{:?}", v2); // v2 is the owner now, so we print v2
}