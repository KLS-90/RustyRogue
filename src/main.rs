fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", 1u32 + 2);
    let pair: (u32, i32) = (12, -6);
    println!("Pair is {:?}", pair);

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", xs);
}

