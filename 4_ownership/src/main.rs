// declare modules in project root
// declare submodules within those modules with mod *modulename*

// tell compiler to include code found in src/sections.rs
pub mod sections;

fn main() {
    // what_is_ownership::run();
    // sections::references::run();
    sections::slice::run();
}
