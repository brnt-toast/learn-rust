fn main(){
    println!("{} days", 31);

    // positional
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // named
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    //format character
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C


    //right-justify
    println!("{number:>5}", number=1);

    //right-padding zeros
    println!("{number:0>5}", number=1); //00001

    //left-adjust by sign flipping
    println!("{number:0<5}", number=1); //10000

    // name arugments
    println!("{number:0>width$}", number=1, width=5);
}