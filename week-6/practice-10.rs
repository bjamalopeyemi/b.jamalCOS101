fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    let res = (a > 10) && (b > 10);
    println!("a > 10 && b > 10: {}", res);

    let c:i32 = 20;
    let d:i32 = 5;

    let res = (c > 10) || (d > 10);
    println!("c > 10 || d > 10: {}", res);

    let is_elder = false;

    if !is_elder {
        println!("Not Elder");
    }
}