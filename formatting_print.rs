// Print is handled by macros defined in std::fmt

// format! - Write text to string
// print! - Same as format!, but text is printed to console
// println! - newline added
// eprint! - Text is printed as a standard error
// eprintln! - Error with newline

fn main()
{
    print!("I am {}\n", 18);

    println!("Testing positional arguments, {1} comes before {0}", 2, 1);
    
    println!("{first} {second} {third}", first = "I", second = "AM", third = "HERE!");

    print!("Testing conditional, is 1 > 5?: {number:>5}", number = 1);

    eprint!("Testing ERROR print\n");
    eprintln!("Testing ERROR println");
}