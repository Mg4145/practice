const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    /*
     * By default variables are immutable
     * To make them mutable use the mut keyword
     */

    println!("{}", fibonacci(10));

    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!(
        "Global constant declared outside of main function {}",
        THREE_HOURS_IN_SECONDS
    );

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
     *      1) Integers Signed and Unsigned
     *      2) Floating Points Signed and Unsigned
     *      3) Booleans
     *      4) Characters
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

    /*
     * These are ways of representing values:
     */

    let decimal = 98_222;
    let hexa = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    println!("Decimal:\t{}", decimal);
    println!("Hexadecimal:\t{}", hexa);
    println!("Octal:\t{}", octal);
    println!("Binary:\t{}", binary);
    println!("Byte:\t{}", byte);

    /*
     * By default, integer values default to i32
     * this can be overwritten by you explicitly telling
     * the machine what type of interger, whether signed
     * or unsigned, your value is.
     * If for some reason you have an unsigned max value,
     * that being 255, and you try to give it a value outside
     * of that range, an integer overflow will occur.
     *
     * This can lead to two behaviors:
     *      1) Rust will panic when you are in debug mode
     *      2) When you are in --release, Rust will not check
     *          and instead will perform a two's complement wrapping
     */

    /*
     * Floating point types are signed by default and set at f64
     */

    let my_float_64 = 2.0; // f64
    let my_float_32: f32 = 3.0; // f32

    println!("This is a 64 bit float:\t{}", my_float_64);
    println!("This is a 32 bit float:\t{}", my_float_32);

    /*
     * Booleans are easy: You either have true or false
     * They are one byte in size
     */

    /*
     * The Character type is the most primitive aplhabetic type
     * They are reprented by using '' and are Unicode Scalar Values
     * They are four bytes in size
     */

    let c = 'z';
    let x = 'ℤ';
    let cat = '😻';

    println!("Here are some characters!\n{}\n{}\n{}", c, x, cat);

    /*
     * Compound Types
     *      Rust has two primitive compound types:
     *          1) Tuples
     *              A tuple without any values is called the unit type and it's
     *              written like (), the value is known as the unit value
     *          2) Arrays
     */

    let my_tuple = (500, 6.4, 1);
    let (x_coordinate, y_coordinate, z_coordinate) = my_tuple;
    println!("({},{},{})", x_coordinate, y_coordinate, z_coordinate);

    let five_hundred = my_tuple.0;
    let six_four = my_tuple.1;
    let one = my_tuple.2;
    println!("({},{},{})", five_hundred, six_four, one);

    /*
     * The Array Data Type
     *      Every element in an array must be of the same data type
     *      They are useful when you your data allocated to the stack
     *      rather than the heap
     *      If you want a data type that is allowed to grow and
     *      shrink in size consider using a vector
     */

    let my_array = [1, 2, 3, 4, 5]; // Array inizialization
    println!("{:?}", my_array);

    /*
     * You can intialize an array to have all the same values by
     */

    let all_same_values = [5; 10]; // The value, the number of elements
    println!("{:?}", all_same_values);
}

fn fibonacci(number: u32) -> u32 {
    match number {
        0 => 0,
        1 => 1,
        _ => fibonacci(number - 1) + fibonacci(number - 2),
    }
}
