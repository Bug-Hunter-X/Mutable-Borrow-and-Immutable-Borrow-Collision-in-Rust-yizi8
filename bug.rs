fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &x; // this line causes an error
    println!("x = {}", x);
}