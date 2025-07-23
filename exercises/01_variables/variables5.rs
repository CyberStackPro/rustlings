fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    let number = 3; // using variable shadowing
                    // let number = "3".parse::<i32>().unwrap();
    println!("Number plus two is: {}", number + 2);
}
