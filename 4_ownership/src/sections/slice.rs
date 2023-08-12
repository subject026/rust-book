/*

Slices let you reference a contiguous sequence of elements in a collection
rather than the whole colleciton. Doesn't have ownership.

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

*/

pub fn run() {
    {
        let mut s = String::from("hello world");

        let word = first_word_without(&s); // gets the value 5

        s.clear(); // replaces value with ""

        // word still has value 5 but isn't any use without string it references
    }

    {
        let mut s = String::from("hello world");
        let word = first_word_with(&s);

        // s.clear(); err - already borrowed as immutable

        println!("{}", word);
    }

    {
        // other slices

        let a = [1, 2, 3, 4, 5];

        // this slice has type &[i32]
        // works same way - stores reference to first element and a length
        let slice = &a[1..3]; // pulls [2, 3] out of array

        assert_eq!(slice, &[2, 3]);
    }
}

/*

function that takes a string of words separated by spaces and returns the first
word it finds in that string. If the function doesn’t find a space in the string,
the whole string must be one word, so the entire string should be returned

*/

//
// without slices

// best we can do return the index at the end of the first word
// and use that to reference the input string
fn first_word_without(s: &String) -> usize {
    // to go through string elemnt by element checking for a space we can convert
    // String into array of bytes
    let bytes = s.as_bytes();

    // next create an iterator over the array of bytes
    // enumerate() wraps result of iter() and returns each element
    // as part of a tuple hence the destructuring that gives the current index
    // and a reference to the current element
    for (i, &item) in bytes.iter().enumerate() {
        // check for the space using the byte literal syntax
        if item == b' ' {
            // if we find a space we return the current position
            return i;
        }
    }

    // otherwise return the final position as it's just one word
    s.len()
}

// return a &str string slice instead
// can still pass reference to String as &str is equivalent to a full slice
// of String
fn first_word_with(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
