fn main() {
    // Testing println!

    println!("{}", 3 + 21);

    println!("{name} {fname}", fname = "shiva", name = "reddy");
    println!("Base 10:{}", 21);
    println!("Base 2:{:b}", 21);

    let x: i32 = 21;
    let y: i32 = 22;
    println!("{:b}", x);
    println!("{:b}", y);
    println!("{:b}", y);

    println!("{0}{1}{2}{3} ", 10, 20, 30, 40);
    println!("My name is {0}, {1} {2}", "James", "Bond", "007");

    let number: f64 = 1.21;
    let width: usize = 5;

    println!("number={number}, width={width}");
    println!("{number:>width$}");
    println!("end");
    
    let flag: bool = true;
    println!("{flag}");
    
    println!("")

}
