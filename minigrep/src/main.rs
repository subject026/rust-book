use std::env;
use std::process;

use minigrep::Config;
fn main() {
    // collect args() iterator values into a vector
    let args: Vec<String> = env::args().collect();

    // build() will return a Config or we handle the error and exit the program
    // using unwrap_or_else allows us to do some non panic error handling
    // we're exiting the program here but I guess this is like catching the errors
    // so we could do something else and keep program running

    // i think the | arg| {} function syntax is called a closure and
    // works as an anonymous callback
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("application error! {e}");
        process::exit(1);
    }
}
