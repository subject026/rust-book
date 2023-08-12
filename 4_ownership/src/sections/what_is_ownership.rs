pub fn run() {
    /*

    All data stored in stack memory must have a known, fixed size.

    Data with an unknown size at compile time, or a size that might change
    must be stored on the heap.

    Heap is less organized.

    When your code calls a function, the values passed into the function
    (including, potentially, pointers to data on the heap) and the function’s
    local variables get pushed onto the stack. When the function is over,
    those values get popped off the stack.

    Keeping track of what parts of code are using what data on the heap,
    minimizing the amount of duplicate data on the heap, and cleaning up
    unused data on the heap so you don’t run out of space are all problems
    that ownership addresses.

    Main purpose of ownership is to manage heap data.


    *** Ownership Rules ***

    - each value has an owner
    - can only be one owner at a time
    - when the owner goes out of scope the value is dropped

    */

    {
        // string literal, immutable, hardcoded into program
        // can be stored on the stack
        // str1 is value from this point until end of the current scope
        let str1 = "hello";
    }

    // scope is over so str1 is no longer valid

    {
        // this kind of string can be mutated so is stored on the heap
        let mut str2 = String::from("hello");

        str2.push_str(", ownership!");

        println!("{}", str2);
    }

    // again scope is over so str2 is no longer valid
    // at this point the memory is automatically returned to the allocator

    // rust actuall calls a special function called drop for us
    // its where the author of String can put the code to return the memory

    // -----------------------
    // move

    {
        // both values are pushed onto stack as fixed and known
        // copy of value in x gets bound to y
        let x = 5;
        let y = x;

        // s1 has a pointer pointing to location in memory on heap
        // also has length & capacity values
        let s1 = String::from("hello");
        // this copies the pointer, length and capacity that are on the stack
        // data on the heap is not copied
        let s1 = s1;

        // these pointer variables could go out of scope at different times
        // which would cause problems when drop function is called and tries
        // to free the same memory twice

        // to ensure memory safety rust considers s1 as no longer valid after the line
        // where it is assigned to s2. It's therefore known as a move.

        // s1 was moved into s2
    }

    {
        // clone() is an option but can be expensive as it
        // makes a deep copy including the data on the heap
        let s1 = String::from("hello");
        let mut s2 = s1.clone();

        s2.push_str(", clone()!");

        println!("{}", s2)
    }

    {
        /*
           Similar things happen when passing arguments to functions
        */
        let s = String::from("moving string");
        // s gets moved
        takes_ownership(s);

        // s is now not valid

        let x = 5;
        // x gets copied as it's on the stack
        makes_copy(x);

        // x still valid as
        println!("{}", x);
    }

    {
        /*
           Returning values can also transfer ownership
        */

        // fn moves it's return value into s1
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        // s2 moved into fn which moves return value into s3
        let s3 = takes_and_gives_back(s2);
    }
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
