Overview
- rust/cargo/setup
- concepts
- syntax specific to rust
- others

macro is like a function but it does things before its run
- eg. `println!` looks at how many params it was given.
As a macro, at the time of generating the binary it looks at the macro and calls it to look at other things first.

documentation
- `cargo doc --open`
- `/// this is for documentation`

playground
- play.rust-lang.org

Types: strongly statically typed
- in dynamically typed, it will only check when code is run
- in rust, types are checked at compile time. you would need to assign the type so the compiler knows

- u32
- i32
- f64
- String
- &str
- bool

- allows for a greater guarantee for safety

-- Type inference, the compiler will infer
- but must specify types at function defs

expressions do not require a semi-colon

exclusive range `0..150` = to 149
inclusive `0..=150`


## Arrays
- for arrays the debug marker is ubvikved,
- otherwise would need `std::fmt`

- {:?} - debug format
- {:#?} = pretty debug

## Iterators:
- filter
- map: can call a funcition on each value in the array
- fold: kinda like map, but results in a single value, takes the type and the accumulator

Lambdas: an anonymous function with a closure available inside of it


## Enums
has variants, specifies variants and when instantiated you choose among the variants
- They are algebraic allowing you to attach types

## Option is an enum that takes some type
- Option<String> : in order to get to the string you need to unwrap.
- eg. you can call expect to try to get the name inside.
