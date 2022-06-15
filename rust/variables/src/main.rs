const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    /* 
     * By default variables are immutable 
     * To make them mutable use the mut keyword
     */

    let mut x = 5;
    println!("The value of x is {}", x); 
    x = 6;
    println!("The value of x is {}", x);


    println!("Global constant declared outside of main function {}",
             THREE_HOURS_IN_SECONDS);

    /* 
     * Up next we will show  the power of shadowing in action
     * Note that Rust does not have postfix increment
     * Shadowing is different than marking a variable as mut
     * Using let allows us to perform transformations on a value
     * but have the variable remain immutable
     * You will get a compile time error if you do not use let though
     */
    
    let shadow = 5;
    println!("The value of shadow is {}", shadow);

    let shadow = shadow + 1;
    println!("The value of shadow after it being shadowed is: {}", shadow);

    {
        let shadow = shadow * 2;
        println!("The value of shadow in the inner scope is {}", shadow);
    }

    println!("The value of shadow is {}", shadow);
    
    /* 
     * Using let allows us to change the type of the value but also 
     * reuse the same name
     */
    
    let spaces = "        ";
    println!("spaces used to be of type str: {}", spaces); 

    let spaces = spaces.len();
    println!("But now spaces is of type usize with a value of {}", spaces);
    
    /* 
     * Data Types
     *  Scalar: 
     *      1) integers
     *      2) floating points 
     *      3) booleans
     *      4) characters
     */
    
    /*
     * Integer Types
     * | Length | Signed | Unsigned |
     * | 8-bit  |  i8    | u8       |
     * | 16-bit |  i16   | u16      |
     * | 32-bit |  i32   | u32      |
     * | 64-bit |  i64   | u64      |
     * | 128-bit|  i128  | u128     |
     * | arch   |  isize | usize    |
     */
   
    let mut value = 0;
    while value != 10 {
        println!("Let's get Rusty!");
        value += 1;
    }
}
