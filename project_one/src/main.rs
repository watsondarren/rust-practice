fn main() {
    /*
    println!("Hello, world!");
    println!("I am learning Rus.");
    println!("It is awesome!");
    print!("No new line, ");
    print!("Just same line\n");
    */
    
    //comment

    /*
    multi 
    line
    comment
    */

    /*
    let name = "John";
    //Rust uses {} as a placeholder in println!() to show variable values.
    println!("My name is: {}", name);
    let age = 30;
    println!("{} is {} years old", name, age);

    //mut keyword (which means mutable/changeable
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
    */

    /*
    //Rust looks at the value and automatically chooses the right type
    let my_num = 5;         // integer
    let my_double = 5.99;   // float
    let my_letter = 'D';    // character
    let my_bool = true;     // boolean
    let my_text = "Hello";  // string

    //explicitly tell Rust what type a value should be
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // string
    */

    /*
    Basic data types in Rust are divided into different groups:

    Numbers - Whole numbers and decimal numbers (i32, f64)
    Characters - Single letters or symbols (char)
    Strings - Text, a sequence of characters (&str)
    Booleans - True or false values (bool)
    */

    /*
    Constant variables are used to store values that never change.

    Unlike regular variables, constants must be defined with a type (e.g. i32 or char).
    */

    /*
    const BIRTHYEAR: i32 = 1980;
    const MINUTES_PER_HOUR: i32 = 60;
    
    println!("My Birth year is {} and there are {} minutes in an hour", BIRTHYEAR, MINUTES_PER_HOUR);
    */

    /*
    Another thing about constants, is that it is considered good practice to declare them with uppercase.

    It is not required, but useful for code readability and common for Rust programmers:

    Examples:

    MAX_SPEED
    PI
    MINUTES_PER_HOUR
    */

    /*
    Operators are used to perform operations on values and variables.

    Rust supports many common operators, like:

    Arithmetic Operators
    Assignment Operators
    Comparison Operators
    Logical Operators
    */
    
    /*
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);
    */
    
    /*
    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);
    */
    
    /*
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);
    */

    /*
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);
    */

    /*
    let time = 20;
    let greeting = if time < 18 {
    "Good day."
    } else {
    "Good evening."
    };
    println!("{}", greeting);
    */

    /*
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }
    */

    /*
    let day = 6;
    
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }
    */


    /*
    let mut count = 1;

    let result = loop {
    println!("Hello!");

    if count == 3 {
        break count; // Stop the loop and return the number 3
    }

    count += 1;
    };

    println!("The loop stopped at: {}", result);
    */

    /*
    let mut num = 1;

    while num <= 10 {
    if num == 6 {
        num += 1;
        continue;
    }

    println!("Number: {}", num);
    num += 1;
    }
    */

    /*
    for i in 1..6 {
    println!("i is: {}", i);
    }
    */
}
