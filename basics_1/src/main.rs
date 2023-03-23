fn main() {
    // create a vector of 5 characters

    // iterate and print the 5 characters

    // iterate and remove the characters from the vector

    // iterate and add new characters

    let name = "shiva";
    let size = name.len();
    println!("{size}");

    for num in 1..10 {
        println!("{num}");
    }

    let a = "shiva reddy";

    //*
    for (idx, char) in a.char_indices() {
        println!("{}:{}", idx, char);
    }
    // */

    /*
    for idx in 0..a.len() {
        let item = a.get(idx);
        println!("{:?}", *item);
    }
    // */
}
