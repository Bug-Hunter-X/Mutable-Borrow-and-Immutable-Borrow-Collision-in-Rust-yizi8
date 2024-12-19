fn main() {
    let mut x = 5;
    {
        let y = &mut x; 
        *y = 10;
    } 
    let z = &x; // this is now fine because y is out of scope 
    println!("x = {}", x);
}