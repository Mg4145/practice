/*
 * Conditionals MUST BE A BOOLEAN
 * In other words, this would not be valid
 *      let value = 3;
 *      if value {
 *          println!("This is not valid");
 *      }
 */


fn main() {
    let num = 5;
    if num < 3 {
        println!("This is less than three");
    } else {
        println!("This is greater than three");
    }

    // This will only return the first instance of true!!
    // So "number is divisible by 2" won't print!
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in an let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // This will not execute because of type mismatch!
    //     let number = if condition { 5 } else { "six" };

    /*
     * Repition with loops
     */

    // The first kind of loop is loop by itself
    let mut x = 0;
    loop {
        println!("This will print 3 times!");
        x += 1;
        if x == 3 {
            break;
        }
    }

    txtbk_loop();

    /*
     * Conditional Loops with While
     */

    let mut number = 3;
    while number != 0 {
        print!("{}...", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    /*
     * Looping through a collection using for
     */

    let my_array = [10, 20, 30, 40, 50];

    // Printing elemtns of an array using while-loop
    // This is slower than using a for-loop
    let mut index = 0;
    while index < my_array.len() {
        println!("The value of my_array at index = {} is {}", index, my_array[index]);
        index += 1;
    }

    // Printing elements of an array using for-loop
    // Use for loop since they are faster for this kind of thing
    for element in my_array {
        println!("The value is: {}", element);
    }

    // Countdown in a for loop!
    // (1..4) -> 1 is inclusive and 4 is exclusive
    for number in (1..4).rev() {
        print!("{}...", number);
    }
    println!("LIFTOFF!");
}


fn txtbk_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
