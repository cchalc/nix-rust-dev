fn main() {
    let tup = (500, 6.4, 1);
    let (x, _y, z) = tup;
    let six_point_four = tup.1;
    println!("the value of x is: {x}");
    println!("the value of y is: {six_point_four}");
    println!("the value of z is: {z}");
}
