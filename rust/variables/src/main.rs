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
    
}
