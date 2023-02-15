fn main() {
    let results = add_2_ints(7, 8);
    println!("{}", results);
}

fn add_2_ints(a: i32, b: i32)-> i32{
    let sum = a+b;
    return sum;
}