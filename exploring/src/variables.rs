fn test() {
    let x = 1.0;
    println!("Size of x in bytes: {}", std::mem::size_of_val(&x));
}