fn main() {
    
    // LET declaration 
    
    // can't declare in global scope
    // variables are immutable by default
    let x = 5;
    // x = 6;   this would throw and error

    // make a variable mutable
    // also communicates that this value will be changed by other code
    let mut y = 5;
    y = 6;

    println!("the value of y is {y}");

    // CONSTANTS
    // can't use mut and must annotate the type
    // also must be constant expression, not something that can only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 560 * 3;
    
    println!("three hours in seconds is {}", THREE_HOURS_IN_SECONDS);


    // SHADOWING

    let z = 24;

    let z = z + z; // shadows previous variable we declared

    {
        // shadowing here will only apply within this scope
        // shadowing also lets us mutate the type which can't be done with mut keyword
        let z = "a string"; 
        println!("the value of z in the inner scope is {}", z);
    }

    println!("the value of z is {}", z);




}
