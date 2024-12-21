fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    let z = &mut x;
    *z += 1;
    println!("x = {}", x);
}