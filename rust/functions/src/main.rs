/**
 * Rust code uses snake_case for writing functions
 * For function signatures, you MUST declare the type of
 * each parameter
 * If the function returns something by default you need not add a
 * semi colon at the end of the function
 */

fn main() {
    another_function();
    fn_with_params(3);
    fn_diff_params(32, "hello world!");
    println!("The 10th fibonacci number is: {}", fibonacci_number(10));

    let y = {
        let x = 3;
        x + 1 // You need not include a ';' when you wish to return a value
    };

    println!("The value of y is {}", y);
    println!(
        "The 10th fibonacci number is: {}",
        fibonacci_number_match(10)
    );
}

fn another_function() {
    println!("Another function.");
}

fn fn_with_params(x: i32) {
    println!("The value of x is: {}", x)
}

fn fn_diff_params(age: i32, name: &str) {
    println!("The value of this function is: {} {}", name, age)
}

fn fibonacci_number(fib: u32) -> u32 {
    if fib == 0 || fib == 1 {
        fib
    } else {
        fibonacci_number(fib - 1) + fibonacci_number(fib - 2)
    }
}

fn fibonacci_number_match(fib: u32) -> u32 {
    match fib {
        0..=1 => fib,
        _ => fibonacci_number_match(fib - 1) + fibonacci_number_match(fib - 2),
    }
}
