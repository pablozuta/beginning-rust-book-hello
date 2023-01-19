fn main() {
    // the classic hello message
    print!("Hello Rust!");

    // instead of using a single literal string
    // you can print several of them, even in a single
    // statement. In this way:
    println!("{}, {}", "Hello", "Rust");
    println!("{} {} {} ,Here we listen Tangerine Dream", "Welcome", "to", "Bandersnatch");

    // a single statement can print several lines
    print!("First Line\nSecond Line\nThird Line\n");

    // printing Integer Numbers
    println!("My number: 140");

    // using a placeholder
    println!("My numberTwo: {}", "4023");
    println!("Another Amazing Number: {}", "6352");

    // removing the quotes around the second argument
    println!("My numberThree: {}", 162);
}

/* the compiler interprets the string 140 contained in the source code 
as a number expressed in decimal format, it generates the equivalent number 
in binary format, and then it saves it into the executable program.
At runtime, the program takes such number in binary format, it transforms it into
the string "140", using the decimal notation, then it replaces the placeholder with that
string, so generating the string to print, and finally it sends the string to the terminal */
