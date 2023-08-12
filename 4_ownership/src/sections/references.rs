/*

References save us from variables becoming invalid when passing them into functions.

A reference is like a pointer, an address to follow to access data
owned by some other variable.

Unlike a pointer a reference is guaranteed to point to a valid value of a
particular type for the life of that reference.

We call the action of creating a reference BORROWING

** Mutable References **

allow us to modify values via reference but once created mean no other
references to the original variable can be created

NOTE a reference's scope continues until it's last usage so a

*/

pub fn run() {
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        // can modify with mutable reference
        let mut s2 = String::from("heyy ");

        // pass mutable reference to s2 like so
        change(&mut s2);

        println!("{}", s2);
    }

    {
        let mut s1 = String::from("hello");

        let r1 = &s1;
        let r2 = &s1;

        // can't use these once mut ref is declared below
        println!("{} and {}", r1, r2);

        let r3 = &mut s1;

        println!("{}", r3);

        // println!("{}", r1); // err

        // can create a new immutable reference again here
        let r4 = &s1;
        println!("{}", r4);

        // means r3 again is unusable down here
        // println!("{}", r3);
    }
}

fn calculate_length(s: &String) -> usize {
    // refers to the value stored in s1 but doesn't own it
    // therefore s1 will remain when s inside fn goes out of scope
    s.len()
}

// can pass a mutable reference which allows us to modify a borrowed value
fn change(s: &mut String) {
    s.push_str(" woofles...")
}
