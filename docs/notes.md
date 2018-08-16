<!-- https://learning-rust.github.io/docs/a7.functions.html -->

### Why Rust?
Rust initially designed and developed by Mozilla employee Graydon Hoare as a personal project. Mozilla began sponsoring the project in 2009 and announced it in 2010. But the first stable release, Rust 1.0 released on May 15, 2015.

Rust is very young and very modern language. It’s a compiled programming language and it uses LLVM on its backend. Also Rust is a multi-paradigm programming language, it supports imperative procedural, concurrent actor, object-oriented and pure functional styles. It also supports generic programming and meta programming, in both static and dynamic styles.

Its design elements came from a wide range of sources.

- Abstract Machine Model : C
- Data types : C, SML, OCaml, Lisp, Limbo
- Optional Bindings : Swift
- Hygienic Macros : Scheme
- Functional Programming : Haskell, OCaml, F#
- Attributes : ECMA-335
- Memory Model and Memory Management : C++, ML Kit,
- Cyclone
- Type Classes : Haskell
- Crate : Assembly in the ECMA-335 CLI model
- Channels and Concurrency : Newsqueak, Alef, Limbo
- Message passing and Thread failure : Erlang

Rust compiler observes the code at compiler time and helps to prevent many types of errors that are possible to write in C++

### Hello World
```
fn main() {
    println!("Hello, world!");
}
```
- `fn` means function. main function is the beginning of every Rust program.
- `println!` prints text to the console and its ! indicate that it’s a macro instead of a function.

Execution
- Compile via `rustc file.rs`
- `./file`


### Cargo
- Create a new project: `cargo new`
- Update dependencies: `cargo update`
- Build Project: `cargo run`
- Run tests: `cargo test`
- Generate documentation via rustdoc: `cargo doc`

Other than that there are some cargo commands, especially for publishing creates via cargo.
- Acquire an API token: `cargo login`
- Make the local create uploadable to crates.io: `cargo package`
- Make the local create uploadable to crates.io and upload the crate: `cargo publish`


### Crate
A crate is a package. Crates can be shared via Cargo. A crate can produce an executable o a library. In other words, it can be a binary crate, or a library crate.

eg1. `cargo new crate_name --bin`: produces an executable
```
├── Cargo.toml
└── src
    └── main.rs
```

eg2. `cargo new another_crate_name --lib` OR `cargo new crate_name`: produces a library.
```
├── Cargo.toml
└── src
    └── lib.rs
```

- Cargo.toml - config file which contains all of the metadata that Cargo needs to compile a project.
- src - folder for source code.
- Each crate has an implicit crate root/ entry point. `main.rs` is the crate root for a binary crate. `lib.rs` is the crate root for a library crate.

When building a binary crate via `cargo build` or `cargo run`, the executable filel will be stored in `target/debug/` folder. But when building via `cargo build --release` it will be stored in the `target/release/` folder.

### Project Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── benches
│   └── large-input.rs
├── examples
│   └── simple.rs
├── src
│   ├── bin
│   │   └── another_executable.rs
│   ├── lib.rs
│   └── main.rs
└── tests
    └── some-integration-tests.rs
```

- Source code goes in the `src` directory.
- The default library file is `src/lib.rs`.
- The default executable file is `src/main.rs`.
- Other executables can be placed in `src/bin/*.rs`.
- Integration tests go in the `tests` directory (unit tests go in each file they’re testing).
- Examples go in the `examples` directory.
- Benchmarks go in the `benches` directory.


### Comments and Documentations
```
// Line Comments
/* Block comments */
```
NOTE: Avoid block comments, and use line comments instead.

#### Doc Comments
```
/// Line comments; document the next item
/** Block comments; document the next item */

//! Line comments; document the enclosing item
/*! Block comments; document the enclosing item !*/
```
Doc comments support  Markdown notations. Using `cargo doc`, the HTML documentation can be generated from these doc comments. Let's see the difference between these two sets of doc comments.

```
/// This module contains tests
mod test {
    // ...
}


mod test {
    //! This module contains tests

    // ...
}
```
NOTE: Only use //! to write crate and module-level documentation, nothing else. When using mod blocks, use /// outside of the block.

### Doc Attributes
We can also doc attributes for documenting the code. An attribute is a general, free-form metadatum that is interpreted acording to name, convention, language, and compiler version. Any declataion may have an attribute applied to it.

Below each comment is equivalent to a relevant data attribute/
```
/// Foo
#[doc="Foo"]

//! Foo
#![doc="Foo"]
```

### Variable bindings, Constants, and Statics
In Rust, variables are immutable by default, so we call them `variable bindings`. To make them mutable, the `mut` keyword is used.

Rust is a statically-typed language. It checks the data type at compile time, but does not require you to type it when declaring variable bindings. In that case, the compuler checks the usage, and sets a better data type for it. But for constants and statics you must annotate the type. Types come after a colon.

- **Variable bindings**
    ```
    let a = true;
    let b: bool = true;

    let (x, y) = (1, 2);

    let mut z = 5;
    z = 6;
    ```
    The `let` keyword is used in binding expressions. We can bind a name to a value || function. Also, because the left-hand side of a `let` expression is a "pattern", you can bind multiple names to set of values or function values.

- **Constants**
    ```
    const N: i32 = 5;
    ```
    The `const` keyword is used to define constants. It lives for the entire lifetime of a program, but has no fixed address in memory.


- **Statics**
    ```
    static N: i32 = 5;
    ```
    `static` is used to define a "global variable" type facility. There is only one instance for each value, and it is at a fixed location in memory. Usually statics are placed on top of the code file, outside functions.


NOTE: Always use `const` instead of `static`. It's pretty rare that you actually want a memory location associated with your constant, and using a `const` allows for optimizations like constant propagation not only in your crate, but also in downstream crates.

### Functions
- Functions are declared with the keyword `fn`
- When using arguments, you must declare data types.
- By default, functions return an empty tuple. If you want to return a value, the type must be specified after ->

#### Passing Arguments
```
fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}
```

#### Returning Values
```
fn plus_one(a: i32) -> i32 {
    a + 1 //no ; means an expression, return a+1
}

fn plus_two(a: i32) -> i32 {
    return a + 2; //return a+2 but bad practice,
    //should use only on conditional returnes, except it's last expression
}

// Function pointers, Usage as a Data Type
let b = plus_one;
let c = b(5); //6

let b: fn(i32) -> i32 = plus_one; //same, with type inference
let c = b(5); //6
```

### Primitive Data Types
Each signed variant can store numbers from `-(2n - 1)` to `2n - 1 - 1` inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from `-(27)` to `27 - 1`, which equals `-128` to `127`. Unsigned variants can store numbers from `0` to `2n - 1`, so a u8 can store numbers from `0` to `28 - 1`, which equals `0` to `255`.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64-bits if you’re on a 64-bit architecture and 32-bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table 3-2. Note that all number literals except the byte literal allow a type suffix, such as `57u8`, and `_` as a visual separator, such as `1_000`.

| Number literals | Example     |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good choices, and integer types default to `i32`: it’s generally the fastest, even on 64-bit systems. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size, respectively. The default type is `f64` because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.The `f32` type is a single-precision float, and `f64` has double precision.

- **bool**: true or false
    ```
    let x = true;
    let y: bool = false;
    // no TRUE, FALSE, 1, 0
    ```

- **char**: a single Unicode scalar value
Rust’s char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters, Chinese/Japanese/Korean ideographs, emoji, and zero width spaces are all valid char types in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. We’ll discuss this topic in detail in the “Strings” section in Chapter 8.
    ```
    let x = 'x';
    let y = '😎';

    // only single quotes
    // because of Unicode support, char is not a single byte, but four.
    ```

- **i8, i16, i32, i64**: Fixed size(bit) signed(+/-) integer types

    | DATA TYPE | MIN                  | MAX                 |
    |-----------|----------------------|---------------------|
    | i8        | -128                 | 127                 |
    | i16       | -32768               | 32767               |
    | i32       | -2147483648          | 2147483647          |
    | i64       | -9223372036854775808 | 9223372036854775807 |

    NOTE: min and max values are based on IEEE standard for Binary Floating-Point Arithmetic. From -2ⁿ⁻¹ to 2ⁿ⁻¹-1. You can use `min_value()` and `max_value()` to find the min and max of each integer type. eg. `i9::min_value();`



- **u8, u16, u32, u64**: Fixed size(bit) unsigned(+) integer types

    | DATA TYPE | MIN | MAX                  |
    |-----------|-----|----------------------|
    | u8        | 0   | 255                  |
    | u16       | 0   | 65535                |
    | u32       | 0   | 4294967295           |
    | u64       | 0   | 18446744073709551615 |

    NOTE: Same as signed numbers, min and max values are based on IEEE standard for Binary Floating-Point Arithmetic; From 0 to 2ⁿ-1 . Same way you can use min_value() and max_value() to find min and max of each integer type, ex. u8::max_value();

- **isize**: Variable sized signed(+/-) integer
    Simply this is the data type to cover all signed integer types but memory allocates according to the size of a pointer. Min and max values are similar to i64.

- **usize**: Variable sized unsigned(+) integer
    Simply this is the data type to cover all unsigned integer types but memory allocates according to the size of a pointer. Min and max values are similar to u64.

- **f32**: 32-bit floating point
    Similar to float in other languages, Single precision.
    NOTE: Should avoid using this unless you need to reduce memory consumption badly or if you are doing low-level optimization, when targeted hardware not supports for double-precision or when single-precision is faster than double-precision on it.

- **f64**: 64-bit floating point
    Similar to double in other languages, Double precision

- **arrays** - Fixed size list of elements of same data type

    ```
    let a = [1, 2, 3]; //a[0] = 1
    let mut b = [1, 2, 3];

    let c: [i32; 0] = []; //[Type; # of elements] -> [] /empty array
    let d: [i32; 3] = [1, 2, 3];

    let e = ["my value"; 3]; //["my value", "my value", "my value"];

    println!("{:?}", a); //[1, 2, 3]
    println!("{:#?}", a);
    //  [
    //      1,
    //      2,
    //      3
    //  ]
    ```
    NOTE: Arrays are immutable by defaule. Even with `mut` its element count cannot be changed.

    For a dynamic, growable array, you can use `vec`. Vectors can contain any type of elements, but all elements must be in the same data type.

- **tuples** - Fixed size ordered list of elements of different(or same) data types.
    ```
    let a = (1, 1.5, true, 'a', "Hellow, world!"); // a.0 = 1, a.1 = 1.5, a.2 = true, a.3 = 'a', a.4 = "Hello, world!"

    let b: (i32, f64) = (1, 1.5);

    let (c, d) = b; // c = 1, d = 1.5
    let (e, _, _, _, f) = a; //e = 1, f = "Hello, world!", _ indicates not interested of that item

    let g = (0,); //single-element tuple

    let h = (b, (2, 4), 5); //((1, 1.5), (2, 4), 5)

    println!("{:?}", a); //(1, 1.5, true, 'a', "Hello, world!")
    ```
    NOTE: Tuples are also immutable by default. With `mut` its element count cannot be changed. Also to change an element's value, the new value should have the same data type as the previous value.

- **slice** - Dynamically-sized reference to another data structure
    eg. If you wan tto get or pass part of an array, or any other data structure. Instead of copying it to another array, Rust allows you to create a view/reference to access only that part of the data (mutable or not).
    ```
    let a: [i32; 4] = [1, 2, 3, 4]; //parent array

    let b: &[i32] = &a; //slicing whole array
    let c = &a[0..4]; // from 0th position to 4th(excluding)
    let d = &a[..]; //slicing whole array

    let e = &a[1..3]; //[2, 3]
    let f = &a[1..]; //[2, 3, 4]
    let g = &a[..3]; //[1, 2, 3]
    ```

- **str** - Unsized UTF-8 sequence of Unicode string slices
    ```
    let a = "Hello, world" //aL &'static str
    let b: &str = "こんにちは, 世界!";
    ```
    NOTE: It's an immutable/statically allocated slice holding an unknown sized sequence of UTF-8 code points stored somewhere in memory. `&str` is used to borrow and assign the whole array to the given variable binding.

    A `String` is a heap-allocated string. This string is growable and is also guaranteed to be UTF-8. They are commonly created by converting from a string slice using the to_string() or String::from() methods. eg. `"Hello".to_string();` or `String::from("Hello");`

    In general you should use `String` when you need `ownership`, and `&str` when you just need to borrow a string.

- **functions** - b is a function point to the plus one function.
    ```
    fn plus_one(a: i32) -> i32 {
        a + 1
    }

    let b: fn(i32) -> i32 = plus_one
    let c = b(5); //6
    ```

### Operators
#### Arithmetic Operators
`+ - * / %`

```
let a = 5;
let b = a + 1; //6
let c = a - 1; //4
let d = a * 2; //10
let e = a / 2; // 2 not 2.5
let f = a % 2; //1

let g = 5.0 / 2.0; //2.5

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

NOTE: Also `+` is used for array and string concatenation

#### Comparison Operators
`== != < > <= >=`

```
let a = 1;
let b = 2;

let c = a == b; //false
let d = a != b; //true
let e = a < b; //true
let f = a > b; //false
let g = a <= a; //true
let h = a >= a; //true

// 🔎
let i = true > false; //true
let j = 'a' > 'A'; //true
```

#### Logical Operators
`! && ||`

```
let a = true;
let b = false;

let c = !a; //false
let d = a && b; //false
let e = a || b; //true
🔎 On integer types, ! inverts the individual bits in the two’s complement representation of the value.


let a = !-2; //1
let b = !-1; //0
let c = !0; //-1
let d = !1; //-2

```
#### Bitwise Operators
`& | ^ << >>`

```
let a = 1;
let b = 2;

let c = a & b; //0  (01 && 10 -> 00)
let d = a | b; //3  (01 || 10 -> 11)
let e = a ^ b; //3  (01 != 10 -> 11)
let f = a << b; //4  (add 2 positions to the end -> '01'+'00' -> 100)
let g = a >> a; //0  (remove 2 positions from the end -> o̶1̶ -> 0)
```

#### Assignment and Compound Assignment Operators
The = operator is used to assign a name to a value or a function. Compound Assignment Operators are created by composing one of `+ - * / % & | ^ << >>` operators with `=` operator

Compound types can group multiple values of other types into one type. Rust has two primitive compound types: tuples and arrays.

```
let mut a = 2;

a += 5; //2 + 5 = 7
a -= 2; //7 - 2 = 5
a *= 5; //5 * 5 = 25
a /= 2; //25 / 2 = 12 not 12.5
a %= 5; //12 % 5 = 2

a &= 2; //10 && 10 -> 10 -> 2
a |= 5; //010 || 101 -> 111 -> 7
a ^= 2; //111 != 010 -> 101 -> 5
a <<= 1; //'101'+'0' -> 1010 -> 10
a >>= 2; //101̶0̶ -> 10 -> 2
```

##### Grouping Values into Tuples
A tuple is a general way of grouping together  some number of other values with a variety of types into one compound type.

```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
```
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

In addition to destructuring through pattern matching, we can also access a tuple element directly by using a period (.) followed by the index of the value we want to access. For example:

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
```

##### Arrays
Unlike a tuple, every element of an array must have the same type. Arrays in Rust are different than arrays in some other languages because arrays in Rust have a fixed length: once declared, they cannot grow or shrink in size.

```
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

###### Accessing Array Elements
An array is a single chunk of memory allocated on the stack. We can access elements of an array using indexing, like this:

Filename: src/main.rs

```
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

In this example, the variable named first will get the value 1, because that is the value at index [0] in the array. The variable named second will get the value 2 from index [1] in the array.


#### Type Casting Operator

```
let a = 15;
let b = (a as f64) / 2.0; //7.5
```

Borrowing and Dereference Operators
`& &mut *`

The `&` or `&mut` operators are used for borrowing and * operator for Dereferencing.

NOTE: For more information, refer Ownership, Borrowing & Lifetimes sections.

### Control Flows
**if - else if - else**

```
// Simplest Example
let team_size = 7;
if team_size < 5 {
    println!("Small");
} else if team_size < 10 {
    println!("Medium");
} else {
    println!("Large");
}

// partially refactored code
let team_size = 7;
let team_size_in_text;
if team_size < 5 {
    team_size_in_text = "Small";
} else if team_size < 10 {
    team_size_in_text = "Medium";
} else {
    team_size_in_text = "Large";
}
println!("Current team size : {}", team_size_in_text);

//optimistic code
let team_size = 7;
let team_size_in_text = if team_size < 5 {
    "Small" //no ;
} else if team_size < 10 {
    "Medium"
} else {
    "Large"
};
println!("Current team size : {}", team_size_in_text);


let is_below_eighteen = if team_size < 18 { true } else { false };
```

NOTE: Return data type should be the same on each block, when using this as an expression

##### Using `if` in a `let` statement
Because if is an expression, we can use it on the right side of a let statement, for instance in Listing 3-2:

Filename: src/main.rs
```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```
Listing 3-2: Assigning the result of an if expression to a variable

The number variable will be bound to a value based on the outcome of the if expression. Run this code to see what happens:

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/branches`
The value of number is: 5
```

Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions. In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type; in Listing 3-2, the results of both the if arm and the else arm were i32 integers. If the types are mismatched, as in the following example, we’ll get an error:

Filename: src/main.rs

```
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```
When we try to run this code, we’ll get an error. The if and else arms have value types that are incompatible, and Rust indicates exactly where to find the problem in the program:

```
error[E0308]: if and else have incompatible types
 --> src/main.rs:4:18
  |
4 |       let number = if condition {
  |  __________________^
5 | |         5
6 | |     } else {
7 | |         "six"
8 | |     };
  | |_____^ expected integral variable, found reference
  |
  = note: expected type `{integer}`
             found type `&str`
```
The expression in the if block evaluates to an integer, and the expression in the else block evaluates to a string. This won’t work because variables must have a single type. Rust needs to know at compile time what type the number variable is, definitively, so it can verify at compile time that its type is valid everywhere we use number. Rust wouldn’t be able to do that if the type of number was only determined at runtime; the compiler would be more complex and would make fewer guarantees about the code if it had to keep track of multiple hypothetical types for any variable.

#### match
```
let tshirt_width = 20;
let tshirt_size = match tshirt_width {
    16 => "S", // check 16
    17 | 18 => "M", // check 17 and 18
    19 ... 21 => "L", // check from 19 to 21 (19,20,21)
    22 => "XL",
    _ => "Not Available",
};
println!("{}", tshirt_size); // L


let is_allowed = false;
let list_type = match is_allowed {
    true => "Full",
    false => "Restricted"
    // no default/ _ condition can be skipped
    // Because data type of is_allowed is boolean and all possibilities checked on conditions
};
println!("{}", list_type); // Restricted


let marks_paper_a: u8 = 25;
let marks_paper_b: u8 = 30;
let output = match (marks_paper_a, marks_paper_b) {
    (50, 50) => "Full marks for both papers",
    (50, _) => "Full marks for paper A",
    (_, 50) => "Full marks for paper B",
    (x, y) if x > 25 && y > 25 => "Good",
    (_, _) => "Work hard"
};
println!("{}", output); // Work hard
```

#### while
```
let mut a = 1;
while a <= 10 {
    println!("Current value : {}", a);
    a += 1; //no ++ or -- on Rust
}

// Usage of break and continue
let mut b = 0;
while b < 5 {
    if b == 0 {
        println!("Skip value : {}", b);
        b += 1;
        continue;
    } else if b == 2 {
        println!("Break At : {}", b);
        break;
    }
    println!("Current value : {}", b);
    b += 1;
}

// Outer break
let mut c1 = 1;
'outer_while: while c1 < 6 { //set label outer_while
    let mut c2 = 1;
    'inner_while: while c2 < 6 {
        println!("Current Value : [{}][{}]", c1, c2);
        if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
        c2 += 1;
    }
    c1 += 1;
}
```

#### loop
```
loop {
    println!("Loop forever!");
}

// Usage of break and continue
let mut a = 0;
loop {
    if a == 0 {
        println!("Skip Value : {}", a);
        a += 1;
        continue;
    } else if a == 2 {
        println!("Break At : {}", a);
        break;
    }
    println!("Current Value : {}", a);
    a += 1;
}

// Outer break
let mut b1 = 1;
'outer_loop: loop { //set label outer_loop
  let mut b2 = 1;
  'inner_loop: loop {
    println!("Current Value : [{}][{}]", b1, b2);
    if b1 == 2 && b2 == 2 {
        break 'outer_loop; // kill outer_loop
    } else if b2 == 5 {
        break;
    }
    b2 += 1;
  }
  b1 += 1;
}
```
#### for
```
for a in 0..10 { //(a = o; a <10; a++) // 0 to 10(exclusive)
  println!("Current value : {}", a);
}

// Usage of break and continue
for b in 0..6 {
  if b == 0 {
    println!("Skip Value : {}", b);
    continue;
  } else if b == 2 {
    println!("Break At : {}", b);
    break;
  }
  println!("Current value : {}", b);
}

// Outer break
'outer_for: for c1 in 1..6 { //set label outer_for
  'inner_for: for c2 in 1..6 {
    println!("Current Value : [{}][{}]", c1, c2);
    if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
  }
}


// Working with arrays/vectors
let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

for n in 0..group.len() { //group.len() = 4 -> 0..4 👎 check group.len()on each iteration
  println!("Current Person : {}", group[n]);
}

for person in group.iter() { //👍 group.iter() turn the array into a simple iterator
  println!("Current Person : {}", person);
}

```

### Vectors
An array is a fixed-size list of elements, of same data type. Even with `mut`, its element count can not be changed. A `vector` is kind of a re-sizable array but all elements must be in the same type.

It’s a generic type, written as `Vec<T>` . `T` can have any type, eg. The type of a Vec of i32s is `Vec<i32>`. Also Vectors always allocate their data in dynamically allocated heap.

#### Create empty vector
```
let mut a = Vec::new(); //1.with new() keyword
let mut b = vec![]; //2.using the vec! macro
```

#### Create with data types
```
let mut a2: Vec<i32> = Vec::new();
let mut b2: Vec<i32> = vec![];
let mut b3 = vec![1i32, 2, 3];//sufixing 1st value with data type

let mut b4 = vec![1, 2, 3];
let mut b5: Vec<i32> = vec![1, 2, 3];
let mut b6  = vec![1i32, 2, 3];
let mut b7 = vec![0; 10]; //ten zeroes
```
#### Access and change data
```
//Accessing and changing existing data
let mut c = vec![5, 4, 3, 2, 1];
c[0] = 1;
c[1] = 2;
//c[6] = 2; can't assign values this way, index out of bounds
println!("{:?}", c); //[1, 2, 3, 2, 1]

//push and pop
let mut d: Vec<i32> = Vec::new();
d.push(1); //[1] : Add an element to the end
d.push(2); //[1, 2]
d.pop(); //[1] : : Remove an element from the end


// Capacity and reallocation
let mut e: Vec<i32> = Vec::with_capacity(10);
println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10

// These are all done without reallocating...
for i in 0..10 {
    e.push(i);
}
// ...but this may make the vector reallocate
e.push(11);
```

#### Mainly a vector represent 3 things,

1. a pointer to the data
2. No of elements currently have(length)
3. capacity (Amount of space allocated for any future elements).

If the length of a vector exceeds its capacity, its capacity will be increased automatically. But its elements will be reallocated(which can be slow). So always use `Vec::with_capacity` whenever it’s possible.

String data type is a UTF-8 encoded vector. But you can not index into a String because of encoding.

NOTE: Vectors can be used with iterators in three ways,
```
let mut v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("A reference to {}", i);
}

for i in &mut v {
    println!("A mutable reference to {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}
```
### Structs
Structs are used to encapsulate related properties into one unified datatype. By convention, the name of the struct starts with a capital letter and follows CamelCase.

There are 3 variants of structs
1. C-like structs
    - one or more comma separated name:value pairs
    - brace-enclosed list
    - similar to classes (without its methods) in OOP languages
    - because fields have names, we can access them through dot notation
2. Tuple structs
    - one or more comma separated values
    - parenthesized list like tuples
    - looks like a named tuples
3. Unit structs
    - a struct with no members at all
    - it defines a new type but it resembles an empty tuple, ()
    - rarely in use, useful with generics
NOTE: When regarding OOP in Rust, attributes and methods are placed separately on structs and traits. Structs contain only attributes, traits contain only methods. They are getting connected via impls.

_More complex examples can be found on impls & traits, lifetimes and modules sections._

#### C-like structs
```
// Struct Declaration
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
  // creating an instance
  let black = Color {red: 0, green: 0, blue: 0};

  // accessing its fields using dot notation
  println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); //Black = rgb(0, 0, 0)

  // structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
  let mut link_color = Color {red: 0,green: 0,blue: 255};
  link_color.blue = 238;
  println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)

  // copy elements from another instance
  let blue = Color {blue: 255, .. link_color};
  println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)

  // destructure the instance using a `let` binding, this will not destruct blue instance
  let Color {red: r, green: g, blue: b} = blue;
  println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

  // creating an instance via functions & accessing its fields
  let midnightblue = get_midnightblue_color();
  println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)

  // destructure the instance using a `let` binding
  let Color {red: r, green: g, blue: b} = get_midnightblue_color();
  println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)
}

fn get_midnightblue_color() -> Color {
    Color {red: 25, green: 25, blue: 112}
}
```

#### Tuple structs
When a tuple struct has only one element, we call it new type pattern. Because it helps to create a new type.

```
struct Color (u8, u8, u8);
struct Kilometers(i32);

fn main() {
  // creating an instance
  let black = Color (0, 0, 0);

  // destructure the instance using a `let` binding, this will not destruct black instance
  let Color (r, g, b) = black;
  println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);

  //newtype pattern
  let distance = Kilometers(20);
  // destructure the instance using a `let` binding
  let Kilometers(distance_in_km) = distance;
  println!("The distance: {} km", distance_in_km); //The distance: 20 km
}
```

#### Unit structs
This is rarely useful on its own, but in combination with other features it can become useful.

eg: A library may ask you to create a structure that implements a certain trait to handle events. If you don’t have any data you need to store in the structure, you can create a unit-like struct.

```
struct Electron;

fn main() {
  let x = Electron;
}
```

### Enums
An `enum` is a single type. It contains variants, which are possible values of the enum at a given time.

```
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

// Day is the enum
// Sunday, Monday, etc. are the variants
```

Variants can be accessed through `::notation` eg `Day::Sunday`

Each enum variant can have:
    - No data (unit variant)
    - unnamed ordered data (tuple variant)
    - named data (struct variant)

```
enum FlashMessage {
    Success, //a unit variant
    Warning{ category: i32, message: String }, // a struct variant
    Error(String) // a tuple variant
}

fn main() {
    let mut form_status = FlashMessage::Success;
    print_flash_message(form_status);

    form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
    print_flash_message(form_status);

    form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
    print_flash_message(form_status);

    form_status = FlashMessage::Error(String::from("Connection Error"));
    print_flash_message(form_status);
}

fn print_flash_message(m: FlashMessage) {
    // pattern matching with enum
    match m {
        FlashMessage::Success =>
            println!("Form Submitted correctly"),
        Flash Message::Warning {category, message} => //Destructure, should use same field names
            println!("Warning : {} - {}", category, message),
        FlashMessage::Error(msg) =>
            println!("Error: {}", msg)
    }
}
```

### Generics
Sometimes, when writing a function or data type, we may want it to work for multiple types. In this case we use `generics`

The concept is, instead of declaring a specific data type, we use an uppercase letter

#### Functions
##### Function Parameters
Functions can also be defined to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, we can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation people tend to use the words “parameter” and “argument” interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

The following rewritten version of another_function shows what parameters look like in Rust:

Filename: src/main.rs

```
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

When you want a function to have multiple parameters, separate the parameter declarations with commas, like this:

Filename: src/main.rs

```
fn main() {
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
```

##### Function Statements and Expressions
We’ve actually already used statements and expressions. Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.

Creating a variable and assigning a value to it with the let keyword is a statement. In Listing 3-1, let y = 6; is a statement:

Filename: src/main.rs
```
fn main() {
    let x = (let y = 6);
}
```

When you run this program, the error you’ll get looks like this:

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^
  |
  = note: variable declaration using `let` is a statement
```
**The let y = 6 statement does not return a value**, so there isn’t anything for x to bind to. This is different than in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

Expressions can be part of statements: in Listing 3-1 that had the statement let y = 6;, 6 is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression, for example:

Filename: src/main.rs

```
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

`The value of y is: 4`

This expression:

```
{
    let x = 3;
    x + 1
}
```
is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note the x + 1 line without a semicolon at the end, unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.

##### Return Values
Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an arrow (`->`). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

Filename: src/main.rs

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```
There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified, too, as -> i32. Try running this code; the output should look like this:

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30 secs
     Running `target/debug/functions`
The value of x is: 5
```
The 5 in five is the function’s return value, which is why the return type is i32. Let’s examine this in more detail. There are two important bits: first, the line let x = five(); shows that we’re using the return value of a function to initialize a variable. Because the function five returns a 5, that line is the same as the following:

`let x = 5;`
Second, the five function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return. Let’s look at another example:

Filename: src/main.rs

```
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
Running this code will print The value of x is: 6. What happens if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement? We’ll get an error:

Filename: src/main.rs

```
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```
Running this code produces an error, as follows:

```
error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |   fn plus_one(x: i32) -> i32 {
  |  ____________________________^
8 | |     x + 1;
  | |          - help: consider removing this semicolon
9 | | }
  | |_^ expected i32, found ()
  |
  = note: expected type `i32`
             found type `()`
```
The main error message, “mismatched types,” reveals the core issue with this code. The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the empty tuple. Therefore, nothing is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possibly help rectify this issue: it suggests removing the semicolon, which would fix the error.


### Ownership
The rules for ownership are as follows:
    1. Each value in Rust has a variable that’s called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.

##### Variable Scope
```
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}         
```

##### The String Type
To illustrate the rules of ownership, we need a data type that is more complex than the ones we covered in Chapter 3. The types covered in the “Data Types” section are all stored on the stack and popped off the stack when their scope is over, but we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data.

We’ll use String as the example here and concentrate on the parts of String that relate to ownership. These aspects also apply to other complex data types provided by the standard library and that you create. We’ll discuss String in more depth in Chapter 8.

We’ve already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren’t always suitable for every situation in which you want to use text. One reason is that they’re immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust has a second string type, String. This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a String from a string literal using the from function, like so:

```

let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print `hello, world!`
```
##### Memory and Allocation
With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

The memory must be requested from the operating system at runtime.
We need a way of returning this memory to the operating system when we’re done with our String.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal:

```
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
                                   // longer valid
```

There is a natural point at which we can return the memory our String needs to the operating system: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing }.

To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and therefore, Rust doesn’t need to free anything when s1 goes out of scope. Check out what happens when you try to use s1 after s2 is created, it won’t work:

```
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

You’ll get an error like this because Rust prevents you from using the invalidated reference:

```
error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
```

If you’ve heard the terms “shallow copy” and “deep copy” while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like a shallow copy. But because Rust also invalidates the first variable, instead of calling this a shallow copy, it’s known as a move.

##### Ways Variables and Data Interact: Clone
If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. We’ll discuss method syntax in Chapter 5, but because methods are a common feature in many programming languages, you’ve probably seen them before.

Here’s an example of the clone method in action:

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

This works just fine and is how you can explicitly produce the behavior shown in Figure 4-3, where the heap data does get copied.

When you see a call to clone, you know that some arbitrary code is being executed and that code may be expensive. It’s a visual indicator that something different is going on.

##### Stack-Only Data: Copy
There’s another wrinkle we haven’t talked about yet. This code using integers, part of which was shown earlier in Listing 4-2, works and is valid:

```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

The reason is that types like integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything differently from the usual shallow copying and we can leave it out.

Rust has a special annotation called the `Copy` trait that we can place on types like integers that are stored on the stack (we’ll talk more about traits in Chapter 10). If a type has the `Copy` trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the `Copy` trait if the type, or any of its parts, has implemented the Drop trait. If the type needs something special to happen when the value goes out of scope and we add the `Copy` annotation to that type, we’ll get a compile time error. To learn about how to add the `Copy` annotation to your type, see Appendix C on Derivable Traits.

So what types are `Copy`? You can check the documentation for the given type to be sure, but as a general rule, any group of simple scalar values can be `Copy`, and nothing that requires allocation or is some form of resource is `Copy`.

Here are some of the types that are `Copy`:
- All the integer types, like u32.
- The Boolean type, bool, with values true and false.
- The character type, char.
- All the floating point types, like f64.
- Tuples, but only if they contain types that are also `Copy`. (i32, i32) is `Copy`, but (i32, String) is not.


##### Ownership and Functions
The semantics for passing a value to a function are similar to assigning a value to a variable. Passing a variable to a function will move or copy, just like assignment. Listing 4-3 has an example with some annotations showing where variables go into and out of scope:

Filename: src/main.rs

```
fn main() {
    let s = String::from("hello");  // s comes into scope.

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here.

    let x = 5;                      // x comes into scope.

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward.

} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

=>
hello
5
```

Listing 4-3: Functions with ownership and scope annotated

If we tried to use s after the call to takes_ownership, Rust would throw a compile time error. These static checks protect us from mistakes. Try adding code to main that uses s and x to see where you can use them and where the ownership rules prevent you from doing so.

##### Return Values and Scope
Returning values can also transfer ownership. Here’s an example with similar annotations to those in Listing 4-3:

Filename: src/main.rs

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1.

    let s2 = String::from("hello");     // s2 comes into scope.

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it.

    let some_string = String::from("hello"); // some_string comes into scope.

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function.
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope.

    a_string  // a_string is returned and moves out to the calling function.
}
```
The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

It’s possible to return multiple values using a tuple, like this:

Filename: src/main.rs

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}
```

But this is too much ceremony and a lot of work for a concept that should be common. Luckily for us, Rust has a feature for this concept, and it’s called references.

#### References and Borrowing
The issue with the tuple code at the end of the preceding section is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length.

Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

Filename: src/main.rs

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The length of 'hello' is 5.
First, notice that all the tuple code in the variable declaration and the function return value is gone. Second, note that we pass &s1 into calculate_length, and in its definition, we take &String rather than String.

**Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.**

Let’s take a closer look at the function call here:



let s1 = String::from("hello");

let len = calculate_length(&s1);
The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, the value it points to will not be dropped when the reference goes out of scope.

Likewise, the signature of the function uses & to indicate that the type of the parameter s is a reference. Let’s add some explanatory annotations:

```
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
```

The scope in which the variable s is valid is the same as any function parameter’s scope, but we don’t drop what the reference points to when it goes out of scope because we don’t have ownership. Functions that have references as parameters instead of the actual values mean we won’t need to return the values in order to give back ownership, since we never had ownership.

We call having references as function parameters borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back.

So what happens if we try to modify something we’re borrowing? Try the code in Listing 4-4. Spoiler alert: it doesn’t work!

Filename: src/main.rs

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

Listing 4-4: Attempting to modify a borrowed value

Here’s the error:

```
error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
 --> error.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- use `&mut String` here to make mutable
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^ cannot borrow as mutable
```

Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.

##### Mutable References
We can fix the error in the code from Listing 4-4 with just a small tweak:

Filename: src/main.rs


fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
First, we had to change s to be mut. Then we had to create a mutable reference with &mut s and accept a mutable reference with some_string: &mut String.

But mutable references have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope. This code will fail:

Filename: src/main.rs

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Here’s the error:

```
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> borrow_twice.rs:5:19
  |
4 |     let r1 = &mut s;
  |                   - first mutable borrow occurs here
5 |     let r2 = &mut s;
  |                   ^ second mutable borrow occurs here
6 | }
  | - first borrow ends here
```
This restriction allows for mutation but in a very controlled fashion. It’s something that new Rustaceans struggle with, because most languages let you mutate whenever you’d like. The benefit of having this restriction is that Rust can prevent data races at compile time.

A data race is similar to a race condition and happens when these three behaviors occur:

Two or more pointers access the same data at the same time.
At least one of the pointers is being used to write to the data.
There’s no mechanism being used to synchronize access to the data.
Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem from happening because it won’t even compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:

```
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
A similar rule exists for combining mutable and immutable references. This code results in an error:


let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```

Here’s the error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as
immutable
 --> borrow_thrice.rs:6:19
  |
4 |     let r1 = &s; // no problem
  |               - immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |                   ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

Whew! We also cannot have a mutable reference while we have an immutable one. Users of an immutable reference don’t expect the values to suddenly change out from under them! However, multiple immutable references are okay because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

Even though these errors may be frustrating at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is instead of you having to track down why sometimes your data isn’t what you thought it should be.


##### Dangling References
In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer that references a location in memory that may have been given to someone else, by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if we have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let’s try to create a dangling reference, which Rust will prevent with a compile-time error:

Filename: src/main.rs

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```
Here’s the error:

```
error[E0106]: missing lifetime specifier
 --> dangle.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is
  no value for it to be borrowed from
  = help: consider giving it a 'static lifetime
```
This error message refers to a feature we haven’t covered yet: lifetimes. We’ll discuss lifetimes in detail in Chapter 10. But, if you disregard the parts about lifetimes, the message does contain the key to why this code is a problem:


this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
Let’s take a closer look at exactly what’s happening at each stage of our dangle code:

```
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```

Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String! That’s no good. Rust won’t let us do this.

The solution here is to return the String directly:

```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

This works without any problems. Ownership is moved out, and nothing is deallocated.

#### The Rules of References
Let’s recap what we’ve discussed about references:

1. At any given time, you can have either but not both of:
    - One mutable reference.
    - Any number of immutable references.
2. References must always be valid.
Next, we’ll look at a different kind of reference: slices.

#### Slices
Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

Here’s a small programming problem: write a function that takes a string and returns the first word it finds in that string. If the function doesn’t find a space in the string, it means the whole string is one word, so the entire string should be returned.

Let’s think about the signature of this function:

`fn first_word(s: &String) -> ?`

This function, first_word, has a &String as a parameter. We don’t want ownership, so this is fine. But what should we return? We don’t really have a way to talk about part of a string. However, we could return the index of the end of the word. Let’s try that as shown in Listing 4-5:

Filename: src/main.rs

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

Listing 4-5: The first_word function that returns a byte index value into the String parameter

Let’s break down this code a bit. Because we need to go through the String element by element and check whether a value is a space, we’ll convert our String to an array of bytes using the as_bytes method:


`let bytes = s.as_bytes();`
Next, we create an iterator over the array of bytes using the iter method:


`for (i, &item) in bytes.iter().enumerate() {`
We’ll discuss iterators in more detail in Chapter 13. For now, know that iter is a method that returns each element in a collection, and enumerate wraps the result of iter and returns each element as part of a tuple instead. The first element of the returned tuple is the index, and the second element is a reference to the element. This is a bit more convenient than calculating the index ourselves.

Because the enumerate method returns a tuple, we can use patterns to destructure that tuple, just like everywhere else in Rust. So in the for loop, we specify a pattern that has i for the index in the tuple and &item for the single byte in the tuple. Because we get a reference to the element from .iter().enumerate(), we use & in the pattern.

We search for the byte that represents the space by using the byte literal syntax. If we find a space, we return the position. Otherwise, we return the length of the string by using s.len():

```
    if item == b' ' {
        return i;
    }
}
s.len()
```
We now have a way to find out the index of the end of the first word in the string, but there’s a problem. We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String. In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future. Consider the program in Listing 4-6 that uses the first_word function from Listing 4-5:

Filename: src/main.rs

```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5.

    s.clear(); // This empties the String, making it equal to "".

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

Listing 4-6: Storing the result from calling the first_word function then changing the String contents

This program compiles without any errors and also would if we used word after calling s.clear(). word isn’t connected to the state of s at all, so word still contains the value 5. We could use that value 5 with the variable s to try to extract the first word out, but this would be a bug because the contents of s have changed since we saved 5 in word.

Having to worry about the index in word getting out of sync with the data in s is tedious and error prone! Managing these indices is even more brittle if we write a second_word function. Its signature would have to look like this:


fn second_word(s: &String) -> (usize, usize) {
Now we’re tracking a start and an ending index, and we have even more values that were calculated from data in a particular state but aren’t tied to that state at all. We now have three unrelated variables floating around that need to be kept in sync.

Luckily, Rust has a solution to this problem: string slices.

##### String Slices
A string slice is a reference to part of a String, and looks like this:

```
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

This is similar to taking a reference to the whole String but with the extra [0..5] bit. Rather than a reference to the entire String, it’s a reference to a portion of the String. The start..end syntax is a range that begins at start and continues up to, but not including, end.

We can create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position included in the slice and ending_index is one more than the last position included in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index. So in the case of let world = &s[6..11];, world would be a slice that contains a pointer to the 6th byte of s and a length value of 5.

Figure 4-6 shows this in a diagram.

world containing a pointer to the 6th byte of String s and a length 5

Figure 4-6: String slice referring to part of a String

With Rust’s .. range syntax, if you want to start at the first index (zero), you can drop the value before the two periods. In other words, these are equal:

```
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

By the same token, if your slice includes the last byte of the String, you can drop the trailing number. That means these are equal:

```
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```
You can also drop both values to take a slice of the entire string. So these are equal:

```
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error. For the purposes of introducing string slices, we are assuming ASCII only in this section; a more thorough discussion of UTF-8 handling is in the “Strings” section of Chapter 8.

With all this information in mind, let’s rewrite first_word to return a slice. The type that signifies “string slice” is written as &str:

Filename: src/main.rs

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

We get the index for the end of the word in the same way as we did in Listing 4-5, by looking for the first occurrence of a space. When we find a space, we return a string slice using the start of the string and the index of the space as the starting and ending indices.

Now when we call first_word, we get back a single value that is tied to the underlying data. The value is made up of a reference to the starting point of the slice and the number of elements in the slice.

Returning a slice would also work for a second_word function:

`fn second_word(s: &String) -> &str {`
We now have a straightforward API that’s much harder to mess up, since the compiler will ensure the references into the String remain valid. Remember the bug in the program in Listing 4-6, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of first_word will throw a compile time error:

Filename: src/main.rs

```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // Error!
}
```
Here’s the compiler error:

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                            - immutable borrow occurs here
5 |
6 |     s.clear(); // Error!
  |     ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```
Recall from the borrowing rules that if we have an immutable reference to something, we cannot also take a mutable reference. Because clear needs to truncate the String, it tries to take a mutable reference, which fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

String Literals Are Slices
Recall that we talked about string literals being stored inside the binary. Now that we know about slices, we can properly understand string literals:


`let s = "Hello, world!";`
The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

##### String Slices as Parameters
Knowing that you can take slices of literals and Strings leads us to one more improvement on first_word, and that’s its signature:


`fn first_word(s: &String) -> &str {`
A more experienced Rustacean would write the following line instead because it allows us to use the same function on both Strings and &strs:


`fn first_word(s: &str) -> &str {`
If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String. Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

Filename: src/main.rs

```
fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

##### Other Slices
String slices, as you might imagine, are specific to strings. But there’s a more general slice type, too. Consider this array:



`let a = [1, 2, 3, 4, 5];`
Just like we might want to refer to a part of a string, we might want to refer to part of an array and would do so like this:

`let a = [1, 2, 3, 4, 5];`

`let slice = &a[1..3];`
This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length. You’ll use this kind of slice for all sorts of other collections. We’ll discuss these collections in detail when we talk about vectors in Chapter 8.

#### Summary
The concepts of ownership, borrowing, and slices are what ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage like other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to the next chapter and look at grouping pieces of data together in a struct.


### Structs
#### Defining and Instantiating Structs
Structs are similar to tuples, which were discussed in Chapter 3. Like tuples, the pieces of a struct can be different types. Unlike tuples, we name each piece of data so it’s clear what the values mean. As a result of these names, structs are more flexible than tuples: we don’t have to rely on the order of the data to specify or access the values of an instance.

To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields. For example, Listing 5-1 shows a struct to store information about a user account:

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Listing 5-1: A User struct definition

To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct, and then add curly brackets containing key: value pairs where the keys are the names of the fields and the values are the data we want to store in those fields. We don’t have to specify the fields in the same order in which we declared them in the struct. In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type. For example, we can declare a particular user as shown in Listing 5-2:

```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

Listing 5-2: Creating an instance of the User struct

To get a specific value from a struct, we can use dot notation. If we wanted just this user’s email address, we can use user1.email wherever we want to use this value. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field. Listing 5-3 shows how to change the value in the email field of a mutable User instance:

```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

Listing 5-3: Changing the value in the email field of a User instance

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. Also note that as with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

Listing 5-4 shows a build_user function that returns a User instance with the given email and username. The active field gets the value of true, and the sign_in_count gets a value of 1.

```
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

Listing 5-4: A build_user function that takes an email and username and returns a User instance

It makes sense to name the function arguments with the same name as the struct fields, but having to repeat the email and username field names and variables is a bit tedious. If the struct had more fields, repeating each name would get even more annoying. Luckily, there’s a convenient shorthand!

#### Using the Field Init Shorthand when Variables and Fields Have the Same Name
Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of email and username in the way shown in Listing 5-5.

```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

Listing 5-5: A build_user function that uses field init shorthand since the email and username parameters have the same name as struct fields

Here, we’re creating a new instance of the User struct, which has a field named email. We want to set the email field’s value to the value in the email parameter of the build_user function. Because the email field and the email parameter have the same name, we only need to write email rather than email: email.

#### Creating Instances From Other Instances With Struct Update Syntax
It’s often useful to create a new instance of a struct that uses most of an old instance’s values, but changes some. We do this using struct update syntax.

First, Listing 5-6 shows how we create a new User instance in user2 without the update syntax. We set new values for email and username, but otherwise use the same values from user1 that we created in Listing 5-2:

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

Listing 5-6: Creating a new User instance using some of the values from user1

Using struct update syntax, we can achieve the same effect with less code, as shown in Listing 5-7. The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
Listing 5-7: Using struct update syntax to set new email and username values for a User instance but use the rest of the values from the fields of the instance in the user1 variable

The code in Listing 5-7 also creates an instance in user2 that has a different value for email and username but has the same values for the active and sign_in_count fields from user1.

#### Tuple Structs without Named Fields to Create Different Types
We can also define structs that look similar to tuples (which were discussed in Chapter 3), called tuple structs, that have the added meaning the struct name provides, but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple be a different type than other tuples, but naming each field as in a regular struct would be verbose or redundant.

To define a tuple struct you start with the struct keyword and the struct name followed by the types in the tuple. For example, here are definitions and usages of two tuple structs named Color and Point:

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Note that the black and origin values are different types, since they’re instances of different tuple structs. Each struct we define is its own type, even though the fields within the struct have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances behave like tuples: you can destructure them into their individual pieces and you can use a . followed by the index to access an individual value, and so on.

####Unit-Like Structs without Any Fields
We can also define structs that don’t have any fields! These are called unit-like structs since they behave similarly to (), the unit type. Unit-like structs can be useful in situations such as when you need to implement a trait on some type, but you don’t have any data that you want to store in the type itself. We’ll discuss traits in Chapter 10.

#### Ownership of Struct Data
In the User struct definition in Listing 5-1, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes, a Rust feature that we’ll discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is. Let’s say you try to store a reference in a struct without specifying lifetimes, like this:

Filename: src/main.rs

```
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

The compiler will complain that it needs lifetime specifiers:

```
error[E0106]: missing lifetime specifier
 -->
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 -->
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```
In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, we’ll fix errors like these using owned types like String instead of references like &str.

#### An Example Program Using Structs
To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle. We’ll start with single variables, and then refactor the program until we’re using structs instead.

Let’s make a new binary project with Cargo called rectangles that will take the width and height of a rectangle specified in pixels and will calculate the area of the rectangle. Listing 5-8 shows a short program with one way of doing just that in our project’s src/main.rs:

Filename: src/main.rs

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

The area of the rectangle is 1500 square pixels.
Listing 5-8: Calculating the area of a rectangle specified by its width and height in separate variables

Now, run this program using cargo run:

The area of the rectangle is 1500 square pixels.
Refactoring with Tuples
Even though Listing 5-8 works and figures out the area of the rectangle by calling the area function with each dimension, we can do better. The width and the height are related to each other because together they describe one rectangle.

The issue with this code is evident in the signature of area:

`fn area(width: u32, height: u32) -> u32 {`
The area function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters. The parameters are related, but that’s not expressed anywhere in our program. It would be more readable and more manageable to group width and height together. We’ve already discussed one way we might do that in the “Grouping Values into Tuples” section of Chapter 3: by using tuples. Listing 5-9 shows another version of our program that uses tuples:

Filename: src/main.rs

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

Listing 5-9: Specifying the width and height of the rectangle with a tuple

In one way, this program is better. Tuples let us add a bit of structure, and we’re now passing just one argument. But in another way this version is less clear: tuples don’t name their elements, so our calculation has become more confusing because we have to index into the parts of the tuple.

It doesn’t matter if we mix up width and height for the area calculation, but if we want to draw the rectangle on the screen, it would matter! We would have to keep in mind that width is the tuple index 0 and height is the tuple index 1. If someone else worked on this code, they would have to figure this out and keep it in mind as well. It would be easy to forget or mix up these values and cause errors, because we haven’t conveyed the meaning of our data in our code.

Refactoring with Structs: Adding More Meaning
We use structs to add meaning by labeling the data. We can transform the tuple we’re using into a data type with a name for the whole as well as names for the parts, as shown in Listing 5-10:

Filename: src/main.rs

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

The area of the rectangle is 1500 square pixels.
Listing 5-10: Defining a Rectangle struct

Here we’ve defined a struct and named it Rectangle. Inside the {} we defined the fields as width and height, both of which have type u32. Then in main we create a particular instance of a Rectangle that has a width of 30 and a height of 50.

Our area function is now defined with one parameter, which we’ve named rectangle, whose type is an immutable borrow of a struct Rectangle instance. As mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, main retains its ownership and can continue using rect1, which is the reason we use the & in the function signature and where we call the function.

The area function accesses the width and height fields of the Rectangle instance. Our function signature for area now says exactly what we mean: calculate the area of a Rectangle, using its width and height fields. This conveys that the width and height are related to each other, and gives descriptive names to the values rather than using the tuple index values of 0 and 1. This is a win for clarity.

#### Adding Useful Functionality with Derived Traits
It’d be nice to be able to print out an instance of our Rectangle while we’re debugging our program and see the values for all its fields. Listing 5-11 tries the println! macro as we have used it in Chapters 2, 3, and 4:

Filename: src/main.rs

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {}", rect1);
}
```

Listing 5-11: Attempting to print a Rectangle instance

When we run this code, we get an error with this core message:


error[E0277]: the trait bound `Rectangle: std::fmt::Display` is not satisfied
The println! macro can do many kinds of formatting, and by default, {} tells println! to use formatting known as `Display`: output intended for direct end user consumption. The primitive types we’ve seen so far implement `Display` by default, because there’s only one way you’d want to show a 1 or any other primitive type to a user. But with structs, the way println! should format the output is less clear because there are more display possibilities: do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn’t try to guess what we want and structs don’t have a provided implementation of `Display`.

If we continue reading the errors, we’ll find this helpful note:


`Rectangle` cannot be formatted with the default formatter; try using
`:?` instead if you are using a format string
Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the {} tells println! we want to use an output format called Debug. Debug is a trait that enables us to print out our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Run the code with this change. Drat! We still get an error:


error[E0277]: the trait bound `Rectangle: std::fmt::Debug` is not satisfied
But again, the compiler gives us a helpful note:


`Rectangle` cannot be formatted using `:?`; if it is defined in your
crate, add `#[derive(Debug)]` or manually implement it
Rust does include functionality to print out debugging information, but we have to explicitly opt-in to make that functionality available for our struct. To do that, we add the annotation #[derive(Debug)] just before the struct definition, as shown in Listing 5-12:

Filename: src/main.rs

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
rect1 is Rectangle { width: 30, height: 50 }
```

Listing 5-12: Adding the annotation to derive the Debug trait and printing the Rectangle instance using debug formatting

Now when we run the program, we won’t get any errors and we’ll see the following output:

`rect1 is Rectangle { width: 30, height: 50 }`
Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string. When we use the {:#?} style in the example, the output will look like this:

```
rect1 is Rectangle {
    width: 30,
    height: 50
}
```
Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types. Those traits and their behaviors are listed in Appendix C. We’ll cover how to implement these traits with custom behavior as well as how to create your own traits in Chapter 10.

Our area function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.

#### Method Syntax
Methods are similar to functions: they’re declared with the fn keyword and their name, they can have parameters and a return value, and they contain some code that is run when they’re called from somewhere else. However, methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

Defining Methods
Let’s change the area function that has a Rectangle instance as a parameter and instead make an area method defined on the Rectangle struct, as shown in Listing 5-13:

Filename: src/main.rs

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

The area of the rectangle is 1500 square pixels.
Listing 5-13: Defining an area method on the Rectangle struct

To define the function within the context of Rectangle, we start an impl (implementation) block. Then we move the area function within the impl curly brackets and change the first (and in this case, only) parameter to be self in the signature and everywhere within the body. In main where we called the area function and passed rect1 as an argument, we can instead use method syntax to call the area method on our Rectangle instance. The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

In the signature for area, we use &self instead of rectangle: &Rectangle because Rust knows the type of self is Rectangle due to this method being inside the impl Rectangle context. Note that we still need to use the & before self, just like we did in &Rectangle. Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just like any other parameter.

We’ve chosen &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and we want to prevent the caller from using the original instance after the transformation.

The main benefit of using methods instead of functions, in addition to using method syntax and not having to repeat the type of self in every method’s signature, is for organization. We’ve put all the things we can do with an instance of a type in one impl block rather than making future users of our code search for capabilities of Rectangle in various places in the library we provide.

##### Where’s the -> Operator?
In languages like C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to `(*object).something().`

Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in Rust that has this behavior.

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

```
p1.distance(&p2);
(&p1).distance(&p2);
```

The first one looks much cleaner. This automatic referencing behavior works because methods have a clear receiver—the type of self. Given the receiver and name of a method, Rust can figure out definitively whether the method is reading (&self), mutating (&mut self), or consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.

Methods with More Parameters
Let’s practice using methods by implementing a second method on the Rectangle struct. This time, we want an instance of Rectangle to take another instance of Rectangle and return true if the second Rectangle can fit completely within self; otherwise it should return false. That is, we want to be able to write the program shown in Listing 5-14, once we’ve defined the can_hold method:

Filename: src/main.rs

```
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Listing 5-14: Demonstration of using the as-yet-unwritten can_hold method

And the expected output would look like the following, because both dimensions of rect2 are smaller than the dimensions of rect1, but rect3 is wider than rect1:

`Can rect1 hold rect2? true
Can rect1 hold rect3? false`
We know we want to define a method, so it will be within the impl Rectangle block. The method name will be can_hold, and it will take an immutable borrow of another Rectangle as a parameter. We can tell what the type of the parameter will be by looking at the code that calls the method: rect1.can_hold(&rect2) passes in &rect2, which is an immutable borrow to rect2, an instance of Rectangle. This makes sense because we only need to read rect2 (rather than write, which would mean we’d need a mutable borrow), and we want main to retain ownership of rect2 so we can use it again after calling the can_hold method. The return value of can_hold will be a Boolean, and the implementation will check whether the width and height of self are both greater than the width and height of the other Rectangle, respectively. Let’s add the new can_hold method to the impl block from Listing 5-13, shown in Listing 5-15:

Filename: src/main.rs

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Listing 5-15: Implementing the can_hold method on Rectangle that takes another Rectangle instance as a parameter

When we run this code with the main function in Listing 5-14, we’ll get our desired output. Methods can take multiple parameters that we add to the signature after the self parameter, and those parameters work just like parameters in functions.

##### Associated Functions
Another useful feature of impl blocks is that we’re allowed to define functions within impl blocks that don’t take self as a parameter. These are called associated functions because they’re associated with the struct. They’re still functions, not methods, because they don’t have an instance of the struct to work with. You’ve already used the String::from associated function.

Associated functions are often used for constructors that will return a new instance of the struct. For example, we could provide an associated function that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:

Filename: src/main.rs

```
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

To call this associated function, we use the :: syntax with the struct name, like let sq = Rectangle::square(3);, for example. This function is namespaced by the struct: the :: syntax is used for both associated functions and namespaces created by modules, which we’ll discuss in Chapter 7.

Multiple impl Blocks
Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block:

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Listing 5-16: Rewriting Listing 5-15 using multiple impl blocks

There’s no reason to separate these methods into multiple impl blocks here, but it’s valid syntax. We will see a case when multiple impl blocks are useful in Chapter 10 when we discuss generic types and traits.

#### Summary
Structs let us create custom types that are meaningful for our domain. By using structs, we can keep associated pieces of data connected to each other and name each piece to make our code clear. Methods let us specify the behavior that instances of our structs have, and associated functions let us namespace functionality that is particular to our struct without having an instance available.

But structs aren’t the only way we can create custom types: let’s turn to Rust’s enum feature to add another tool to our toolbox.

### Defining an Enum
Let’s look at a situation we might want to express in code and see why enums are useful and more appropriate than structs in this case. Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and version six. These are the only possibilities for an IP address that our program will come across: we can enumerate all possible values, which is where enumeration gets its name.

Any IP address can be either a version four or a version six address but not both at the same time. That property of IP addresses makes the enum data structure appropriate for this case, because enum values can only be one of the variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and V6. These are known as the variants of the enum:


```
enum IpAddrKind {
    V4,
    V6,
}
```

`IpAddrKind` is now a custom data type that we can use elsewhere in our code.

#### Enum Values
We can create instances of each of the two variants of IpAddrKind like this:


```
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two. The reason this is useful is that now both values `IpAddrKind::V4` and `IpAddrKind::V6` are of the same type: `IpAddrKind`.

We can then, for instance, define a function that takes any `IpAddrKind`:

`fn route(ip_type: IpAddrKind) { }`
And we can call this function with either variant:

```
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

Using enums has even more advantages. Thinking more about our IP address type, at the moment we don’t have a way to store the actual IP address data; we only know what kind it is. Given that you just learned about structs in Chapter 5, you might tackle this problem as shown in Listing 6-1:

```
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

Listing 6-1: Storing the data and IpAddrKind variant of an IP address using a struct

Here, we’ve defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind (the enum we defined previously) and an address field of type String. We have two instances of this struct. The first, home, has the value IpAddrKind::V4 as its kind with associated address data of 127.0.0.1. The second instance, loopback, has the other variant of IpAddrKind as its kind value, V6, and has address ::1 associated with it. We’ve used a struct to bundle the kind and address values together, so now the variant is associated with the value.

We can represent the same concept in a more concise way using just an enum, rather than an enum inside a struct, by putting data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have associated String values:

```
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

We attach data to each variant of the enum directly, so there is no need for an extra struct.

There’s another advantage to using an enum rather than a struct: each variant can have different types and amounts of associated data. Version four type IP addresses will always have four numeric components that will have values between 0 and 255. If we wanted to store V4 addresses as four u8 values but still express V6 addresses as one String value, we wouldn’t be able to with a struct. Enums handle this case with ease:


```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

We’ve shown several different possibilities that we could define in our code for storing IP addresses of the two different varieties using an enum. However, as it turns out, wanting to store IP addresses and encode which kind they are is so common that the standard library has a definition we can use! Let’s look at how the standard library defines IpAddr: it has the exact enum and variants that we’ve defined and used, but it embeds the address data inside the variants in the form of two different structs, which are defined differently for each variant:


```
struct Ipv4Addr {
    // details elided
}

struct Ipv6Addr {
    // details elided
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum! Also, standard library types are often not much more complicated than what you might come up with.

Note that even though the standard library contains a definition for IpAddr, we can still create and use our own definition without conflict because we haven’t brought the standard library’s definition into our scope. We’ll talk more about bringing types into scope in Chapter 7.

Let’s look at another example of an enum in Listing 6-2: this one has a wide variety of types embedded in its variants:


```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```


Listing 6-2: A Message enum whose variants each store different amounts and types of values

This enum has four variants with different types:

Quit has no data associated with it at all.
Move includes an anonymous struct inside it.
Write includes a single String.
ChangeColor includes three i32 values.
Defining an enum with variants like the ones in Listing 6-2 is similar to defining different kinds of struct definitions except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:


```
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

But if we used the different structs, which each have their own type, we wouldn’t be able to as easily define a function that could take any of these kinds of messages as we could with the Message enum defined in Listing 6-2, which is a single type.

There is one more similarity between enums and structs: just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:


```
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

The body of the method would use self to get the value that we called the method on. In this example, we’ve created a variable m that has the value Message::Write(String::from("hello")), and that is what self will be in the body of the call method when `m.call()` runs.

Let’s look at another enum in the standard library that is very common and useful: Option.

##### The Option Enum and Its Advantages Over Null Values
In the previous section, we looked at how the IpAddr enum let us use Rust’s type system to encode more information than just the data into our program. This section explores a case study of Option, which is another enum defined by the standard library. The Option type is used in many places because it encodes the very common scenario in which a value could be something or it could be nothing. Expressing this concept in terms of the type system means the compiler can check that you’ve handled all the cases you should be handling, which can prevent bugs that are extremely common in other programming languages.

Programming language design is often thought of in terms of which features you include, but the features you exclude are important too. Rust doesn’t have the null feature that many other languages have. Null is a value that means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

In “Null References: The Billion Dollar Mistake,” Tony Hoare, the inventor of null, has this to say:

I call it my billion-dollar mistake. At that time, I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe, with checking performed automatically by the compiler. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.

The problem with null values is that if you try to actually use a value that’s null as if it is a not-null value, you’ll get an error of some kind. Because this null or not-null property is pervasive, it’s extremely easy to make this kind of error.

However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

The problem isn’t with the actual concept but with the particular implementation. As such, Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>`, and it is defined by the standard library as follows:


```
enum `Option<T>` {
    Some(T),
    None,
}
```

The `Option<T>` enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. In addition, so are its variants: you can use Some and None directly without prefixing them with Option::. `Option<T>` is still just a regular enum, and Some(T) and None are still variants of type `Option<T>`.

The `<T>` syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that <T> means the Some variant of the Option enum can hold one piece of data of any type. Here are some examples of using Option values to hold number types and string types:


```
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

If we use None rather than Some, we need to tell Rust what type of `Option<T>` we have, because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.

When we have a Some value, we know that a value is present, and the value is held within the Some. When we have a None value, in some sense, it means the same thing as null: we don’t have a valid value. So why is having `Option<T>` any better than having null?

In short, because `Option<T>` and T (where T can be any type) are different types, the compiler won’t let us use an `Option<T>` value as if it was definitely a valid value. For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>:

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

If we run this code, we get an error message like this:

```
error[E0277]: the trait bound i8: std::ops::Add<std::option::Option<i8>> is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for i8 + std::option::Option<i8>
  |
```

Intense! In effect, this error message means that Rust doesn’t understand how to add an i8 and an Option<i8>, because they’re different types. When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value.

In other words, you have to convert an `Option<T>` to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

Not having to worry about missing an assumption of having a not-null value helps you to be more confident in your code. In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an `Option<T>`, you can safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

So, how do you get the T value out of a Some variant when you have a value of type `Option<T>` so you can use that value? The `Option<T>` enum has a large number of methods that are useful in a variety of situations; you can check them out in its documentation. Becoming familiar with the methods on `Option<T>` will be extremely useful in your journey with Rust.

In general, in order to use an `Option<T>` value, we want to have code that will handle each variant. We want some code that will run only when we have a Some(T) value, and this code is allowed to use the inner T. We want some other code to run if we have a None value, and that code doesn’t have a T value available. The match expression is a control flow construct that does just this when used with enums: it will run different code depending on which variant of the enum it has, and that code can use the data inside the matching value.


#### The match Control Flow Operator
Rust has an extremely powerful control-flow operator called match that allows us to compare a value against a series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal values, variable names, wildcards, and many other things; Chapter 18 covers all the different kinds of patterns and what they do. The power of match comes from the expressiveness of the patterns and the compiler checks that all possible cases are handled.

Think of a match expression kind of like a coin sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into. In the same way, values go through each pattern in a match, and at the first pattern the value “fits,” the value will fall into the associated code block to be used during execution.

Because we just mentioned coins, let’s use them as an example using match! We can write a function that can take an unknown United States coin and, in a similar way as the counting machine, determine which coin it is and return its value in cents, as shown here in Listing 6-3:

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Listing 6-3: An enum and a match expression that has the variants of the enum as its patterns.

Let’s break down the match in the value_in_cents function. First, we list the match keyword followed by an expression, which in this case is the value coin. This seems very similar to an expression used with if, but there’s a big difference: with if, the expression needs to return a Boolean value. Here, it can be any type. The type of coin in this example is the Coin enum that we defined in Listing 6-3.

Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run. The code in this case is just the value 1. Each arm is separated from the next with a comma.

When the match expression executes, it compares the resulting value against the pattern of each arm, in order. If a pattern matches the value, the code associated with that pattern is executed. If that pattern doesn’t match the value, execution continues to the next arm, much like a coin sorting machine. We can have as many arms as we need: in Listing 6-3, our match has four arms.

The code associated with each arm is an expression, and the resulting value of the expression in the matching arm is the value that gets returned for the entire match expression.

Curly brackets typically aren’t used if the match arm code is short, as it is in Listing 6-3 where each arm just returns a value. If you want to run multiple lines of code in a match arm, you can use curly brackets. For example, the following code would print out “Lucky penny!” every time the method was called with a Coin::Penny but would still return the last value of the block, 1:


```
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Patterns that Bind to Values
Another useful feature of match arms is that they can bind to parts of the values that match the pattern. This is how we can extract values out of enum variants.

As an example, let’s change one of our enum variants to hold data inside it. From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value. We can add this information to our enum by changing the Quarter variant to include a UsState value stored inside it, which we’ve done here in Listing 6-4:


```
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

Listing 6-4: A Coin enum where the Quarter variant also holds a UsState value

Let’s imagine that a friend of ours is trying to collect all 50 state quarters. While we sort our loose change by coin type, we’ll also call out the name of the state associated with each quarter so if it’s one our friend doesn’t have, they can add it to their collection.

In the match expression for this code, we add a variable called state to the pattern that matches values of the variant Coin::Quarter. When a Coin::Quarter matches, the state variable will bind to the value of that quarter’s state. Then we can use state in the code for that arm, like so:

```
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

If we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be Coin::Quarter(UsState::Alaska). When we compare that value with each of the match arms, none of them match until we reach Coin::Quarter(state). At that point, the binding for state will be the value UsState::Alaska. We can then use that binding in the println! expression, thus getting the inner state value out of the Coin enum variant for Quarter.

Matching with Option<T>
In the previous section we wanted to get the inner T value out of the Some case when using Option<T>; we can also handle Option<T> using match as we did with the Coin enum! Instead of comparing coins, we’ll compare the variants of Option<T>, but the way that the match expression works remains the same.

Let’s say we want to write a function that takes an Option<i32>, and if there’s a value inside, adds one to that value. If there isn’t a value inside, the function should return the None value and not attempt to perform any operations.

This function is very easy to write, thanks to match, and will look like Listing 6-5:


```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Listing 6-5: A function that uses a match expression on an Option<i32>

Matching Some(T)
Let’s examine the first execution of plus_one in more detail. When we call plus_one(five), the variable x in the body of plus_one will have the value Some(5). We then compare that against each match arm.


None => None,
The Some(5) value doesn’t match the pattern None, so we continue to the next arm.


`Some(i) => Some(i + 1),`
Does Some(5) match Some(i)? Well yes it does! We have the same variant. The i binds to the value contained in Some, so i takes the value 5. The code in the match arm is then executed, so we add one to the value of i and create a new Some value with our total 6 inside.

##### Matching None
Now let’s consider the second call of plus_one in Listing 6-5 where x is None. We enter the match and compare to the first arm.


`None => None,`
It matches! There’s no value to add to, so the program stops and returns the None value on the right side of =>. Because the first arm matched, no other arms are compared.

Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

##### Matches Are Exhaustive
There’s one other aspect of match we need to discuss. Consider this version of our plus_one function:

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We didn’t handle the None case, so this code will cause a bug. Luckily, it’s a bug Rust knows how to catch. If we try to compile this code, we’ll get this error:

```
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

Rust knows that we didn’t cover every possible case and even knows which pattern we forgot! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making the billion dollar mistake discussed earlier.

The _ Placeholder
Rust also has a pattern we can use in situations when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255. If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the special pattern _ instead:

```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

The _ pattern will match any value. By putting it after our other arms, the _ will match all the possible cases that aren’t specified before it. The () is just the unit value, so nothing will happen in the _ case. As a result, we can say that we want to do nothing for all the possible values that we don’t list before the _ placeholder.

However, the match expression can be a bit wordy in a situation in which we only care about one of the cases. For this situation, Rust provides if let.


#### Concise Control Flow with if let
The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern and ignore the rest. Consider the program in Listing 6-6 that matches on an Option<u8> value but only wants to execute code if the value is three:

```
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

Listing 6-6: A match that only cares about executing code when the value is Some(3)

We want to do something with the Some(3) match but do nothing with any other Some<u8> value or the None value. To satisfy the match expression, we have to add _ => () after processing just one variant, which is a lot of boilerplate code to add.

Instead, we could write this in a shorter way using if let. The following code behaves the same as the match in Listing 6-6:


```
if let Some(3) = some_u8_value {
    println!("three");
}
```

if let takes a pattern and an expression separated by an =. It works the same way as a match, where the expression is given to the match and the pattern is its first arm.

Using if let means you have less to type, less indentation, and less boilerplate code. However, we’ve lost the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and if gaining conciseness is an appropriate trade-off for losing exhaustive checking.

In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.

We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else. Recall the Coin enum definition in Listing 6-4, where the Quarter variant also held a UsState value. If we wanted to count all non-quarter coins we see while also announcing the state of the quarters, we could do that with a match expression like this:


```
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
```

Or we could use an if let and else expression like this:


```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.

#### Summary
We’ve now covered how to use enums to create custom types that can be one of a set of enumerated values. We’ve shown how the standard library’s Option<T> type helps you use the type system to prevent errors. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

Your Rust programs can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensures type safety: the compiler will make certain your functions only get values of the type each function expects.

In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules.


### Using Modules to Reuse and Organize Code
When you start writing programs in Rust, your code might live solely in the main function. As your code grows, you’ll eventually move functionality into other functions for reuse and better organization. By splitting your code into smaller chunks, each chunk is easier to understand on its own. But what happens if you have too many functions? Rust has a module system that enables the reuse of code in an organized fashion.

In the same way that you extract lines of code into a function, you can extract functions (and other code, like structs and enums) into different modules. A module is a namespace that contains definitions of functions or types, and you can choose whether those definitions are visible outside their module (public) or not (private). Here’s an overview of how modules work:

The mod keyword declares a new module. Code within the module appears either immediately following this declaration within curly brackets or in another file.
By default, functions, types, constants, and modules are private. The pub keyword makes an item public and therefore visible outside its namespace.
The use keyword brings modules, or the definitions inside modules, into scope so it’s easier to refer to them.
We’ll look at each of these parts to see how they fit into the whole.


#### `mod` and Filesystem

We'll start our module example by making a new project with Cargo, but instead of creating a binary create, we'll make a library crate: a project that other people can pull into their projects as a dependency. For example, the `rand` crate discussed in Chapter 2 is a library crate that we used as a dependency in the guessing game project.

We'll create a skeleton of a library that provides some general networking functionality; we'll concentrate on the organization of the modules and functions but we won't worry about what code goes in the function bodies. We'll call our library communicator. By default, Cargo will create a library unless another type of project is specified: if we omit the --bin option that we've been using in all of the chapters preceding this ne, our project will be a libraru:
```
$ cargo new communicator
$ cd communicator
```
Notice that Cargo generated srx/lib.rs instead of src/main.rs. Inside src/lib.rs we'll find the following:

Filename: src/lib.rs

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}
```

Cargo creates an example test to help us get our library started, rather than the “Hello, world!” binary that we get when we use the --bin option. We’ll look at the #[] and mod tests syntax in the “Using super to Access a Parent Module” section later in this chapter, but for now, leave this code at the bottom of src/lib.rs.

Because we don’t have a src/main.rs file, there’s nothing for Cargo to execute with the cargo run command. Therefore, we’ll use the cargo build command to compile our library crate’s code.

We’ll look at different options for organizing your library’s code that will be suitable in a variety of situations, depending on the intent of the code.

Module Definitions
For our communicator networking library, we’ll first define a module named network that contains the definition of a function called connect. Every module definition in Rust starts with the mod keyword. Add this code to the beginning of the src/lib.rs file, above the test code:

Filename: src/lib.rs


```
mod network {
    fn connect() {
    }
}
```


After the mod keyword, we put the name of the module, network, and then a block of code in curly brackets. Everything inside this block is inside the namespace network. In this case, we have a single function, connect. If we wanted to call this function from code outside the network module, we would need to specify the module and use the namespace syntax ::, like so: network::connect() rather than just connect().

We can also have multiple modules, side by side, in the same src/lib.rs file. For example, to also have a client module that has a function named connect as well, we can add it as shown in Listing 7-1:

Filename: src/lib.rs


```
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```


Listing 7-1: The network module and the client module defined side by side in src/lib.rs

Now we have a network::connect function and a client::connect function. These can have completely different functionality, and the function names do not conflict with each other because they’re in different modules.

In this case, because we’re building a library, the file that serves as the entry point for building our library is src/lib.rs. However, in respect to creating modules, there’s nothing special about src/lib.rs. We could also create modules in src/main.rs for a binary crate in the same way as we’re creating modules in src/lib.rs for the library crate. In fact, we can put modules inside of modules, which can be useful as your modules grow to keep related functionality organized together and separate functionality apart. The choice of how you organize your code depends on how you think about the relationship between the parts of your code. For instance, the client code and its connect function might make more sense to users of our library if they were inside the network namespace instead, as in Listing 7-2:

Filename: src/lib.rs


```
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```

Listing 7-2: Moving the client module inside the network module

In your src/lib.rs file, replace the existing mod network and mod client definitions with the ones in Listing 7-2, which have the client module as an inner module of network. Now we have the functions network::connect and network::client::connect: again, the two functions named connect don’t conflict with each other because they’re in different namespaces.

In this way, modules form a hierarchy. The contents of src/lib.rs are at the topmost level, and the submodules are at lower levels. Here’s what the organization of our example in Listing 7-1 looks like when thought of as a hierarchy:

```
communicator
 ├── network
 └── client
```

And here’s the hierarchy corresponding to the example in Listing 7-2:

```
communicator
 └── network
     └── client
```

The hierarchy shows that in Listing 7-2, client is a child of the network module rather than a sibling. More complicated projects can have many modules, and they’ll need to be organized logically in order to keep track of them. What “logically” means in your project is up to you and depends on how you and your library’s users think about your project’s domain. Use the techniques shown here to create side-by-side modules and nested modules in whatever structure you would like.

Moving Modules to Other Files
Modules form a hierarchical structure, much like another structure in computing that you’re used to: filesystems! We can use Rust’s module system along with multiple files to split up Rust projects so not everything lives in src/lib.rs or src/main.rs. For this example, let’s start with the code in Listing 7-3:

Filename: src/lib.rs


```
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```


Listing 7-3: Three modules, client, network, and network::server, all defined in src/lib.rs

The file src/lib.rs has this module hierarchy:

```
communicator
 ├── client
 └── network
     └── server
```

If these modules had many functions, and those functions were becoming lengthy, it would be difficult to scroll through this file to find the code we wanted to work with. Because the functions are nested inside one or more mod blocks, the lines of code inside the functions will start getting lengthy as well. These would be good reasons to separate the client, network, and server modules from src/lib.rs and place them into their own files.

First, replace the client module code with only the declaration of the client module, so that your src/lib.rs looks like code shown in Listing 7-4:

Filename: src/lib.rs

```
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```


Listing 7-4: Extracting the contents of the client module but leaving the declaration in src/lib.rs

We’re still declaring the client module here, but by replacing the block with a semicolon, we’re telling Rust to look in another location for the code defined within the scope of the client module. In other words, the line mod client; means:

```
mod client {
    // contents of client.rs
}
```

Now we need to create the external file with that module name. Create a client.rs file in your src/ directory and open it. Then enter the following, which is the connect function in the client module that we removed in the previous step:

Filename: src/client.rs


```
fn connect() {
}
```

Note that we don’t need a mod declaration in this file because we already declared the client module with mod in `src/lib.rs`. This file just provides the contents of the client module. If we put a mod client here, we’d be giving the client module its own submodule named client!

Rust only knows to look in src/lib.rs by default. If we want to add more files to our project, we need to tell Rust in `src/lib.rs` to look in other files; this is why mod client needs to be defined in src/lib.rs and can’t be defined in src/client.rs.

Now the project should compile successfully, although you’ll get a few warnings. Remember to use cargo build instead of cargo run because we have a library crate rather than a binary crate:

```
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
warning: function is never used: `connect`
 --> src/client.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/lib.rs:4:5
  |
4 | /     fn connect() {
5 | |     }
  | |_____^

warning: function is never used: `connect`
 --> src/lib.rs:8:9
  |
8 | /         fn connect() {
9 | |         }
  | |_________^
```

These warnings tell us that we have functions that are never used. Don’t worry about these warnings for now; we’ll address them later in this chapter in the “Controlling Visibility with pub” section. The good news is that they’re just warnings; our project built successfully!

Next, let’s extract the network module into its own file using the same pattern. In src/lib.rs, delete the body of the network module and add a semicolon to the declaration, like so:

Filename: src/lib.rs

```
mod client;

mod network;
```


Then create a new `src/network.rs` file and enter the following:

Filename: src/network.rs


```
fn connect() {
}

mod server {
    fn connect() {
    }
}
```


Notice that we still have a mod declaration within this module file; this is because we still want server to be a submodule of network.

Run cargo build again. Success! We have one more module to extract: server. Because it’s a submodule—that is, a module within a module—our current tactic of extracting a module into a file named after that module won’t work. We’ll try anyway so you can see the error. First, change src/network.rs to have mod server; instead of the server module’s contents:

Filename: src/network.rs

```
fn connect() {
}

mod server;
```

Then create a src/server.rs file and enter the contents of the server module that we extracted:

Filename: src/server.rs


```
fn connect() {
}
```

When we try to cargo build, we’ll get the error shown in Listing 7-5:

```
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `src/network.rs` to its own directory via `src/network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

Listing 7-5: Error when trying to extract the server submodule into src/server.rs

The error says we cannot declare a new module at this location and is pointing to the mod server; line in src/network.rs. So src/network.rs is different than src/lib.rs somehow: keep reading to understand why.

The note in the middle of Listing 7-5 is actually very helpful because it points out something we haven’t yet talked about doing:


note: maybe move this module `network` to its own directory via
`network/mod.rs`
Instead of continuing to follow the same file naming pattern we used previously, we can do what the note suggests:

Make a new directory named network, the parent module’s name.
Move the src/network.rs file into the new network directory, and rename it to src/network/mod.rs.
Move the submodule file src/server.rs into the network directory.
Here are commands to carry out these steps:

```
$ mkdir src/network
$ mv src/network.rs src/network/mod.rs
$ mv src/server.rs src/network
```

Now when we try to run cargo build, compilation will work (we’ll still have warnings though). Our module layout still looks like this, which is exactly the same as it did when we had all the code in src/lib.rs in Listing 7-3:

```
communicator
 ├── client
 └── network
     └── server
```

The corresponding file layout now looks like this:

```
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```


So when we wanted to extract the network::server module, why did we have to also change the src/network.rs file to the src/network/mod.rs file and put the code for network::server in the network directory in src/network/server.rs instead of just being able to extract the network::server module into src/server.rs? The reason is that Rust wouldn’t be able to recognize that server was supposed to be a submodule of network if the server.rs file was in the src directory. To clarify Rust’s behavior here, let’s consider a different example with the following module hierarchy, where all the definitions are in src/lib.rs:

```
communicator
 ├── client
 └── network
     └── client
```
In this example, we have three modules again: client, network, and network::client. Following the same steps we did earlier for extracting modules into files, we would create src/client.rs for the client module. For the network module, we would create src/network.rs. But we wouldn’t be able to extract the network::client module into a src/client.rs file because that already exists for the top-level client module! If we could put the code for both the client and network::client modules in the src/client.rs file, Rust wouldn’t have any way to know whether the code was for client or for network::client.

Therefore, in order to extract a file for the network::client submodule of the network module, we needed to create a directory for the network module instead of a src/network.rs file. The code that is in the network module then goes into the src/network/mod.rs file, and the submodule network::client can have its own src/network/client.rs file. Now the top-level src/client.rs is unambiguously the code that belongs to the client module.

##### Rules of Module Filesystems
Let’s summarize the rules of modules with regard to files:

If a module named foo has no submodules, you should put the declarations for foo in a file named foo.rs.
If a module named foo does have submodules, you should put the declarations for foo in a file named foo/mod.rs.
These rules apply recursively, so if a module named foo has a submodule named bar and bar does not have submodules, you should have the following files in your src directory:

```
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```

The modules should be declared in their parent module’s file using the mod keyword.

Next, we’ll talk about the pub keyword and get rid of those warnings!

#### Controlling Visibility with pub
We resolved the error messages shown in Listing 7-5 by moving the network and network::server code into the src/network/mod.rs and src/network/server.rs files, respectively. At that point, cargo build was able to build our project, but we still get warning messages about the client::connect, network::connect, and network::server::connect functions not being used:

```
warning: function is never used: `connect`
 --> src/client.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/network/mod.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^

warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
```

So why are we receiving these warnings? After all, we’re building a library with functions that are intended to be used by our users, not necessarily by us within our own project, so it shouldn’t matter that these connect functions go unused. The point of creating them is that they will be used by another project, not our own.

To understand why this program invokes these warnings, let’s try using the connect library from another project, calling it externally. To do that, we’ll create a binary crate in the same directory as our library crate by making a src/main.rs file containing this code:

Filename: src/main.rs

```
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```

We use the extern crate command to bring the communicator library crate into scope. Our package now contains two crates. Cargo treats src/main.rs as the root file of a binary crate, which is separate from the existing library crate whose root file is src/lib.rs. This pattern is quite common for executable projects: most functionality is in a library crate, and the binary crate uses that library crate. As a result, other programs can also use the library crate, and it’s a nice separation of concerns.

From the point of view of a crate outside the communicator library looking in, all the modules we’ve been creating are within a module that has the same name as the crate, communicator. We call the top-level module of a crate the root module.

Also note that even if we’re using an external crate within a submodule of our project, the extern crate should go in our root module (so in src/main.rs or src/lib.rs). Then, in our submodules, we can refer to items from external crates as if the items are top-level modules.

Right now, our binary crate just calls our library’s connect function from the client module. However, invoking cargo build will now give us an error after the warnings:

```
error[E0603]: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Ah ha! This error tells us that the client module is private, which is the crux of the warnings. It’s also the first time we’ve run into the concepts of public and private in the context of Rust. The default state of all code in Rust is private: no one else is allowed to use the code. If you don’t use a private function within your program, because your program is the only code allowed to use that function, Rust will warn you that the function has gone unused.

After we specify that a function like client::connect is public, not only will our call to that function from our binary crate be allowed, but the warning that the function is unused will go away. Marking a function as public lets Rust know that the function will be used by code outside of our program. Rust considers the theoretical external usage that’s now possible as the function “being used.” Thus, when a function is marked public, Rust will not require that it be used in our program and will stop warning that the function is unused.

Making a Function Public
To tell Rust to make a function public, we add the pub keyword to the start of the declaration. We’ll focus on fixing the warning that indicates client::connect has gone unused for now, as well as the module `client` is private error from our binary crate. Modify src/lib.rs to make the client module public, like so:

Filename: src/lib.rs

```
pub mod client;

mod network;
The pub keyword is placed right before mod. Let’s try building again:


error[E0603]: function `connect` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Hooray! We have a different error! Yes, different error messages are a cause for celebration. The new error shows function `connect` is private, so let’s edit src/client.rs to make client::connect public too:

Filename: src/client.rs


```
pub fn connect() {
}
Now run cargo build again:


warning: function is never used: `connect`
 --> src/network/mod.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
```

The code compiled, and the warning about client::connect not being used is gone!

Unused code warnings don’t always indicate that an item in your code needs to be made public: if you didn’t want these functions to be part of your public API, unused code warnings could be alerting you to code you no longer need that you can safely delete. They could also be alerting you to a bug if you had just accidentally removed all places within your library where this function is called.

But in this case, we do want the other two functions to be part of our crate’s public API, so let’s mark them as pub as well to get rid of the remaining warnings. Modify src/network/mod.rs to look like the following:

Filename: src/network/mod.rs
```

pub fn connect() {
}

mod server;
Then compile the code:


warning: function is never used: `connect`
 --> src/network/mod.rs:1:1
  |
1 | / pub fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default

warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
```
Hmmm, we’re still getting an unused function warning, even though network::connect is set to pub. The reason is that the function is public within the module, but the network module that the function resides in is not public. We’re working from the interior of the library out this time, whereas with client::connect we worked from the outside in. We need to change src/lib.rs to make network public too, like so:

Filename: src/lib.rs

```
pub mod client;

pub mod network;
```

Now when we compile, that warning is gone:

```
warning: function is never used: `connect`
 --> src/network/server.rs:1:1
  |
1 | / fn connect() {
2 | | }
  | |_^
  |
  = note: #[warn(dead_code)] on by default
```

Only one warning is left! Try to fix this one on your own!

##### Privacy Rules
Overall, these are the rules for item visibility:

If an item is public, it can be accessed through any of its parent modules.
If an item is private, it can be accessed only by its immediate parent module and any of the parent’s child modules.
Privacy Examples
Let’s look at a few more privacy examples to get some practice. Create a new library project and enter the code in Listing 7-6 into your new project’s src/lib.rs:

Filename: src/lib.rs

```
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    outermost::middle_secret_function();
    outermost::inside::inner_function();
    outermost::inside::secret_function();
}
```
Listing 7-6: Examples of private and public functions, some of which are incorrect

Before you try to compile this code, make a guess about which lines in the try_me function will have errors. Then, try compiling the code to see whether you were right, and read on for the discussion of the errors!

Looking at the Errors
The try_me function is in the root module of our project. The module named outermost is private, but the second privacy rule states that the try_me function is allowed to access the outermost module because outermost is in the current (root) module, as is try_me.

The call to outermost::middle_function will work because middle_function is public, and try_me is accessing middle_function through its parent module outermost. We determined in the previous paragraph that this module is accessible.

The call to outermost::middle_secret_function will cause a compilation error. middle_secret_function is private, so the second rule applies. The root module is neither the current module of middle_secret_function (outermost is), nor is it a child module of the current module of middle_secret_function.

The module named inside is private and has no child modules, so it can only be accessed by its current module outermost. That means the try_me function is not allowed to call outermost::inside::inner_function or outermost::inside::secret_function.

##### Fixing the Errors
Here are some suggestions for changing the code in an attempt to fix the errors. Before you try each one, make a guess as to whether it will fix the errors, and then compile the code to see whether or not you’re right, using the privacy rules to understand why.

- What if the inside module was public?
- What if outermost was public and inside was private?
- What if, in the body of inner_function, you called ::outermost::middle_secret_function()? (The two colons at the beginning mean that we want to refer to the modules starting from the root module.)
- Feel free to design more experiments and try them out!

Next, let’s talk about bringing items into scope with the use keyword.


#### Referring to Names in Different Modules
We’ve covered how to call functions defined within a module using the module name as part of the call, as in the call to the nested_modules function shown here in Listing 7-7:

Filename: src/main.rs

```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

fn main() {
    a::series::of::nested_modules();
}
```

Listing 7-7: Calling a function by fully specifying its enclosing module’s path

As you can see, referring to the fully qualified name can get quite lengthy. Fortunately, Rust has a keyword to make these calls more concise.

Bringing Names into Scope with the use Keyword
Rust’s use keyword shortens lengthy function calls by bringing the modules of the function you want to call into scope. Here’s an example of bringing the a::series::of module into a binary crate’s root scope:

Filename: src/main.rs

```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```

The line use a::series::of; means that rather than using the full a::series::of path wherever we want to refer to the of module, we can use of.

The use keyword brings only what we’ve specified into scope: it does not bring children of modules into scope. That’s why we still have to use of::nested_modules when we want to call the nested_modules function.

We could have chosen to bring the function into scope by instead specifying the function in the use as follows:

```
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

Doing so allows us to exclude all the modules and reference the function directly.

Because enums also form a sort of namespace like modules, we can bring an enum’s variants into scope with use as well. For any kind of use statement, if you’re bringing multiple items from one namespace into scope, you can list them using curly brackets and commas in the last position, like so:

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```


We’re still specifying the TrafficLight namespace for the Green variant because we didn’t include Green in the use statement.

Bringing All Names into Scope with a Glob
To bring all the items in a namespace into scope at once, we can use the * syntax, which is called the glob operator. This example brings all the variants of an enum into scope without having to list each specifically:

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```

The * will bring into scope all the visible items in the TrafficLight namespace. You should use globs sparingly: they are convenient, but this might also pull in more items than you expected and cause naming conflicts.

Using super to Access a Parent Module
As we saw at the beginning of this chapter, when you create a library crate, Cargo makes a tests module for you. Let’s go into more detail about that now. In your communicator project, open src/lib.rs:

Filename: src/lib.rs

```
pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

Chapter 11 explains more about testing, but parts of this example should make sense now: we have a module named tests that lives next to our other modules and contains one function named it_works. Even though there are special annotations, the tests module is just another module! So our module hierarchy looks like this:

```
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```

Tests are for exercising the code within our library, so let’s try to call our client::connect function from this it_works function, even though we won’t be checking any functionality right now. This won’t work yet:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```


Run the tests by invoking the cargo test command:

```
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
error[E0433]: failed to resolve. Use of undeclared type or module `client`
 --> src/lib.rs:9:9
  |
9 |         client::connect();
  |         ^^^^^^ Use of undeclared type or module `client`
```

The compilation failed, but why? We don’t need to place communicator:: in front of the function like we did in src/main.rs because we are definitely within the communicator library crate here. The reason is that paths are always relative to the current module, which here is tests. The only exception is in a use statement, where paths are relative to the crate root by default. Our tests module needs the client module in its scope!

So how do we get back up one module in the module hierarchy to call the client::connect function in the tests module? In the tests module, we can either use leading colons to let Rust know that we want to start from the root and list the whole path, like this:


`::client::connect();`
Or, we can use super to move up one module in the hierarchy from our current module, like this:


`super::client::connect();`
These two options don’t look that different in this example, but if you’re deeper in a module hierarchy, starting from the root every time would make your code lengthy. In those cases, using super to get from the current module to sibling modules is a good shortcut. Plus, if you’ve specified the path from the root in many places in your code and then you rearrange your modules by moving a subtree to another place, you’d end up needing to update the path in several places, which would be tedious.

It would also be annoying to have to type super:: in each test, but you’ve already seen the tool for that solution: use! The super:: functionality changes the path you give to use so it is relative to the parent module instead of to the root module.

For these reasons, in the tests module especially, use super::something is usually the best solution. So now our test looks like this:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```

When we run cargo test again, the test will pass and the first part of the test result output will be the following:

```
$ cargo test
   Compiling communicator v0.1.0 (file:///projects/communicator)
     Running target/debug/communicator-92007ddb5330fa5a

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### Summary
Now you know some new techniques for organizing your code! Use these techniques to group related functionality together, keep files from becoming too long, and present a tidy public API to your library users.

Next, we’ll look at some collection data structures in the standard library that you can use in your nice, neat code!


### Common Collections
Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time. In this chapter, we’ll discuss three collections that are used very often in Rust programs:

A vector allows us to store a variable number of values next to each other.
A string is a collection of characters. We’ve discussed the String type previously, but in this chapter we’ll talk about it in depth.
A hash map allows us to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
To learn about the other kinds of collections provided by the standard library, see the documentation.

We’ll discuss how to create and update vectors, strings, and hash maps, as well as what makes each special.

#### Vectors Store Lists of Values
The first collection type we’ll look at is `Vec<T>`, also known as a vector. Vectors allow us to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful in situations in which you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

Creating a New Vector
To create a new, empty vector, we can call the `Vec::new` function as shown in Listing 8-1:



`let v: Vec<i32> = Vec::new();`
Listing 8-1: Creating a new, empty vector to hold values of type i32

Note that we added a type annotation here. Because we aren’t inserting any values into this vector, Rust doesn’t know what kind of elements we intend to store. This is an important point. Vectors are implemented using generics; we’ll cover how to use generics with your own types in Chapter 10. For now, know that the `Vec<T>` type provided by the standard library can hold any type, and when a specific vector holds a specific type, the type is specified within angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in v will hold elements of the i32 type.

In more realistic code, Rust can often infer the type of value we want to store once we insert values, so you rarely need to do this type annotation. It’s more common to create a `Vec<T>` that has initial values, and Rust provides the `vec!` macro for convenience. The macro will create a new vector that holds the values we give it. Listing 8-2 creates a new `Vec<i32> `that holds the values 1, 2, and 3:



`let v = vec![1, 2, 3];`
Listing 8-2: Creating a new vector containing values

Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary. Next, we’ll look at how to modify a vector.

#### Updating a Vector
To create a vector and then add elements to it, we can use the push method as shown in Listing 8-3:


```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
Listing 8-3: Using the push method to add values to a vector

As with any variable, as discussed in Chapter 3, if we want to be able to change its value, we need to make it mutable using the mut keyword. The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.

Dropping a Vector Drops Its Elements
Like any other struct, a vector will be freed when it goes out of scope, as annotated in Listing 8-4:

```
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```
Listing 8-4: Showing where the vector and its elements are dropped

When the vector gets dropped, all of its contents will also be dropped, meaning those integers it holds will be cleaned up. This may seem like a straightforward point but can get a bit more complicated when we start to introduce references to the elements of the vector. Let’s tackle that next!

Reading Elements of Vectors
Now that you know how to create, update, and destroy vectors, knowing how to read their contents is a good next step. There are two ways to reference a value stored in a vector. In the examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.

Listing 8-5 shows both methods of accessing a value in a vector either with indexing syntax or the get method:


```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```
Listing 8-5: Using indexing syntax or the get method to access an item in a vector

Note two details here. First, we use the index value of 2 to get the third element: vectors are indexed by number, starting at zero. Second, the two different ways to get the third element are by using & and [], which gives us a reference, or by using the get method with the index passed as an argument, which gives us an Option<&T>.

The reason Rust has two ways to reference an element is so you can choose how the program behaves when you try to use an index value that the vector doesn’t have an element for. As an example, let’s see what a program will do if it has a vector that holds five elements and then tries to access an element at index 100, as shown in Listing 8-6:


```
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
   Compiling playground v0.0.1 (file:///playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.53 secs
     Running `target/debug/playground`
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', /checkout/src/liballoc/vec.rs:1551:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
Listing 8-6: Attempting to access the element at index 100 in a vector containing 5 elements

When you run this code, the first [] method will cause a panic! because it references a nonexistent element. This method is best used when you want your program to consider an attempt to access an element past the end of the vector to be a fatal error that crashes the program.

When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector happens occasionally under normal circumstances. Your code will then have logic to handle having either Some(&element) or None, as discussed in Chapter 6. For example, the index could be coming from a person entering a number. If they accidentally enter a number that’s too large and the program gets a None value, you could tell the user how many items are in the current vector and give them another chance to enter a valid value. That would be more user-friendly than crashing the program due to a typo!

#### Invalid References
When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure this reference and any other references to the contents of the vector remain valid. Recall the rule that states we can’t have mutable and immutable references in the same scope. That rule applies in Listing 8-7 where we hold an immutable reference to the first element in a vector and try to add an element to the end, which won’t work:

```
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);
```
Listing 8-7: Attempting to add an element to a vector while holding a reference to an item

Compiling this code will result in this error:

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 -->
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^ mutable borrow occurs here
7 |
8 | }
  | - immutable borrow ends here
```
The code in Listing 8-7 might look like it should work: why should a reference to the first element care about what changes at the end of the vector? The reason behind this error is due to the way vectors work: adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space if there isn’t enough room to put all the elements next to each other where the vector was. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

Note: For more on the implementation details of the Vec<T> type, see “The Rustonomicon” at https://doc.rust-lang.org/stable/nomicon/vec.html.

#### Iterating Over the Values in a Vector
If we want to access each element in a vector in turn, we can iterate through all of the elements rather than use indexes to access one at a time. Listing 8-8 shows how to use a for loop to get immutable references to each element in a vector of i32 values and print them out:


```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

Listing 8-8: Printing each element in a vector by iterating over the elements using a for loop

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop in Listing 8-9 will add 50 to each element:


```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

Listing 8-9: Iterating over mutable references to elements in a vector

To change the value that the mutable reference refers to, we have to use the dereference operator (`*`) to get to the value in i before we can use the += operator .

#### Using an Enum to Store Multiple Types
At the beginning of this chapter, we said that vectors can only store values that are the same type. This can be inconvenient; there are definitely use cases for needing to store a list of items of different types. Fortunately, the variants of an enum are defined under the same enum type, so when we need to store elements of a different type in a vector, we can define and use an enum!

For example, let’s say we want to get values from a row in a spreadsheet where some of the columns in the row contain integers, some floating-point numbers, and some strings. We can define an enum whose variants will hold the different value types, and then all the enum variants will be considered the same type: that of the enum. Then we can create a vector that holds that enum and so, ultimately, holds different types. We’ve demonstrated this in Listing 8-10:


```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
Listing 8-10: Defining an enum to store values of different types in one vector

The reason Rust needs to know what types will be in the vector at compile time is so it knows exactly how much memory on the heap will be needed to store each element. A secondary advantage is that we can be explicit about what types are allowed in this vector. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector. Using an enum plus a match expression means that Rust will ensure at compile time that we always handle every possible case, as discussed in Chapter 6.

If you don’t know the exhaustive set of types the program will get at runtime to store in a vector when you’re writing a program, the enum technique won’t work. Instead, you can use a trait object, which we’ll cover in Chapter 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure to review the API documentation for all the many useful methods defined on Vec<T> by the standard library. For example, in addition to push, a pop method removes and returns the last element. Let’s move on to the next collection type: String!


### Strings Store UTF-8 Encoded Text
We talked about strings in Chapter 4, but we’ll look at them in more depth now. New Rustaceans commonly get stuck on strings due to a combination of three concepts: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8. These concepts combine in a way that can seem difficult when you’re coming from other programming languages.

This discussion of strings is in the collections chapter because strings are implemented as a collection of bytes plus some methods to provide useful functionality when those bytes are interpreted as text. In this section, we’ll talk about the operations on String that every collection type has, such as creating, updating, and reading. We’ll also discuss the ways in which String is different than the other collections, namely how indexing into a String is complicated by the differences between how people and computers interpret String data.

#### What Is a String?
We’ll first define what we mean by the term string. Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. In Chapter 4, we talked about string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the binary output of the program and are therefore string slices.

The String type is provided in Rust’s standard library rather than coded into the core language and is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types, not just one of those types. Although this section is largely about String, both types are used heavily in Rust’s standard library and both String and string slices are UTF-8 encoded.

Rust’s standard library also includes a number of other string types, such as OsString, OsStr, CString, and CStr. Library crates can provide even more options for storing string data. Similar to the `*String/*Str` naming, they often provide an owned and borrowed variant, just like String/&str. These string types can store text in different encodings or be represented in memory in a different way, for example. We won’t discuss these other string types in this chapter; see their API documentation for more about how to use them and when each is appropriate.

#### Creating a New String
Many of the same operations available with Vec<T> are available with String as well, starting with the new function to create a string, shown in Listing 8-11:



`let mut s = String::new();`
Listing 8-11: Creating a new, empty String

This line creates a new empty string called s that we can then load data into. Often, we’ll have some initial data that we want to start the string with. For that, we use the to_string method, which is available on any type that implements the Display trait, which string literals do. Listing 8-12 shows two examples:


```
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```
Listing 8-12: Using the to_string method to create a String from a string literal

This code creates a string containing initial contents.

We can also use the function String::from to create a String from a string literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12 that uses to_string:



`let s = String::from("initial contents");`
Listing 8-13: Using the String::from function to create a String from a string literal

Because strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options. Some of them can seem redundant, but they all have their place! In this case, String::from and to_string do the same thing, so which you choose is a matter of style.

Remember that strings are UTF-8 encoded, so we can include any properly encoded data in them, as shown in Listing 8-14:


```
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
```
Listing 8-14: Storing greetings in different languages in strings

All of these are valid String values.

#### Updating a String
A String can grow in size and its contents can change, just like the contents of a Vec<T>, by pushing more data into it. In addition, we can conveniently use the + operator or the format! macro to concatenate String values together.

Appending to a String with push_str and push
We can grow a String by using the push_str method to append a string slice, as shown in Listing 8-15:


```
let mut s = String::from("foo");
s.push_str("bar");
```

Listing 8-15: Appending a string slice to a String using the push_str method

After these two lines, s will contain foobar. The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter. For example, the code in Listing 8-16 shows that it would be unfortunate if we weren’t able to use s2 after appending its contents to s1:


```
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(&s2);
println!("s2 is {}", s2);
```
Listing 8-16: Using a string slice after appending its contents to a String

If the push_str method took ownership of s2, we wouldn’t be able to print out its value on the last line. However, this code works as we’d expect!

The push method takes a single character as a parameter and adds it to the String. Listing 8-17 shows code that adds the letter l character to a String using the push method:


```
let mut s = String::from("lo");
s.push('l');
```
Listing 8-17: Adding one character to a String value using push

As a result of this code, s will contain lol.

Concatenation with the + Operator or the format! Macro
Often, we’ll want to combine two existing strings. One way is to use the + operator, as shown in Listing 8-18:


```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
```
Listing 8-18: Using the + operator to combine two String values into a new String value

The string s3 will contain Hello, world! as a result of this code. The reason s1 is no longer valid after the addition and the reason we used a reference to s2 has to do with the signature of the method that gets called when we use the + operator. The + operator uses the add method, whose signature looks something like this:


`fn add(self, s: &str) -> String {`
This isn’t the exact signature that’s in the standard library: in the standard library, add is defined using generics. Here, we’re looking at the signature of add with concrete types substituted for the generic ones, which is what happens when we call this method with String values. We’ll discuss generics in Chapter 10. This signature gives us the clues we need to understand the tricky bits of the + operator.

First, `s2` has an `&,` meaning that we’re adding a reference of the second string to the first string because of the `s` parameter in the add function: we can only add a `&str` to a `String`; we can’t add two `String` values together. But wait - the type of `&s2` is `&String`, not `&str`, as specified in the second parameter to add. So why does Listing 8-18 compile?

The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]. We’ll discuss deref coercion in more depth in Chapter 15. Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.

Second, we can see in the signature that add takes ownership of self, because self does not have an &. This means s1 in Listing 8-18 will be moved into the add call and no longer be valid after that. So although let `s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. In other words, it looks like it’s making a lot of copies but isn’t: the implementation is more efficient than copying.

If we need to concatenate multiple strings, the behavior of + gets unwieldy:


```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
```
At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on. For more complicated string combining, we can use the format! macro:


```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```
This code also sets s to tic-tac-toe. The format! macro works in the same way as println!, but instead of printing the output to the screen, it returns a String with the contents. The version of the code using format! is much easier to read and also doesn’t take ownership of any of its parameters.

#### Indexing into Strings
In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation. However, if we try to access parts of a String using indexing syntax in Rust, we’ll get an error. Consider the invalid code in Listing 8-19:

```
let s1 = String::from("hello");
let h = s1[0];
```
Listing 8-19: Attempting to use indexing syntax with a String

This code will result in the following error:

```
error[E0277]: the trait bound `std::string::String: std::ops::Index<{integer}>` is not satisfied
 -->
  |
3 |     let h = s1[0];
  |             ^^^^^ the type `std::string::String` cannot be indexed by `{integer}`
  |
  = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
```
The error and the note tell the story: Rust strings don’t support indexing. But why not? To answer that question, we need to discuss how Rust stores strings in memory.

#### Internal Representation
A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 example strings from Listing 8-14. First, this one:



`let len = String::from("Hola").len();`
In this case, `len` will be four, which means the Vec storing the string “Hola” is four bytes long. Each of these letters takes one byte when encoded in UTF-8. But what about the following line?



`let len = String::from("Здравствуйте").len();`
Note that this string begins with the capital Cyrillic letter Ze, not the Arabic number 3. Asked how long the string is, you might say 12. However, Rust’s answer is 24: that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because each Unicode scalar value takes two bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value. To demonstrate, consider this invalid Rust code:

```
let hello = "Здравствуйте";
let answer = &hello[0];
```
What should the value of answer be? Should it be З, the first letter? When encoded in UTF-8, the first byte of З is 208, and the second is 151, so answer should in fact be 208, but 208 is not a valid character on its own. Returning 208 is likely not what a user would want if they asked for the first letter of this string; however, that’s the only data that Rust has at byte index 0. Returning the byte value is probably not what users want, even if the string contains only Latin letters: if `&"hello"[0]` was valid code that returned the byte value, it would return 104, not h. To avoid returning an unexpected value and causing bugs that might not be discovered immediately, Rust doesn’t compile this code at all and prevents misunderstandings earlier in the development process.

Bytes and Scalar Values and Grapheme Clusters! Oh My!
Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is ultimately stored as a Vec of u8 values that looks like this:

```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```
That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:


`['न', 'म', 'स', '्', 'त', 'े']`
There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:


`["न", "म", "स्", "ते"]`
Rust provides different ways of interpreting the raw string data that computers store so that each program can choose the interpretation it needs, no matter what human language the data is in.

A final reason Rust doesn’t allow us to index into a String to get a character is that indexing operations are expected to always take constant time (O(1)). But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.

#### Slicing Strings
Indexing into a string is often a bad idea because it’s not clear what the return type of the string indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices. To be more specific in your indexing and indicate that you want a string slice, rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes:


```
let hello = "Здравствуйте";

let s = &hello[0..4];
```

Here, s will be a &str that contains the first four bytes of the string. Earlier, we mentioned that each of these characters was two bytes, which means s will be Зд.

What would happen if we used &hello[0..1]? The answer: Rust will panic at runtime in the same way that accessing an invalid index in a vector does:


thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', `src/libcore/str/mod.rs:2188:4`
You should use ranges to create string slices with caution, because it can crash your program.

#### Methods for Iterating Over Strings
Fortunately, we can access elements in a string in other ways.

If we need to perform operations on individual Unicode scalar values, the best way to do so is to use the chars method. Calling chars on “नमस्ते” separates out and returns six values of type char, and we can iterate over the result in order to access each element:


```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
This code will print the following:

```
न
म
स
्
त
े
```
The bytes method returns each raw byte, which might be appropriate for your domain:


```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

This code will print the 18 bytes that make up this String, starting with:

```
224
164
168
224
// ... etc
```
But be sure to remember that valid Unicode scalar values may be made up of more than one byte.

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. Crates are available on crates.io if this is the functionality you need.

#### Strings Are Not So Simple
To summarize, strings are complicated. Different programming languages make different choices about how to present this complexity to the programmer. Rust has chosen to make the correct handling of String data the default behavior for all Rust programs, which means programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than other programming languages do but prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.

Let’s switch to something a bit less complex: hash maps!


### Hash Maps Store Keys Associated with Values
The last of our common collections is the hash map. The type HashMap<K, V> stores a mapping of keys of type K to values of type V. It does this via a hashing function, which determines how it places these keys and values into memory. Many different programming languages support this kind of data structure, but often use a different name, such as hash, map, object, hash table, or associative array, just to name a few.

Hash maps are useful for when you want to look up data not by an index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team’s score in a hash map where each key is a team’s name and the values are each team’s score. Given a team name, you can retrieve its score.

We’ll go over the basic API of hash maps in this section, but many more goodies are hiding in the functions defined on `HashMap<K, V>` by the standard library. As always, check the standard library documentation for more information.

#### Creating a New Hash Map
We can create an empty hash map with new and add elements with insert. In Listing 8-20, we’re keeping track of the scores of two teams whose names are Blue and Yellow. The Blue team will start with 10 points, and the Yellow team starts with 50:


```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```
Listing 8-20: Creating a new hash map and inserting some keys and values

Note that we need to first use the HashMap from the collections portion of the standard library. Of our three common collections, this one is the least often used, so it’s not included in the features brought into scope automatically in the prelude. Hash maps also have less support from the standard library; there’s no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This HashMap has keys of type String and values of type i32. Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

Another way of constructing a hash map is by using the collect method on a vector of tuples, where each tuple consists of a key and its value. The collect method gathers data into a number of collection types, including HashMap. For example, if we had the team names and initial scores in two separate vectors, we can use the zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. Then we can use the collect method to turn that vector of tuples into a HashMap as shown in Listing 8-21:


```
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```
Listing 8-21: Creating a hash map from a list of teams and a list of scores

The type annotation HashMap`<_, _>` is needed here because it’s possible to collect into many different data structures, and Rust doesn’t know which you want unless you specify. For the type parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

#### Hash Maps and Ownership
For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values as demonstrated in Listing 8-22:


```
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```
Listing 8-22: Showing that keys and values are owned by the hash map once they’re inserted

We aren’t able to use the variables field_name and field_value after they’ve been moved into the hash map with the call to insert.

If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid. We’ll talk more about these issues in the “Validating References with Lifetimes” section in Chapter 10.

#### Accessing Values in a Hash Map
We can get a value out of the hash map by providing its key to the get method as shown in Listing 8-23:


```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
Listing 8-23: Accessing the score for the Blue team stored in the hash map

Here, score will have the value that’s associated with the Blue team, and the result will be Some(&10). The result is wrapped in Some because get returns an Option<&V>; if there’s no value for that key in the hash map, get will return None. The program will need to handle the Option in one of the ways that we covered in Chapter 6.

We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:


```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
This code will print each pair in an arbitrary order:

```
Yellow: 50
Blue: 10
```
Updating a Hash Map
Although the number of keys and values is growable, each key can only have one value associated with it at a time. When we want to change the data in a hash map, we have to decide how to handle the case when a key already has a value assigned. We could replace the old value with the new value, completely disregarding the old value. We could keep the old value and ignore the new value, and only add the new value if the key doesn’t already have a value. Or we could combine the old value and the new value. Let’s look at how to do each of these!

#### Overwriting a Value
If we insert a key and a value into a hash map, and then insert that same key with a different value, the value associated with that key will be replaced. Even though the code in Listing 8-24 calls insert twice, the hash map will only contain one key/value pair because we’re inserting the value for the Blue team’s key both times:


```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```
Listing 8-24: Replacing a value stored with a particular key

This code will print {"Blue": 25}. The original value of 10 has been overwritten.

#### Only Insert If the Key Has No Value
It’s common to check whether a particular key has a value, and if it doesn’t, insert a value for it. Hash maps have a special API for this called entry that takes the key we want to check as a parameter. The return value of the entry function is an enum called Entry that represents a value that might or might not exist. Let’s say we want to check whether the key for the Yellow team has a value associated with it. If it doesn’t, we want to insert the value 50, and the same for the Blue team. Using the entry API, the code looks like Listing 8-25:


```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```
Listing 8-25: Using the entry method to only insert if the key does not already have a value

The or_insert method on Entry is defined to return the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns the modified Entry. This technique is much cleaner than writing the logic ourselves, and in addition, plays more nicely with the borrow checker.

Running the code in Listing 8-25 will print {"Yellow": 50, "Blue": 10}. The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to entry will not change the hash map because the Blue team already has the value 10.

#### Updating a Value Based on the Old Value
Another common use case for hash maps is to look up a key’s value and then update it based on the old value. For instance, Listing 8-26 shows code that counts how many times each word appears in some text. We use a hash map with the words as keys and increment the value to keep track of how many times we’ve seen that word. If it’s the first time we’ve seen a word, we’ll first insert the value 0:


```
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
Listing 8-26: Counting occurrences of words using a hash map that stores words and counts

This code will print {"world": 2, "hello": 1, "wonderful": 1}. The or_insert method actually returns a mutable reference (&mut V) to the value for this key. Here we store that mutable reference in the count variable, so in order to assign to that value we must first dereference count using the asterisk (`*`). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

#### Hashing Function
By default, HashMap uses a cryptographically secure hashing function that can provide resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

#### Summary
Vectors, strings, and hash maps will provide a large amount of functionality that you need in programs where you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

Given a list of integers, use a vector and return the mean (average), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail; so, it’s a perfect time to discuss error handling next!

## Error Handling
Rust’s commitment to reliability extends to error handling. Errors are a fact of life in software, so Rust has a number of features for handling situations in which something goes wrong. In many cases, Rust requires you to acknowledge the possibility of an error occurring and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!

Rust groups errors into two major categories: recoverable and unrecoverable errors. Recoverable errors are situations in which it’s reasonable to report the problem to the user and retry the operation, like a file not found error. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Most languages don’t distinguish between these two kinds of errors and handle both in the same way using mechanisms like exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when it encounters unrecoverable errors. This chapter covers calling panic! first and then talks about returning Result<T, E> values. Additionally, we’ll explore considerations to take into account when deciding whether to try to recover from an error or to stop execution.

### Unrecoverable Errors with panic!
Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit. The most common situation this occurs in is when a bug of some kind has been detected, and it’s not clear to the programmer how to handle the error.

Unwinding the Stack or Aborting in Response to a panic!
By default, when a panic! occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. But this walking back and cleanup is a lot of work. The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting on panic by adding panic = 'abort' to the appropriate [profile] sections in your `Cargo.toml` file. For example, if you want to abort on panic in release mode, add this:

```
[profile.release]
panic = 'abort'
Let’s try calling panic! in a simple program:

Filename: src/main.rs


fn main() {
    panic!("crash and burn");
}
When you run the program, you’ll see something like this:


$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25 secs
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:4
```
note: Run with `RUST_BACKTRACE=1` for a backtrace.
The call to panic! causes the error message contained in the last three lines. The first line shows our panic message and the place in our source code where the panic occurred: src/main.rs:2:4 indicates that it’s the second line, fourth character of our src/main.rs file.

In this case, the line indicated is part of our code, and if we go to that line, we see the panic! macro call. In other cases, the panic! call might be in code that our code calls. The filename and line number reported by the error message will be someone else’s code where the panic! macro is called, not the line of our code that eventually led to the panic! call. We can use the backtrace of the functions the panic! call came from to figure out the part of our code that is causing the problem. We’ll discuss what a backtrace is in more detail next.

### Using a panic! Backtrace
Let’s look at another example to see what it’s like when a panic! call comes from a library because of a bug in our code instead of from our code calling the macro directly. Listing 9-1 has some code that attempts to access an element by index in a vector:

Filename: `src/main.rs`

```
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
Listing 9-1: Attempting to access an element beyond the end of a vector, which will cause a panic!

Here, we’re attempting to access the hundredth element of our vector (which is at index 99 because indexing starts at zero), but it has only three elements. In this situation, Rust will panic. Using [] is supposed to return an element, but if you pass an invalid index, there’s no element that Rust could return here that would be correct.

Other languages, like C, will attempt to give you exactly what you asked for in this situation, even though it isn’t what you want: you’ll get whatever is at the location in memory that would correspond to that element in the vector, even though the memory doesn’t belong to the vector. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the array.

To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue. Let’s try it and see:

```
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
error: Process didn't exit successfully: `target/debug/panic` (exit code: 101)
```
This error points at a file we didn’t write, vec.rs. That’s the implementation of Vec<T> in the standard library. The code that gets run when we use [] on our vector v is in vec.rs, and that is where the panic! is actually happening.

The next note line tells us that we can set the RUST_BACKTRACE environment variable to get a backtrace of exactly what happened to cause the error. A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work like they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote. That’s the spot where the problem originated. The lines above the lines mentioning your files are code that your code called; the lines below are code that called your code. These lines might include core Rust code, standard library code, or crates that you’re using. Let’s try getting a backtrace: Listing 9-2 shows output similar to what you’ll see:

```
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
             at /checkout/src/liballoc/vec.rs:1555
  11: panic::main
             at src/main.rs:4
  12: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  13: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  14: main
  15: __libc_start_main
  16: <unknown>
```
Listing 9-2: The backtrace generated by a call to panic! displayed when the environment variable RUST_BACKTRACE is set

That’s a lot of output! The exact output you see might be different depending on your operating system and Rust version. In order to get backtraces with this information, debug symbols must be enabled. Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.

In the output in Listing 9-2, line 11 of the backtrace points to the line in our project that’s causing the problem: src/main.rs in line 4. If we don’t want our program to panic, the location pointed to by the first line mentioning a file we wrote is where we should start investigating to figure out how we got to this location with values that caused the panic. In Listing 9-1 where we deliberately wrote code that would panic in order to demonstrate how to use backtraces, the way to fix the panic is to not request an element at index 99 from a vector that only contains three items. When your code panics in the future, you’ll need to figure out what action the code is taking with what values that causes the panic and what the code should do instead.

We’ll come back to panic! and when we should and should not use panic! to handle error conditions later in the chapter. Next, we’ll look at how to recover from an error using Result.


### Recoverable Errors with Result
Most errors aren’t serious enough to require the program to stop entirely. Sometimes, when a function fails, it’s for a reason that we can easily interpret and respond to. For example, if we try to open a file and that operation fails because the file doesn’t exist, we might want to create the file instead of terminating the process.

Recall from “Handling Potential Failure with the Result Type” in Chapter 2 that the Result enum is defined as having two variants, Ok and Err, as follows:


```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The T and E are generic type parameters: we’ll discuss generics in more detail in Chapter 10. What you need to know right now is that T represents the type of the value that will be returned in a success case within the Ok variant, and E represents the type of the error that will be returned in a failure case within the Err variant. Because Result has these generic type parameters, we can use the Result type and the functions that the standard library has defined on it in many different situations where the successful value and error value we want to return may differ.

Let’s call a function that returns a Result value because the function could fail: in Listing 9-3 we try to open a file:

Filename: src/main.rs

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```
Listing 9-3: Opening a file

How do we know File::open returns a Result? We could look at the standard library API documentation, or we could ask the compiler! If we give f a type annotation of a type that we know the return type of the function is not and then we try to compile the code, the compiler will tell us that the types don’t match. The error message will then tell us what the type of f is. Let’s try it: we know that the return type of File::open isn’t of type u32, so let’s change the let f statement to this:


`let f: u32 = File::open("hello.txt");`
Attempting to compile now gives us the following output:

```
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
             found type `std::result::Result<std::fs::File, std::io::Error>`
```
This tells us the return type of the File::open function is a Result<T, E>. The generic parameter T has been filled in here with the type of the success value, std::fs::File, which is a file handle. The type of E used in the error value is std::io::Error.

This return type means the call to File::open might succeed and return to us a file handle that we can read from or write to. The function call also might fail: for example, the file might not exist or we might not have permission to access the file. The File::open function needs to have a way to tell us whether it succeeded or failed and at the same time give us either the file handle or error information. This information is exactly what the Result enum conveys.

In the case where File::open succeeds, the value we will have in the variable f will be an instance of Ok that contains a file handle. In the case where it fails, the value in f will be an instance of Err that contains more information about the kind of error that happened.

We need to add to the code in Listing 9-3 to take different actions depending on the value File::open returned. Listing 9-4 shows one way to handle the Result using a basic tool: the match expression that we discussed in Chapter 6.

Filename: src/main.rs

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```
Listing 9-4: Using a match expression to handle the Result variants we might have

Note that, like the Option enum, the Result enum and its variants have been imported in the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.

Here we tell Rust that when the result is Ok, return the inner file value out of the Ok variant, and we then assign that file handle value to the variable f. After the match, we can then use the file handle for reading or writing.

The other arm of the match handles the case where we get an Err value from File::open. In this example, we’ve chosen to call the panic! macro. If there’s no file named hello.txt in our current directory and we run this code, we’ll see the following output from the panic! macro:

```
thread 'main' panicked at 'There was a problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```
As usual, this output tells us exactly what has gone wrong.

#### Matching on Different Errors
The code in Listing 9-4 will panic! no matter the reason that File::open failed. What we want to do instead is take different actions for different failure reasons: if File::open failed because the file doesn’t exist, we want to create the file and return the handle to the new file. If File::open failed for any other reason, for example because we didn’t have permission to open the file, we still want the code to panic! in the same way as it did in Listing 9-4. Look at Listing 9-5, which adds another arm to the match:

Filename: src/main.rs

```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```
Listing 9-5: Handling different kinds of errors in different ways

The type of the value that File::open returns inside the Err variant is io::Error, which is a struct provided by the standard library. This struct has a method kind that we can call to get an io::ErrorKind value. io::ErrorKind is an enum provided by the standard library that has variants representing the different kinds of errors that might result from an io operation. The variant we want to use is ErrorKind::NotFound, which indicates the file we’re trying to open doesn’t exist yet.

The condition if error.kind() == ErrorKind::NotFound is called a match guard: it’s an extra condition on a match arm that further refines the arm’s pattern. This condition must be true for that arm’s code to be run; otherwise, the pattern matching will move on to consider the next arm in the match. The ref in the pattern is needed so error is not moved into the guard condition but is merely referenced by it. The reason ref is used to take a reference in a pattern instead of & will be covered in detail in Chapter 18. In short, in the context of a pattern, & matches a reference and gives us its value, but ref matches a value and gives us a reference to it.

The condition we want to check in the match guard is whether the value returned by error.kind() is the NotFound variant of the ErrorKind enum. If it is, we try to create the file with File::create. However, because File::create could also fail, we need to add an inner match statement as well. When the file can’t be opened, a different error message will be printed. The last arm of the outer match stays the same so the program panics on any error besides the missing file error.

#### Shortcuts for Panic on Error: unwrap and expect
Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well. The Result<T, E> type has many helper methods defined on it to do various tasks. One of those methods, called unwrap, is a shortcut method that is implemented just like the match statement we wrote in Listing 9-4. If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:

Filename: `src/main.rs`

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
If we run this code without a hello.txt file, we’ll see an error message from the panic! call that the unwrap method makes:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```
Another method, expect, which is similar to unwrap, lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. The syntax of expect looks like this:

Filename: src/main.rs

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses. Here’s what it looks like:

```
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```
Because this error message starts with the text we specified, Failed to open hello.txt, it will be easier to find where in the code this error message is coming from. If we use unwrap in multiple places, it can take more time to figure out exactly which unwrap is causing the panic because all unwrap calls that panic print the same message.

Propagating Errors
When you’re writing a function whose implementation calls something that might fail, instead of handling the error within this function, you can return the error to the calling code so that it can decide what to do. This is known as propagating the error and gives more control to the calling code where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

For example, Listing 9-6 shows a function that reads a username from a file. If the file doesn’t exist or can’t be read, this function will return those errors to the code that called this function:

Filename: src/main.rs


```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
Listing 9-6: A function that returns errors to the calling code using match

Let’s look at the return type of the function first: Result<String, io::Error>. This means the function is returning a value of the type Result<T, E> where the generic parameter T has been filled in with the concrete type String, and the generic type E has been filled in with the concrete type io::Error. If this function succeeds without any problems, the code that calls this function will receive an Ok value that holds a String—the username that this function read from the file. If this function encounters any problems, the code that calls this function will receive an Err value that holds an instance of io::Error that contains more information about what the problems were. We chose io::Error as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the File::open function and the read_to_string method.

The body of the function starts by calling the File::open function. Then we handle the Result value returned with a match similar to the match in Listing 9-4, only instead of calling panic! in the Err case, we return early from this function and pass the error value from File::open back to the calling code as this function’s error value. If File::open succeeds, we store the file handle in the variable f and continue.

Then we create a new String in variable s and call the read_to_string method on the file handle in f to read the contents of the file into s. The read_to_string method also returns a Result because it might fail, even though File::open succeeded. So we need another match to handle that Result: if read_to_string succeeds, then our function has succeeded, and we return the username from the file that’s now in s wrapped in an Ok. If read_to_string fails, we return the error value in the same way that we returned the error value in the match that handled the return value of File::open. However, we don’t need to explicitly say return, because this is the last expression in the function.

The code that calls this code will then handle getting either an Ok value that contains a username or an Err value that contains an io::Error. We don’t know what the calling code will do with those values. If the calling code gets an Err value, it could call panic! and crash the program, use a default username, or look up the username from somewhere other than a file, for example. We don’t have enough information on what the calling code is actually trying to do, so we propagate all the success or error information upwards for it to handle appropriately.

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

A Shortcut for Propagating Errors: ?
Listing 9-7 shows an implementation of read_username_from_file that has the same functionality as it had in Listing 9-6, but this implementation uses the question mark operator:

Filename: src/main.rs


```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
Listing 9-7: A function that returns errors to the calling code using ?

The `?` placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression and the program will continue. If the value is an Err, the value inside the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

There is a difference between what the match expression from Listing 9-6 and the question mark operator do: error values used with `?` go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. When the question mark calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. As long as each error type implements the from function to define how to convert itself to the returned error type, the question mark operator takes care of the conversion automatically.

In the context of Listing 9-7, the `?` at the end of the File::open call will return the value inside an Ok to the variable f. If an error occurs, `?` will return early out of the whole function and give any Err value to the calling code. The same thing applies to the `?` at the end of the read_to_string call.

The `?` eliminates a lot of boilerplate and makes this function’s implementation simpler. We could even shorten this code further by chaining method calls immediately after the `?` as shown in Listing 9-8:

Filename: src/main.rs


```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```
Listing 9-8: Chaining method calls after the question mark operator

We’ve moved the creation of the new String in s to the beginning of the function; that part hasn’t changed. Instead of creating a variable f, we’ve chained the call to read_to_string directly onto the result of File::open("hello.txt")?. We still have a ? at the end of the read_to_string call, and we still return an Ok value containing the username in s when both File::open and read_to_string succeed rather than returning errors. The functionality is again the same as in Listing 9-6 and Listing 9-7; this is just a different, more ergonomic way to write it.

#### `?` Can Only Be Used in Functions That Return Result
The ? can only be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression we defined in Listing 9-6. The part of the match that requires a return type of Result is return Err(e), so the return type of the function must be a Result to be compatible with this return.

Let’s look at what happens if we use ? in the main function, which you’ll recall has a return type of ():

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```
When we compile this code, we get the following error message:

```
error[E0277]: the trait bound `(): std::ops::Try` is not satisfied
 --> src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ------------------------
  |             |
  |             the `?` operator can only be used in a function that returns
  `Result` (or another type that implements `std::ops::Try`)
  |             in this macro invocation
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```
This error points out that we’re only allowed to use the question mark operator in a function that returns Result. In functions that don’t return Result, when you call other functions that return Result, you’ll need to use a match or one of the Result methods to handle it instead of using ? to potentially propagate the error to the calling code.

Now that we’ve discussed the details of calling panic! or returning Result, let’s return to the topic of how to decide which is appropriate to use in which cases.


### To panic! or Not to panic!
So how do you decide when you should panic! and when you should return Result? When code panics, there’s no way to recover. You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision on behalf of the code calling your code that a situation is unrecoverable. When you choose to return a Result value, you give the calling code options rather than making the decision for it. The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call panic! and turn your recoverable error into an unrecoverable one. Therefore, returning Result is a good default choice when you’re defining a function that might fail.

In a few situations it’s more appropriate to write code that panics instead of returning a Result, but they are less common. Let’s explore why it’s appropriate to panic in examples, prototype code, and tests; then in situations where you as a human can know a method won’t fail that the compiler can’t reason about; and conclude with some general guidelines on how to decide whether to panic in library code.

#### Examples, Prototype Code, and Tests Are All Places it’s Perfectly Fine to Panic
When you’re writing an example to illustrate some concept, having robust error handling code in the example as well can make the example less clear. In examples, it’s understood that a call to a method like unwrap that could panic! is meant as a placeholder for the way that you’d want your application to handle errors, which can differ based on what the rest of your code is doing.

Similarly, the unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

If a method call fails in a test, we’d want the whole test to fail, even if that method isn’t the functionality under test. Because panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.

#### Cases When You Have More Information Than the Compiler
It would also be appropriate to call unwrap when you have some other logic that ensures the Result will have an Ok value, but the logic isn’t something the compiler understands. You’ll still have a Result value that you need to handle: whatever operation you’re calling still has the possibility of failing in general, even though it’s logically impossible in your particular situation. If you can ensure by manually inspecting the code that you’ll never have an Err variant, it’s perfectly acceptable to call unwrap. Here’s an example:


```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```
We’re creating an IpAddr instance by parsing a hardcoded string. We can see that 127.0.0.1 is a valid IP address, so it’s acceptable to use unwrap here. However, having a hardcoded, valid string doesn’t change the return type of the parse method: we still get a Result value, and the compiler will still make us handle the Result as if the Err variant is still a possibility because the compiler isn’t smart enough to see that this string is always a valid IP address. If the IP address string came from a user rather than being hardcoded into the program, and therefore did have a possibility of failure, we’d definitely want to handle the Result in a more robust way instead.

#### Guidelines for Error Handling
It’s advisable to have your code panic! when it’s possible that your code could end up in a bad state. In this context, bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

The bad state is not something that’s expected to happen occasionally.
Your code after this point needs to rely on not being in this bad state.
There’s not a good way to encode this information in the types you use.
If someone calls your code and passes in values that don’t make sense, the best choice might be to panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control, and it returns an invalid state that you have no way of fixing.

When a bad state is reached, but it’s expected to happen no matter how well you write your code, it’s still more appropriate to return a Result rather than making a panic! call. Examples of this include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit. In these cases, you should indicate that failure is an expected possibility by returning a Result to propagate these bad states upwards so the calling code can decide how to handle the problem. To panic! wouldn’t be the best way to handle these cases.

When your code performs operations on values, your code should verify the values are valid first, and panic! if the values aren’t valid. This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. This is the main reason the standard library will panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem. Functions often have contracts: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug, and it’s not a kind of error you want the calling code to have to explicitly handle. In fact, there’s no reasonable way for calling code to recover: the calling programmers need to fix the code. Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

However, having lots of error checks in all of your functions would be verbose and annoying. Fortunately, you can use Rust’s type system (and thus the type checking the compiler does) to do many of the checks for you. If your function has a particular type as a parameter, you can proceed with your code’s logic knowing that the compiler has already ensured you have a valid value. For example, if you have a type rather than an Option, your program expects to have something rather than nothing. Your code then doesn’t have to handle two cases for the Some and None variants: it will only have one case for definitely having a value. Code trying to pass nothing to your function won’t even compile, so your function doesn’t have to check for that case at runtime. Another example is using an unsigned integer type like u32, which ensures the parameter is never negative.

#### Creating Custom Types for Validation
Let’s take the idea of using Rust’s type system to ensure we have a valid value one step further and look at creating a custom type for validation. Recall the guessing game in Chapter 2 where our code asked the user to guess a number between 1 and 100. We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive. In this case, the consequences were not very dire: our output of “Too high” or “Too low” would still be correct. It would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that’s out of range versus when a user types, for example, letters instead.

One way to do this would be to parse the guess as an i32 instead of only a u32 to allow potentially negative numbers, and then add a check for the number being in range, like so:

```
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}
```
The if expression checks whether our value is out of range, tells the user about the problem, and calls continue to start the next iteration of the loop and ask for another guess. After the if expression, we can proceed with the comparisons between guess and the secret number knowing that guess is between 1 and 100.

However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, it would be tedious (and potentially impact performance) to have a check like this in every function.

Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere. That way, it’s safe for functions to use the new type in their signatures and confidently use the values they receive. Listing 9-9 shows one way to define a Guess type that will only create an instance of Guess if the new function receives a value between 1 and 100:


```
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```
Listing 9-9: A Guess type that will only continue with values between 1 and 100

First, we define a struct named Guess that has a field named value that holds a u32. This is where the number will be stored.

Then we implement an associated function named new on Guess that creates instances of Guess values. The new function is defined to have one parameter named value of type u32 and to return a Guess. The code in the body of the new function tests value to make sure it’s between 1 and 100. If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on. The conditions in which Guess::new might panic should be discussed in its public-facing API documentation; we’ll cover documentation conventions indicating the possibility of a panic! in the API documentation that you create in Chapter 14. If value does pass the test, we create a new Guess with its value field set to the value parameter and return the Guess.

Next, we implement a method named value that borrows self, doesn’t have any other parameters, and returns a u32. This is a kind of method sometimes called a getter, because its purpose is to get some data from its fields and return it. This public method is necessary because the value field of the Guess struct is private. It’s important that the value field is private so code using the Guess struct is not allowed to set value directly: code outside the module must use the Guess::new function to create an instance of Guess, which ensures there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.

A function that has a parameter or returns only numbers between 1 and 100 could then declare in its signature that it takes or returns a Guess rather than a u32 and wouldn’t need to do any additional checks in its body.

### Summary
Rust’s error handling features are designed to help you write more robust code. The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values. The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well. Using panic! and Result in the appropriate situations will make your code more reliable in the face of inevitable problems.

Now that you’ve seen useful ways that the standard library uses generics with the Option and Result enums, we’ll talk about how generics work and how you can use them in your code in the next chapter.

## Generic Types, Traits, and Lifetimes
Every programming language has tools to deal effectively with duplication of concepts; in Rust, one of those tools is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing and compiling the code we can express properties of generics, such as their behavior or how they relate to other generics, without needing to know what will actually be in their place.

In the same way that a function takes parameters whose value we don’t know in order to write code once that will be run on multiple concrete values, we can write functions that take parameters of some generic type instead of a concrete type like i32 or String. We’ve already used generics in Chapter 6 with Option<T>, Chapter 8 with `Vec<T>` and `HashMap<K, V>`, and Chapter 9 with `Result<T, E>`. In this chapter, we’ll explore how to define our own types, functions, and methods with generics!

First, we’re going to review the mechanics of extracting a function that reduces code duplication. Then we’ll use the same mechanics to make a generic function out of two functions that only differ in the types of their parameters. We’ll go over using generic types in struct and enum definitions too.

After that, we’ll discuss traits, which are a way to define behavior in a generic way. Traits can be combined with generic types in order to constrain a generic type to those types that have a particular behavior, rather than any type at all.

Finally, we’ll discuss lifetimes, which are a kind of generic that let us give the compiler information about how references are related to each other. Lifetimes are the feature in Rust that allow us to borrow values in many situations and still have the compiler check that references will be valid.

Removing Duplication by Extracting a Function
Before getting into generics syntax, let’s first review a technique for dealing with duplication that doesn’t use generic types: extracting a function. Once that’s fresh in our minds, we’ll use the same mechanics with generics to extract a generic function! In the same way that you recognize duplicated code to extract into a function, you’ll start to recognize duplicated code that can use generics.

Consider a small program that finds the largest number in a list, shown in Listing 10-1:

Filename: `src/main.rs`

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```
Listing 10-1: Code to find the largest number in a list of numbers

This code takes a list of integers, stored here in the variable number_list. It puts the first item in the list in a variable named largest. Then it iterates through all the numbers in the list, and if the current value is greater than the number stored in largest, it replaces the value in largest. If the current value is smaller than the largest value seen so far, largest is not changed. When all the items in the list have been considered, largest will hold the largest value, which in this case is 100.

If we needed to find the largest number in two different lists of numbers, we could duplicate the code in Listing 10-1 and have the same logic exist in two places in the program, as in Listing 10-2:

Filename: `src/main.rs`

```
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```
Listing 10-2: Code to find the largest number in two lists of numbers

While this code works, duplicating code is tedious and error-prone, and means we have multiple places to update the logic if we need to change it.

To eliminate this duplication, we can create an abstraction, which in this case will be in the form of a function that operates on any list of integers given to the function in a parameter. This will increase the clarity of our code and let us communicate and reason about the concept of finding the largest number in a list independently of the specific places this concept is used.

In the program in Listing 10-3, we’ve extracted the code that finds the largest number into a function named largest. This program can find the largest number in two different lists of numbers, but the code from Listing 10-1 only exists in one spot:

Filename: src/main.rs

```
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```
Listing 10-3: Abstracted code to find the largest number in two lists

The function has a parameter, list, which represents any concrete slice of i32 values that we might pass into the function. The code in the function definition operates on the list representation of any &[i32]. When we call the largest function, the code actually runs on the specific values that we pass in.

The mechanics we went through to get from Listing 10-2 to Listing 10-3 were these steps:

### We noticed there was duplicate code.
We extracted the duplicate code into the body of the function, and specified the inputs and return values of that code in the function signature.
We replaced the two concrete places that had the duplicated code to call the function instead.
We can use these same steps with generics to reduce code duplication in different ways in different scenarios. In the same way that the function body is now operating on an abstract list instead of concrete values, code using generics will operate on abstract types. The concepts powering generics are the same concepts you already know that power functions, just applied in different ways.

What if we had two functions, one that found the largest item in a slice of i32 values and one that found the largest item in a slice of char values? How would we get rid of that duplication? Let’s find out!


### Generic Data Types
Using generics where we usually place types, like in function signatures or structs, lets us create definitions that we can use for many different concrete data types. Let’s take a look at how to define functions, structs, enums, and methods using generics, and at the end of this section we’ll discuss the performance of code using generics.

Using Generic Data Types in Function Definitions
We can define functions that use generics in the signature of the function where the data types of the parameters and return value go. In this way, the code we write can be more flexible and provide more functionality to callers of our function, while not introducing code duplication.

Continuing with our largest function, Listing 10-4 shows two functions providing the same functionality to find the largest value in a slice. The first function is the one we extracted in Listing 10-3 that finds the largest i32 in a slice. The second function finds the largest char in a slice:

Filename: `src/main.rs`

```
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```
Listing 10-4: Two functions that differ only in their names and the types in their signatures

Here, the functions largest_i32 and largest_char have the exact same body, so it would be nice if we could turn these two functions into one and get rid of the duplication. Luckily, we can do that by introducing a generic type parameter!

To parameterize the types in the signature of the one function we’re going to define, we need to create a name for the type parameter, just like how we give names for the value parameters to a function. We’re going to choose the name T. Any identifier can be used as a type parameter name, but we’re choosing T because Rust’s type naming convention is CamelCase. Generic type parameter names also tend to be short by convention, often just one letter. Short for “type”, T is the default choice of most Rust programmers.

When we use a parameter in the body of the function, we have to declare the parameter in the signature so that the compiler knows what that name in the body means. Similarly, when we use a type parameter name in a function signature, we have to declare the type parameter name before we use it. Type name declarations go in angle brackets between the name of the function and the parameter list.

The function signature of the generic largest function we’re going to define will look like this:


`fn largest<T>(list: &[T]) -> T {`
We would read this as: the function largest is generic over some type T. It has one parameter named list, and the type of list is a slice of values of type T. The largest function will return a value of the same type T.

Listing 10-5 shows the unified largest function definition using the generic data type in its signature, and shows how we’ll be able to call largest with either a slice of `i32` values or char values. Note that this code won’t compile yet!

Filename: `src/main.rs`

```
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
Listing 10-5: A definition of the largest function that uses generic type parameters but doesn’t compile yet

If we try to compile this code right now, we’ll get this error:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
```
The note mentions `std::cmp::PartialOrd`, which is a trait. We’re going to talk about traits in the next section, but briefly, what this error is saying is that the body of largest won’t work for all possible types that T could be; since we want to compare values of type T in the body, we can only use types that know how to be ordered. The standard library has defined the trait std::cmp::PartialOrd that types can implement to enable comparisons. We’ll come back to traits and how to specify that a generic type has a particular trait in the next section, but let’s set this example aside for a moment and explore other places we can use generic type parameters first.

Using Generic Data Types in Struct Definitions
We can define structs to use a generic type parameter in one or more of the struct’s fields with the <> syntax too. Listing 10-6 shows the definition and use of a Point struct that can hold x and y coordinate values of any type:

Filename: src/main.rs

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```
Listing 10-6: A Point struct that holds x and y values of type T

The syntax is similar to using generics in function definitions. First, we have to declare the name of the type parameter within angle brackets just after the name of the struct. Then we can use the generic type in the struct definition where we would specify concrete data types.

Note that because we’ve only used one generic type in the definition of Point, what we’re saying is that the Point struct is generic over some type T, and the fields x and y are both that same type, whatever it ends up being. If we try to create an instance of a Point that has values of different types, as in Listing 10-7, our code won’t compile:

Filename: src/main.rs

```
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```
Listing 10-7: The fields x and y must be the same type because both have the same generic data type T

If we try to compile this, we’ll get the following error:

```
error[E0308]: mismatched types
 -->
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integral variable, found
  floating-point variable
  |
  = note: expected type `{integer}`
  = note:    found type `{float}`
```

When we assigned the integer value 5 to x, the compiler then knows for this instance of Point that the generic type T will be an integer. Then when we specified 4.0 for y, which is defined to have the same type as x, we get a type mismatch error.

If we wanted to define a Point struct where x and y could have different types but still have those types be generic, we can use multiple generic type parameters. In listing 10-8, we’ve changed the definition of Point to be generic over types T and U. The field x is of type T, and the field y is of type U:

Filename: src/main.rs

```
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```
Listing 10-8: A Point generic over two types so that x and y may be values of different types

Now all of these instances of Point are allowed! You can use as many generic type parameters in a definition as you want, but using more than a few gets hard to read and understand. If you get to a point of needing lots of generic types, it’s probably a sign that your code could use some restructuring to be separated into smaller pieces.

Using Generic Data Types in Enum Definitions
Similarly to structs, enums can be defined to hold generic data types in their variants. We used the Option<T> enum provided by the standard library in Chapter 6, and now its definition should make more sense. Let’s take another look:


```
enum Option<T> {
    Some(T),
    None,
}
```
In other words, Option<T> is an enum generic in type T. It has two variants: Some, which holds one value of type T, and a None variant that doesn’t hold any value. The standard library only has to have this one definition to support the creation of values of this enum that have any concrete type. The idea of “an optional value” is a more abstract concept than one specific type, and Rust lets us express this abstract concept without lots of duplication.

Enums can use multiple generic types as well. The definition of the Result enum that we used in Chapter 9 is one example:


```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
The Result enum is generic over two types, T and E. Result has two variants: Ok, which holds a value of type T, and Err, which holds a value of type E. This definition makes it convenient to use the Result enum anywhere we have an operation that might succeed (and return a value of some type T) or fail (and return an error of some type E). Recall Listing 9-2 when we opened a file: in that case, T was filled in with the type std::fs::File when the file was opened successfully and E was filled in with the type std::io::Error when there were problems opening the file.

When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can remove the duplication by using the same process we used with the function definitions to introduce generic types instead.

Using Generic Data Types in Method Definitions
Like we did in Chapter 5, we can implement methods on structs and enums that have generic types in their definitions. Listing 10-9 shows the Point<T> struct we defined in Listing 10-6. We’ve then defined a method named x on Point<T> that returns a reference to the data in the field x:

Filename: `src/main.rs`

```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference to the x field, which is of type T.

Note that we have to declare T just after impl in order to use T in the type Point<T>. Declaring T as a generic type after the impl is how Rust knows the type in the angle brackets in Point is a generic type rather than a concrete type. For example, we could choose to implement methods on Point<f32> instances rather than Point instances with any generic type. Listing 10-10 shows that we don’t declare anything after the impl in this case, since we’re using a concrete type, f32:


```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
Listing 10-10: Building an impl block which only applies to a struct with a specific type is used for the generic type parameter T

This code means the type Point<f32> will have a method named distance_from_origin, and other instances of Point<T> where T is not of type f32 will not have this method defined. This method measures how far our point is from the point of coordinates (0.0, 0.0) and uses mathematical operations which are only available for floating-point types.

Generic type parameters in a struct definition aren’t always the same generic type parameters you want to use in that struct’s method signatures. Listing 10-11 defines a method mixup on the Point<T, U> struct from Listing 10-8. The method takes another Point as a parameter, which might have different types than the self Point that we’re calling mixup on. The method creates a new Point instance that has the x value from the self Point (which is of type T) and the y value from the passed-in Point (which is of type W):

Filename: src/main.rs

```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
Listing 10-11: Methods that use different generic types than their struct’s definition

In main, we’ve defined a Point that has an i32 for x (with value 5) and an f64 for y (with value 10.4). p2 is a Point that has a string slice for x (with value "Hello") and a char for y (with value c). Calling mixup on p1 with the argument p2 gives us p3, which will have an i32 for x, since x came from p1. p3 will have a char for y, since y came from p2. The println! will print p3.x = 5, p3.y = c.

Note that the generic parameters T and U are declared after impl, since they go with the struct definition. The generic parameters V and W are declared after fn mixup, since they are only relevant to the method.

### Performance of Code Using Generics
You may have been reading this section and wondering if there’s a run-time cost to using generic type parameters. Good news: the way that Rust has implemented generics means that your code will not run any slower than if you had specified concrete types instead of generic type parameters!

Rust accomplishes this by performing monomorphization of code using generics at compile time. Monomorphization is the process of turning generic code into specific code with the concrete types that are actually used filled in.

What the compiler does is the opposite of the steps that we performed to create the generic function in Listing 10-5. The compiler looks at all the places that generic code is called and generates code for the concrete types that the generic code is called with.

Let’s work through an example that uses the standard library’s Option enum:


```
let integer = Some(5);
let float = Some(5.0);
```
When Rust compiles this code, it will perform monomorphization. The compiler will read the values that have been passed to Option and see that we have two kinds of Option<T>: one is i32, and one is f64. As such, it will expand the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

The monomorphized version of our code that the compiler generates looks like this, with the uses of the generic Option replaced with the specific definitions created by the compiler:

Filename: src/main.rs

```
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
We can write the non-duplicated code using generics, and Rust will compile that into code that specifies the type in each instance. That means we pay no runtime cost for using generics; when the code runs, it performs just like it would if we had duplicated each particular definition by hand. The process of monomorphization is what makes Rust’s generics extremely efficient at runtime.

### Using Generic Data Types in Method Definitions
Like we did in Chapter 5, we can implement methods on structs and enums that have generic types in their definitions. Listing 10-9 shows the Point<T> struct we defined in Listing 10-6. We’ve then defined a method named x on Point<T> that returns a reference to the data in the field x:

Filename: src/main.rs

```
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
Listing 10-9: Implementing a method named x on the Point<T> struct that will return a reference to the x field, which is of type T.

Note that we have to declare T just after impl in order to use T in the type Point<T>. Declaring T as a generic type after the impl is how Rust knows the type in the angle brackets in Point is a generic type rather than a concrete type. For example, we could choose to implement methods on Point<f32> instances rather than Point instances with any generic type. Listing 10-10 shows that we don’t declare anything after the impl in this case, since we’re using a concrete type, f32:


```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
Listing 10-10: Building an impl block which only applies to a struct with a specific type is used for the generic type parameter T

This code means the type Point<f32> will have a method named distance_from_origin, and other instances of Point<T> where T is not of type f32 will not have this method defined. This method measures how far our point is from the point of coordinates (0.0, 0.0) and uses mathematical operations which are only available for floating-point types.

Generic type parameters in a struct definition aren’t always the same generic type parameters you want to use in that struct’s method signatures. Listing 10-11 defines a method mixup on the Point<T, U> struct from Listing 10-8. The method takes another Point as a parameter, which might have different types than the self Point that we’re calling mixup on. The method creates a new Point instance that has the x value from the self Point (which is of type T) and the y value from the passed-in Point (which is of type W):

Filename: src/main.rs

```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
Listing 10-11: Methods that use different generic types than their struct’s definition

In main, we’ve defined a Point that has an i32 for x (with value 5) and an f64 for y (with value 10.4). p2 is a Point that has a string slice for x (with value "Hello") and a char for y (with value c). Calling mixup on p1 with the argument p2 gives us p3, which will have an i32 for x, since x came from p1. p3 will have a char for y, since y came from p2. The `println!` will print `p3.x = 5, p3.y = c.`

Note that the generic parameters T and U are declared after impl, since they go with the struct definition. The generic parameters V and W are declared after fn mixup, since they are only relevant to the method.

### Performance of Code Using Generics
You may have been reading this section and wondering if there’s a run-time cost to using generic type parameters. Good news: the way that Rust has implemented generics means that your code will not run any slower than if you had specified concrete types instead of generic type parameters!

Rust accomplishes this by performing monomorphization of code using generics at compile time. Monomorphization is the process of turning generic code into specific code with the concrete types that are actually used filled in.

What the compiler does is the opposite of the steps that we performed to create the generic function in Listing 10-5. The compiler looks at all the places that generic code is called and generates code for the concrete types that the generic code is called with.

Let’s work through an example that uses the standard library’s Option enum:


```
let integer = Some(5);
let float = Some(5.0);
```
When Rust compiles this code, it will perform monomorphization. The compiler will read the values that have been passed to Option and see that we have two kinds of Option<T>: one is i32, and one is f64. As such, it will expand the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.

The monomorphized version of our code that the compiler generates looks like this, with the uses of the generic Option replaced with the specific definitions created by the compiler:

Filename: src/main.rs

```
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
We can write the non-duplicated code using generics, and Rust will compile that into code that specifies the type in each instance. That means we pay no runtime cost for using generics; when the code runs, it performs just like it would if we had duplicated each particular definition by hand. The process of monomorphization is what makes Rust’s generics extremely efficient at runtime.

### Traits: Defining Shared Behavior
Traits allow us to use another kind of abstraction: they let us abstract over behavior that types can have in common. A trait tells the Rust compiler about functionality a particular type has and might share with other types. In situations where we use generic type parameters, we can use trait bounds to specify, at compile time, that the generic type may be any type that implements a trait and therefore has the behavior we want to use in that situation.

Note: Traits are similar to a feature often called ‘interfaces’ in other languages, though with some differences.

Defining a Trait
The behavior of a type consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together in order to define a set of behaviors necessary to accomplish some purpose.

For example, say we have multiple structs that hold various kinds and amounts of text: a NewsArticle struct that holds a news story filed in a particular place in the world, and a Tweet that can have at most 140 characters in its content along with metadata like whether it was a retweet or a reply to another tweet.

We want to make a media aggregator library that can display summaries of data that might be stored in a NewsArticle or Tweet instance. The behavior we need each struct to have is that it’s able to be summarized, and that we can ask for that summary by calling a summary method on an instance. Listing 10-12 shows the definition of a Summarizable trait that expresses this concept:

Filename: lib.rs


```
pub trait Summarizable {
    fn summary(&self) -> String;
}
```
Listing 10-12: Definition of a Summarizable trait that consists of the behavior provided by a summary method

We declare a trait with the trait keyword, then the trait’s name, in this case Summarizable. Inside curly brackets we declare the method signatures that describe the behaviors that types that implement this trait will need to have, in this case fn summary(&self) -> String. After the method signature, instead of providing an implementation within curly brackets, we put a semicolon. Each type that implements this trait must then provide its own custom behavior for the body of the method, but the compiler will enforce that any type that has the Summarizable trait will have the method summary defined for it with this signature exactly.

A trait can have multiple methods in its body, with the method signatures listed one per line and each line ending in a semicolon.

### Implementing a Trait on a Type
Now that we’ve defined the Summarizable trait, we can implement it on the types in our media aggregator that we want to have this behavior. Listing 10-13 shows an implementation of the Summarizable trait on the NewsArticle struct that uses the headline, the author, and the location to create the return value of summary. For the Tweet struct, we’ve chosen to define summary as the username followed by the whole text of the tweet, assuming that tweet content is already limited to 140 characters.

Filename: lib.rs


```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```
Listing 10-13: Implementing the Summarizable trait on the NewsArticle and Tweet types

Implementing a trait on a type is similar to implementing methods that aren’t related to a trait. The difference is after impl, we put the trait name that we want to implement, then say for and the name of the type that we want to implement the trait for. Within the impl block, we put the method signatures that the trait definition has defined, but instead of putting a semicolon after each signature, we put curly brackets and fill in the method body with the specific behavior that we want the methods of the trait to have for the particular type.

Once we’ve implemented the trait, we can call the methods on instances of NewsArticle and Tweet in the same manner that we call methods that aren’t part of a trait:

```
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```
This will print 1 new tweet: horse_ebooks: of course, as you probably already know, people.

Note that because we’ve defined the Summarizable trait and the NewsArticle and Tweet types all in the same lib.rs in Listing 10-13, they’re all in the same scope. If this lib.rs is for a crate we’ve called aggregator, and someone else wants to use our crate’s functionality plus implement the Summarizable trait on their WeatherForecast struct, their code would need to import the Summarizable trait into their scope first before they could implement it, like in Listing 10-14:

Filename: lib.rs

```
extern crate aggregator;

use aggregator::Summarizable;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of
        precipitation is {}%.", self.high_temp, self.low_temp,
        self.chance_of_precipitation)
    }
}
```
Listing 10-14: Bringing the Summarizable trait from our aggregator crate into scope in another crate

This code also assumes Summarizable is a public trait, which it is because we put the pub keyword before trait in Listing 10-12.

One restriction to note with trait implementations: we may implement a trait on a type as long as either the trait or the type are local to our crate. In other words, we aren’t allowed to implement external traits on external types. We can’t implement the Display trait on Vec, for example, since both Display and Vec are defined in the standard library. We are allowed to implement standard library traits like Display on a custom type like Tweet as part of our aggregator crate functionality. We could also implement Summarizable on Vec in our aggregator crate, since we’ve defined Summarizable there. This restriction is part of what’s called the orphan rule, which you can look up if you’re interested in type theory. Briefly, it’s called the orphan rule because the parent type is not present. Without this rule, two crates could implement the same trait for the same type, and the two implementations would conflict: Rust wouldn’t know which implementation to use. Because Rust enforces the orphan rule, other people’s code can’t break your code and vice versa.

Default Implementations
Sometimes it’s useful to have default behavior for some or all of the methods in a trait, instead of making every implementation on every type define custom behavior. When we implement the trait on a particular type, we can choose to keep or override each method’s default behavior.

Listing 10-15 shows how we could have chosen to specify a default string for the summary method of the Summarizable trait instead of choosing to only define the method signature like we did in Listing 10-12:

Filename: lib.rs


```
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}
```
Listing 10-15: Definition of a Summarizable trait with a default implementation of the summary method

If we wanted to use this default implementation to summarize instances of NewsArticle instead of defining a custom implementation like we did in Listing 10-13, we would specify an empty impl block:


`impl Summarizable for NewsArticle {}`
Even though we’re no longer choosing to define the summary method on NewsArticle directly, since the summary method has a default implementation and we specified that NewsArticle implements the Summarizable trait, we can still call the summary method on an instance of NewsArticle:

```
let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from("The Pittsburgh Penguins once again are the best
    hockey team in the NHL."),
};

println!("New article available! {}", article.summary());
```
This code prints New article available! (Read more...).

Changing the Summarizable trait to have a default implementation for summary does not require us to change anything about the implementations of Summarizable on Tweet in Listing 10-13 or WeatherForecast in Listing 10-14: the syntax for overriding a default implementation is exactly the same as the syntax for implementing a trait method that doesn’t have a default implementation.

Default implementations are allowed to call the other methods in the same trait, even if those other methods don’t have a default implementation. In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it. We could choose to have the Summarizable trait also have an author_summary method whose implementation is required, then a summary method that has a default implementation that calls the author_summary method:


```
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```
In order to use this version of Summarizable, we’re only required to define author_summary when we implement the trait on a type:

```
impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }
}
```
Once we define author_summary, we can call summary on instances of the Tweet struct, and the default implementation of summary will call the definition of author_summary that we’ve provided.

```
let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
};

println!("1 new tweet: {}", tweet.summary());
```
This will print 1 new tweet: (Read more from @horse_ebooks...).

Note that it is not possible to call the default implementation from an overriding implementation.

### Trait Bounds
Now that we’ve defined traits and implemented those traits on types, we can use traits with generic type parameters. We can constrain generic types so that rather than being any type, the compiler will ensure that the type will be limited to those types that implement a particular trait and thus have the behavior that we need the types to have. This is called specifying trait bounds on a generic type.

For example, in Listing 10-13, we implemented the Summarizable trait on the types NewsArticle and Tweet. We can define a function notify that calls the summary method on its parameter item, which is of the generic type T. To be able to call summary on item without getting an error, we can use trait bounds on T to specify that item must be of a type that implements the Summarizable trait:

```
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```
Trait bounds go with the declaration of the generic type parameter, after a colon and within the angle brackets. Because of the trait bound on T, we can call notify and pass in any instance of NewsArticle or Tweet. The external code from Listing 10-14 that’s using our aggregator crate can call our notify function and pass in an instance of WeatherForecast, since Summarizable is implemented for WeatherForecast as well. Code that calls notify with any other type, like a String or an i32, won’t compile, since those types do not implement Summarizable.

We can specify multiple trait bounds on a generic type by using +. If we needed to be able to use display formatting on the type T in a function as well as the summary method, we can use the trait bounds T: Summarizable + Display. This means T can be any type that implements both Summarizable and Display.

For functions that have multiple generic type parameters, each generic has its own trait bounds. Specifying lots of trait bound information in the angle brackets between a function’s name and its parameter list can get hard to read, so there’s an alternate syntax for specifying trait bounds that lets us move them to a where clause after the function signature. So instead of:

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
We can write this instead with a where clause:


fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```
This is less cluttered and makes this function’s signature look more similar to a function without lots of trait bounds, in that the function name, parameter list, and return type are close together.

Fixing the largest Function with Trait Bounds
So any time you want to use behavior defined by a trait on a generic, you need to specify that trait in the generic type parameter’s type bounds. We can now fix the definition of the largest function that uses a generic type parameter from Listing 10-5! When we set that code aside, we were getting this error:

```
error[E0369]: binary operation `>` cannot be applied to type `T`
  |
5 |         if item > largest {
  |            ^^^^
  |
```
note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
In the body of largest we wanted to be able to compare two values of type T using the greater-than operator. That operator is defined as a default method on the standard library trait std::cmp::PartialOrd. So in order to be able to use the greater-than operator, we need to specify PartialOrd in the trait bounds for T so that the largest function will work on slices of any type that can be compared. We don’t need to bring PartialOrd into scope because it’s in the prelude.


`fn largest<T: PartialOrd>(list: &[T]) -> T {`
If we try to compile this, we’ll get different errors:

```
error[E0508]: cannot move out of type `[T]`, a non-copy array
 --> src/main.rs:4:23
  |
4 |     let mut largest = list[0];
  |         -----------   ^^^^^^^ cannot move out of here
  |         |
  |         hint: to prevent move, use `ref largest` or `ref mut largest`

error[E0507]: cannot move out of borrowed content
 --> src/main.rs:6:9
  |
6 |     for &item in list.iter() {
  |         ^----
  |         ||
  |         |hint: to prevent move, use `ref item` or `ref mut item`
  |         cannot move out of borrowed content
```
The key to this error is cannot move out of type [T], a non-copy array. With our non-generic versions of the largest function, we were only trying to find the largest i32 or char. As we discussed in Chapter 4, types like i32 and char that have a known size can be stored on the stack, so they implement the Copy trait. When we changed the largest function to be generic, it’s now possible that the list parameter could have types in it that don’t implement the Copy trait, which means we wouldn’t be able to move the value out of list[0] and into the largest variable.

If we only want to be able to call this code with types that are Copy, we can add Copy to the trait bounds of T! Listing 10-16 shows the complete code of a generic largest function that will compile as long as the types of the values in the slice that we pass into largest implement both the PartialOrd and Copy traits, like i32 and char:

Filename: src/main.rs

```
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
Listing 10-16: A working definition of the largest function that works on any generic type that implements the PartialOrd and Copy traits

If we don’t want to restrict our largest function to only types that implement the Copy trait, we could specify that T has the trait bound Clone instead of Copy and clone each value in the slice when we want the largest function to have ownership. Using the clone function means we’re potentially making more heap allocations, though, and heap allocations can be slow if we’re working with large amounts of data. Another way we could implement largest is for the function to return a reference to a T value in the slice. If we change the return type to be &T instead of T and change the body of the function to return a reference, we wouldn’t need either the Clone or Copy trait bounds and we wouldn’t be doing any heap allocations. Try implementing these alternate solutions on your own!

### Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an impl block that uses generic type parameters, we can conditionally implement methods only for types that implement the specified traits. For example, the type Pair<T> in listing 10-17 always implements the new method, but Pair<T> only implements the cmp_display if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing:


```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
Listing 10-17: Conditionally implement methods on a generic type depending on trait bounds

We can also conditionally implement a trait for any type that implements a trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations, and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. This impl block looks similar to this code:

```
impl<T: Display> ToString for T {
    // --snip--
}
```
Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this since integers implement Display:



`let s = 3.to_string();`
Blanket implementations appear in the documentation for the trait in the “Implementors” section.

Traits and trait bounds let us write code that uses generic type parameters in order to reduce duplication, but still specify to the compiler exactly what behavior our code needs the generic type to have. Because we’ve given the trait bound information to the compiler, it can check that all the concrete types used with our code provide the right behavior. In dynamically typed languages, if we tried to call a method on a type that the type didn’t implement, we’d get an error at runtime. Rust moves these errors to compile time so that we’re forced to fix the problems before our code is even able to run. Additionally, we don’t have to write code that checks for behavior at runtime since we’ve already checked at compile time, which improves performance compared to other languages without having to give up the flexibility of generics.

There’s another kind of generics that we’ve been using without even realizing it called lifetimes. Rather than helping us ensure that a type has the behavior we need it to have, lifetimes help us ensure that references are valid as long as we need them to be. Let’s learn how lifetimes do that.

### Validating References with Lifetimes
When we talked about references in Chapter 4, we left out an important detail: every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like most of the time types are inferred. Similarly to when we have to annotate types because multiple types are possible, there are cases where the lifetimes of references could be related in a few different ways, so Rust needs us to annotate the relationships using generic lifetime parameters so that it can make sure the actual references used at runtime will definitely be valid.

Yes, it’s a bit unusual, and will be different to tools you’ve used in other programming languages. Lifetimes are, in some ways, Rust’s most distinctive feature.

Lifetimes are a big topic that can’t be covered in entirety in this chapter, so we’ll cover common ways you might encounter lifetime syntax in this chapter to get you familiar with the concepts. Chapter 19 will contain more advanced information about everything lifetimes can do.

Lifetimes Prevent Dangling References
The main aim of lifetimes is to prevent dangling references, which will cause a program to reference data other than the data we’re intending to reference. Consider the program in Listing 10-18, with an outer scope and an inner scope. The outer scope declares a variable named r with no initial value, and the inner scope declares a variable named x with the initial value of 5. Inside the inner scope, we attempt to set the value of r as a reference to x. Then the inner scope ends, and we attempt to print out the value in r:

```
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```
Listing 10-18: An attempt to use a reference whose value has gone out of scope

Uninitialized Variables Cannot Be Used
The next few examples declare variables without giving them an initial value, so that the variable name exists in the outer scope. This might appear to be in conflict with Rust not having null. However, if we try to use a variable before giving it a value, we’ll get a compile-time error. Try it out!

When we compile this code, we’ll get an error:

```
error: `x` does not live long enough
   |
6  |         r = &x;
   |              - borrow occurs here
7  |     }
   |     ^ `x` dropped here while still borrowed
...
10 | }
   | - borrowed value needs to live until here
```
The variable x doesn’t “live long enough.” Why not? Well, x is going to go out of scope when we hit the closing curly bracket on line 7, ending the inner scope. But r is valid for the outer scope; its scope is larger and we say that it “lives longer.” If Rust allowed this code to work, r would be referencing memory that was deallocated when x went out of scope, and anything we tried to do with r wouldn’t work correctly. So how does Rust determine that this code should not be allowed?

### The Borrow Checker
The part of the compiler called the borrow checker compares scopes to determine that all borrows are valid. Listing 10-19 shows the same example from Listing 10-18 with annotations showing the lifetimes of the variables:

```
{
    let r;                // -------+-- 'a
                          //        |
    {                     //        |
        let x = 5;        // -+-----+-- 'b
        r = &x;           //  |     |
    }                     // -+     |
                          //        |
    println!("r: {}", r); //        |
}                         // -------+
```
Listing 10-19: Annotations of the lifetimes of r and x, named 'a and 'b respectively

We’ve annotated the lifetime of r with 'a and the lifetime of x with 'b. As you can see, the inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a, but that it refers to an object with a lifetime of 'b. The program is rejected because the lifetime 'b is shorter than the lifetime of 'a: the subject of the reference does not live as long as the reference.

Let’s look at an example in Listing 10-20 that doesn’t try to make a dangling reference and compiles without any errors:


```
{
    let x = 5;            // -----+-- 'b
                          //      |
    let r = &x;           // --+--+-- 'a
                          //   |  |
    println!("r: {}", r); //   |  |
                          // --+  |
}                         // -----+
```
Listing 10-20: A valid reference because the data has a longer lifetime than the reference

Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x: Rust knows that the reference in r will always be valid while x is valid.

Now that we’ve shown where the lifetimes of references are in a concrete example and discussed how Rust analyzes lifetimes to ensure references will always be valid, let’s talk about generic lifetimes of parameters and return values in the context of functions.

### Generic Lifetimes in Functions
Let’s write a function that will return the longest of two string slices. We want to be able to call this function by passing it two string slices, and we want to get back a string slice. The code in Listing 10-21 should print The longest string is abcd once we’ve implemented the longest function:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
Listing 10-21: A main function that calls the longest function to find the longest of two string slices

Note that we want the function to take string slices (which are references, as we talked about in Chapter 4) since we don’t want the longest function to take ownership of its arguments. We want the function to be able to accept slices of a String (which is the type of the variable string1) as well as string literals (which is what variable string2 contains).

Refer back to the “String Slices as Parameters” section of Chapter 4 for more discussion about why these are the arguments we want.

If we try to implement the longest function as shown in Listing 10-22, it won’t compile:

Filename: src/main.rs

```
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
Listing 10-22: An implementation of the longest function that returns the longest of two string slices, but does not yet compile

Instead we get the following error that talks about lifetimes:

```
error[E0106]: missing lifetime specifier
   |
1  | fn longest(x: &str, y: &str) -> &str {
   |                                 ^ expected lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but the
   signature does not say whether it is borrowed from `x` or `y`
```
The help text is telling us that the return type needs a generic lifetime parameter on it because Rust can’t tell if the reference being returned refers to x or y. Actually, we don’t know either, since the if block in the body of this function returns a reference to x and the else block returns a reference to y!

As we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes like we did in Listings 10-19 and 10-20 in order to determine that the reference we return will always be valid. The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value. We’re going to add generic lifetime parameters that will define the relationship between the references so that the borrow checker can perform its analysis.

### Lifetime Annotation Syntax
Lifetime annotations don’t change how long any of the references involved live. In the same way that functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime when the signature specifies a generic lifetime parameter. What lifetime annotations do is relate the lifetimes of multiple references to each other.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe '. The names of lifetime parameters are usually all lowercase, and like generic types, their names are usually very short. 'a is the name most people use as a default. Lifetime parameter annotations go after the & of a reference, and a space separates the lifetime annotation from the reference’s type.

Here’s some examples: we’ve got a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a:

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
One lifetime annotation by itself doesn’t have much meaning: lifetime annotations tell Rust how the generic lifetime parameters of multiple references relate to each other. If we have a function with the parameter first that is a reference to an i32 that has the lifetime 'a, and the function has another parameter named second that is another reference to an i32 that also has the lifetime 'a, these two lifetime annotations that have the same name indicate that the references first and second must both live as long as the same generic lifetime.

Lifetime Annotations in Function Signatures
Let’s look at lifetime annotations in the context of the longest function we’re working on. Just like generic type parameters, generic lifetime parameters need to be declared within angle brackets between the function name and the parameter list. The constraint we want to tell Rust about for the references in the parameters and the return value is that they all must have the same lifetime, which we’ll name 'a and add to each reference as shown in Listing 10-23:

Filename: src/main.rs


```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
Listing 10-23: The longest function definition that specifies all the references in the signature must have the same lifetime, 'a

This will compile and will produce the result we want when used with the main function in Listing 10-21.

The function signature now says that for some lifetime 'a, the function will get two parameters, both of which are string slices that live at least as long as the lifetime 'a. The function will return a string slice that also will last at least as long as the lifetime 'a. This is the contract we are telling Rust we want it to enforce.

By specifying the lifetime parameters in this function signature, we are not changing the lifetimes of any values passed in or returned, but we are saying that any values that do not adhere to this contract should be rejected by the borrow checker. This function does not know (or need to know) exactly how long x and y will live, but only needs to know that there is some scope that can be substituted for 'a that will satisfy this signature.

When annotating lifetimes in functions, the annotations go on the function signature, and not in any of the code in the function body. This is because Rust is able to analyze the code within the function without any help, but when a function has references to or from code outside that function, the lifetimes of the arguments or return values will potentially be different each time the function is called. This would be incredibly costly and often impossible for Rust to figure out. In this case, we need to annotate the lifetimes ourselves.

When concrete references are passed to longest, the concrete lifetime that gets substituted for 'a is the part of the scope of x that overlaps with the scope of y. Since scopes always nest, another way to say this is that the generic lifetime 'a will get the concrete lifetime equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will therefore be guaranteed to be valid as long as the shorter of the lifetimes of x and y.

Let’s see how this restricts the usage of the longest function by passing in references that have different concrete lifetimes. Listing 10-24 is a straightforward example that should match your intuition from any language: string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. The borrow checker approves of this code; it will compile and print The longest string is long string is long when run:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```
Listing 10-24: Using the longest function with references to String values that have different concrete lifetimes

Next, let’s try an example that will show that the lifetime of the reference in result must be the smaller lifetime of the two arguments. We’ll move the declaration of the result variable outside the inner scope, but leave the assignment of the value to the result variable inside the scope with string2. Next, we’ll move the println! that uses result outside of the inner scope, after it has ended. The code in Listing 10-25 will not compile:

Filename: src/main.rs

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```
Listing 10-25: Attempting to use result after string2 has gone out of scope won’t compile

If we try to compile this, we’ll get this error:

```
error: `string2` does not live long enough
   |
6  |         result = longest(string1.as_str(), string2.as_str());
   |                                            ------- borrow occurs here
7  |     }
   |     ^ `string2` dropped here while still borrowed
8  |     println!("The longest string is {}", result);
9  | }
   | - borrowed value needs to live until here
```
The error is saying that in order for result to be valid for the println!, string2 would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values with the same lifetime parameter, 'a.

We can look at this code as humans and see that string1 is longer, and therefore result will contain a reference to string1. Because string1 has not gone out of scope yet, a reference to string1 will still be valid for the println!. However, what we’ve told Rust with the lifetime parameters is that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. Therefore, the borrow checker disallows the code in Listing 10-25 as possibly having an invalid reference.

Try designing some more experiments that vary the values and lifetimes of the references passed in to the longest function and how the returned reference is used. Make hypotheses about whether your experiments will pass the borrow checker or not before you compile, then check to see if you’re right!

Thinking in Terms of Lifetimes
The exact way to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the longest function to always return the first argument rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. This code compiles:

Filename: src/main.rs


```
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```
In this example, we’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, since the lifetime of y does not have any relationship with the lifetime of x or the return value.

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter of one of the arguments. If the reference returned does not refer to one of the arguments, the only other possibility is that it refers to a value created within this function, which would be a dangling reference since the value will go out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile:

Filename: src/main.rs

```
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
```
Even though we’ve specified a lifetime parameter 'a for the return type, this implementation fails to compile because the return value lifetime is not related to the lifetime of the parameters at all. Here’s the error message we get:

```
error: `result` does not live long enough
  |
3 |     result.as_str()
  |     ^^^^^^ does not live long enough
4 | }
  | - borrowed value only lives until here
  |
note: borrowed value must be valid for the lifetime 'a as defined on the block
at 1:44...
  |
1 | fn longest<'a>(x: &str, y: &str) -> &'a str {
  |                                             ^
```
The problem is that result will go out of scope and get cleaned up at the end of the longest function, and we’re trying to return a reference to result from the function. There’s no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference. In this case, the best fix would be to return an owned data type rather than a reference so that the calling function is then responsible for cleaning up the value.

Ultimately, lifetime syntax is about connecting the lifetimes of various arguments and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

Lifetime Annotations in Struct Definitions
Up until now, we’ve only defined structs to hold owned types. It is possible for structs to hold references, but we need to add a lifetime annotation on every reference in the struct’s definition. Listing 10-26 has a struct named ImportantExcerpt that holds a string slice:

Filename: src/main.rs

```
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```
Listing 10-26: A struct that holds a reference, so its definition needs a lifetime annotation

This struct has one field, part, that holds a string slice, which is a reference. Just like with generic data types, we have to declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so that we can use the lifetime parameter in the body of the struct definition.

The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel.

### Lifetime Elision
In this section, we’ve learned that every reference has a lifetime, and we need to specify lifetime parameters for functions or structs that use references. However, in Chapter 4 we had a function in the “String Slices” section, shown again in Listing 10-27, that compiled without lifetime annotations:

Filename: src/lib.rs

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```
Listing 10-27: A function we defined in Chapter 4 that compiled without lifetime annotations, even though the parameter and return type are references

The reason this function compiles without lifetime annotations is historical: in early versions of pre-1.0 Rust, this indeed wouldn’t have compiled. Every reference needed an explicit lifetime. At that time, the function signature would have been written like this:


`fn first_word<'a>(s: &'a str) -> &'a str {`
After writing a lot of Rust code, the Rust team found that Rust programmers were typing the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The Rust team then programmed these patterns into the Rust compiler’s code so that the borrow checker can infer the lifetimes in these situations without forcing the programmer to explicitly add the annotations.

We mention this piece of Rust history because it’s entirely possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; the rules are a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

The elision rules don’t provide full inference: if Rust deterministically applies the rules but there’s still ambiguity as to what lifetimes the references have, it won’t guess what the lifetime of the remaining references should be. In this case, the compiler will give you an error that can be resolved by adding the lifetime annotations that correspond to your intentions for how the references relate to each other.

First, some definitions: Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

Now, on to the rules that the compiler uses to figure out what lifetimes references have when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second two rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references that it can’t figure out lifetimes for, the compiler will stop with an error.

Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32), a function with two arguments gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32), and so on.

If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32.`

If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, then the lifetime of self is assigned to all output lifetime parameters. This makes writing methods much nicer.

Let’s pretend we’re the compiler and apply these rules to figure out what the lifetimes of the references in the signature of the first_word function in Listing 10-27 are. The signature starts without any lifetimes associated with the references:


`fn first_word(s: &str) -> &str {`
Then we (as the compiler) apply the first rule, which says each parameter gets its own lifetime. We’re going to call it 'a as usual, so now the signature is:


`fn first_word<'a>(s: &'a str) -> &str {`
On to the second rule, which applies because there is exactly one input lifetime. The second rule says the lifetime of the one input parameter gets assigned to the output lifetime, so now the signature is:


`fn first_word<'a>(s: &'a str) -> &'a str {`
Now all the references in this function signature have lifetimes, and the compiler can continue its analysis without needing the programmer to annotate the lifetimes in this function signature.

Let’s do another example, this time with the longest function that had no lifetime parameters when we started working with in Listing 10-22:


`fn longest(x: &str, y: &str) -> &str {`
Pretending we’re the compiler again, let’s apply the first rule: each parameter gets its own lifetime. This time we have two parameters, so we have two lifetimes:


`fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {`
Looking at the second rule, it doesn’t apply since there is more than one input lifetime. Looking at the third rule, this also does not apply because this is a function rather than a method, so none of the parameters are self. So we’re out of rules, but we haven’t figured out what the return type’s lifetime is. This is why we got an error trying to compile the code from Listing 10-22: the compiler worked through the lifetime elision rules it knows, but still can’t figure out all the lifetimes of the references in the signature.

Because the third rule only really applies in method signatures, let’s look at lifetimes in that context now, and see why the third rule means we don’t have to annotate lifetimes in method signatures very often.

Lifetime Annotations in Method Definitions
When we implement methods on a struct with lifetimes, the syntax is again the same as that of generic type parameters that we showed in Listing 10-11: the place that lifetime parameters are declared and used depends on whether the lifetime parameter is related to the struct fields or the method arguments and return values.

Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, since those lifetimes are part of the struct’s type.

In method signatures inside the impl block, references might be tied to the lifetime of references in the struct’s fields, or they might be independent. In addition, the lifetime elision rules often make it so that lifetime annotations aren’t necessary in method signatures. Let’s look at some examples using the struct named ImportantExcerpt that we defined in Listing 10-26.

First, here’s a method named level. The only parameter is a reference to self, and the return value is just an i32, not a reference to anything:


```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
The lifetime parameter declaration after impl and use after the type name is required, but we’re not required to annotate the lifetime of the reference to self because of the first elision rule.

Here’s an example where the third lifetime elision rule applies:


```
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.

### The Static Lifetime
There is one special lifetime we need to discuss: 'static. The 'static lifetime is the entire duration of the program. All string literals have the 'static lifetime, which we can choose to annotate as follows:



`let s: &'static str = "I have a static lifetime.";`
The text of this string is stored directly in the binary of your program and the binary of your program is always available. Therefore, the lifetime of all string literals is 'static.

You may see suggestions to use the 'static lifetime in error message help text, but before specifying 'static as the lifetime for a reference, think about whether the reference you have is one that actually lives the entire lifetime of your program or not (or even if you want it to live that long, if it could). Most of the time, the problem in the code is an attempt to create a dangling reference or a mismatch of the available lifetimes, and the solution is fixing those problems, not specifying the 'static lifetime.

### Generic Type Parameters, Trait Bounds, and Lifetimes Together
Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!


```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
This is the longest function from Listing 10-23 that returns the longest of two string slices, but with an extra argument named ann. The type of ann is the generic type T, which may be filled in by any type that implements the Display trait as specified by the where clause. This extra argument will be printed out before the function compares the lengths of the string slices, which is why the Display trait bound is necessary. Because lifetimes are a type of generic, the declarations of both the lifetime parameter 'a and the generic type parameter T go in the same list within the angle brackets after the function name.

### Summary
We covered a lot in this chapter! Now that you know about generic type parameters, traits and trait bounds, and generic lifetime parameters, you’re ready to write code that isn’t duplicated but can be used in many different situations. Generic type parameters mean the code can be applied to different types. Traits and trait bounds ensure that even though the types are generic, those types will have the behavior the code needs. Relationships between the lifetimes of references specified by lifetime annotations ensure that this flexible code won’t have any dangling references. And all of this happens at compile time so that run-time performance isn’t affected!

Believe it or not, there’s even more to learn in these areas: Chapter 17 will discuss trait objects, which are another way to use traits. Chapter 19 will be covering more complex scenarios involving lifetime annotations. Chapter 20 will get to some advanced type system features. Up next, though, let’s talk about how to write tests in Rust so that we can make sure our code using all these features is working the way we want it to!

# Writing Automated Tests
In his 1972 essay, “The Humble Programmer,” Edsger W. Dijkstra said that “Program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence.” That doesn’t mean we shouldn’t try to test as much as we can! Correctness in our programs is the extent to which our code does what we intend it to do. Rust is a programming language designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove. Rust’s type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness. As such, Rust includes support for writing automated software tests within the language.

As an example, say we write a function called add_two that adds two to whatever number is passed to it. This function’s signature accepts an integer as a parameter and returns an integer as a result. When we implement and compile that function, Rust does all the type checking and borrow checking that you’ve learned so far to ensure that, for instance, we aren’t passing a String value or an invalid reference to this function. But Rust can’t check that this function will do precisely what we intend, which is return the parameter plus two rather than, say, the parameter plus 10 or the parameter minus 50! That’s where tests come in.

We can write tests that assert, for example, that when we pass 3 to the add_two function, the returned value is 5. We can run these tests whenever we make changes to our code to make sure any existing correct behavior has not changed.

Testing is a complex skill: although we can’t cover every detail about how to write good tests in one chapter, we’ll discuss the mechanics of Rust’s testing facilities. We’ll talk about the annotations and macros available to you when writing your tests, the default behavior and options provided for running your tests, and how to organize tests into unit tests and integration tests.

## How to Write Tests
Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

Set up any needed data or state
Run the code we want to test
Assert the results are what we expect
Let’s look at the features Rust provides specifically for writing tests that take these actions, which include the test attribute, a few macros, and the should_panic attribute.

### The Anatomy of a Test Function
At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code; one example is the derive attribute we used with structs in Chapter 5. To change a function into a test function, we add #[test] on the line before fn. When we run our tests with the cargo test command, Rust builds a test runner binary that runs the functions annotated with the test attribute and reports on whether each test function passes or fails.

In Chapter 7, we saw that when we make a new library project with Cargo, a test module with a test function in it is automatically generated for us. This module helps us start writing our tests so we don’t have to look up the exact structure and syntax of test functions every time we start a new project. We can add as many additional test functions and as many test modules as we want!

We’ll explore some aspects of how tests work by experimenting with the template test generated for us without actually testing any code. Then we’ll write some real-world tests that call some code that we’ve written and assert that its behavior is correct.

Let’s create a new library project called adder:

```
$ cargo new adder
     Created library `adder` project
$ cd adder
```
The contents of the src/lib.rs file in your adder library should look like Listing 11-1:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
Listing 11-1: The test module and function generated automatically by cargo new

For now, let’s ignore the top two lines and focus on the function to see how it works. Note the #[test] annotation before the fn line: this attribute indicates this is a test function, so the test runner knows to treat this function as a test. We could also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we need to indicate which functions are tests by using the #[test] attribute.

The function body uses the assert_eq! macro to assert that 2 + 2 equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.

The cargo test command runs all tests in our project, as shown in Listing 11-2:

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.22 secs
     Running target/debug/deps/adder-ce99bcc2479f4607

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Listing 11-2: The output from running the automatically generated test

Cargo compiled and ran the test. After the Compiling, Finished, and Running lines is the line running 1 test. The next line shows the name of the generated test function, called it_works, and the result of running that test, ok. The overall summary of running the tests appears next. The text test result: ok. means that all the tests passed, and the portion that reads 1 passed; 0 failed totals the number of tests that passed or failed.

Because we don’t have any tests we’ve marked as ignored, the summary shows 0 ignored. We also haven’t filtered the tests being run, so the end of the summary shows 0 filtered out. We’ll talk about ignoring and filtering out tests in the next section, “Controlling How Tests Are Run.”

The 0 measured statistic is for benchmark tests that measure performance. Benchmark tests are, as of this writing, only available in nightly Rust. See Chapter 1 for more information about nightly Rust.

The next part of the test output, which starts with Doc-tests adder, is for the results of any documentation tests. We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation. This feature helps us keep our docs and our code in sync! We’ll discuss how to write documentation tests in the “Documentation Comments” section of Chapter 14. For now, we’ll ignore the Doc-tests output.

Let’s change the name of our test to see how that changes the test output. Change the it_works function to a different name, such as exploration, like so:

Filename: `src/lib.rs`


```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```
Then run cargo test again. The output now shows exploration instead of it_works:

```
running 1 test
test tests::exploration ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Let’s add another test, but this time we’ll make a test that fails! Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. We talked about the simplest way to cause a panic in Chapter 9, which is to call the panic! macro. Enter the new test, another, so your src/lib.rs file looks like Listing 11-3:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```
Listing 11-3: Adding a second test that will fail because we call the panic! macro

Run the tests again using cargo test. The output should look like Listing 11-4, which shows that our exploration test passed and another failed:

```
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
    thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:10:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed
```
Listing 11-4: Test results when one test passes and one test fails

Instead of ok, the line test tests::another shows FAILED. Two new sections appear between the individual results and the summary: the first section displays the detailed reason for each test failure. In this case, another failed because it panicked at 'Make this test fail', which happened on line 10 in the src/lib.rs file. The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output. We can use the name of a failing test to run just that test to more easily debug it; we’ll talk more about ways to run tests in the “Controlling How Tests Are Run” section.

The summary line displays at the end: overall, our test result is FAILED. We had one test pass and one test fail.

Now that you’ve seen what the test results look like in different scenarios, let’s look at some macros other than panic! that are useful in tests.

### Checking Results with the assert! Macro
The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true. We give the assert! macro an argument that evaluates to a Boolean. If the value is true, assert! does nothing and the test passes. If the value is false, the assert! macro calls the panic! macro, which causes the test to fail. Using the assert! macro helps us check that our code is functioning in the way we intend.

In Chapter 5, Listing 5-15, we used a Rectangle struct and a can_hold method, which are repeated here in Listing 11-5. Let’s put this code in the src/lib.rs file and write some tests for it using the assert! macro.

Filename: src/lib.rs

```

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}
```
Listing 11-5: Using the Rectangle struct and its can_hold method from Chapter 5

The can_hold method returns a Boolean, which means it’s a perfect use case for the assert! macro. In Listing 11-6, we write a test that exercises the can_hold method by creating a Rectangle instance that has a length of 8 and a width of 7, and asserting that it can hold another Rectangle instance that has a length of 5 and a width of 1:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```
Listing 11-6: A test for can_hold that checks that a larger rectangle can indeed hold a smaller rectangle

Note that we’ve added a new line inside the tests module: the use super::`*`; line. The tests module is a regular module that follows the usual visibility rules we covered in Chapter 7 in the “Privacy Rules” section. Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. We use a glob here so anything we define in the outer module is available to this tests module.

We’ve named our test larger_can_hold_smaller, and we’ve created the two Rectangle instances that we need. Then we called the assert! macro and passed it the result of calling larger.can_hold(&smaller). This expression is supposed to return true, so our test should pass. Let’s find out!

```
running 1 test
test tests::larger_can_hold_smaller ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
It does pass! Let’s add another test, this time asserting that a smaller rectangle cannot hold a larger rectangle:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        // --snip--
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}
```
Because the correct result of the can_hold function in this case is false, we need to negate that result before we pass it to the assert! macro. As a result, our test will pass if can_hold returns false:

```
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Two tests that pass! Now let’s see what happens to our test results when we introduce a bug in our code. Let’s change the implementation of the can_hold method by replacing the greater-than sign with a less-than sign when it compares the lengths:


```
// --snip--

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length < other.length && self.width > other.width
    }
}
```
Running the tests now produces the following:

```
running 2 tests
test tests::smaller_cannot_hold_larger ... ok
test tests::larger_can_hold_smaller ... FAILED

failures:

---- tests::larger_can_hold_smaller stdout ----
    thread 'tests::larger_can_hold_smaller' panicked at 'assertion failed:
    larger.can_hold(&smaller)', src/lib.rs:22:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::larger_can_hold_smaller

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
Our tests caught the bug! Because larger.length is 8 and smaller.length is 5, the comparison of the lengths in can_hold now returns false: 8 is not less than 5.

### Testing Equality with the assert_eq! and assert_ne! Macros
A common way to test functionality is to compare the result of the code under test to the value we expect the code to return to make sure they’re equal. We could do this using the assert! macro and passing it an expression using the == operator. However, this is such a common test that the standard library provides a pair of macros—assert_eq! and assert_ne!—to perform this test more conveniently. These macros compare two arguments for equality or inequality, respectively. They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed; conversely, the assert! macro only indicates that it got a false value for the == expression, not the values that lead to the false value.

In Listing 11-7, we write a function named add_two that adds 2 to its parameter and returns the result. Then we test this function using the assert_eq! macro.

Filename: src/lib.rs


```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}
```
Listing 11-7: Testing the function add_two using the assert_eq! macro

Let’s check that it passes!

```
running 1 test
test tests::it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
The first argument we gave to the assert_eq! macro, 4, is equal to the result of calling add_two(2). The line for this test is test tests::it_adds_two ... ok, and the ok text indicates that our test passed!

Let’s introduce a bug into our code to see what it looks like when a test that uses assert_eq! fails. Change the implementation of the add_two function to instead add 3:


```
pub fn add_two(a: i32) -> i32 {
    a + 3
}
Run the tests again:


running 1 test
test tests::it_adds_two ... FAILED

failures:

---- tests::it_adds_two stdout ----
        thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::it_adds_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
Our test caught the bug! The it_adds_two test failed, displaying the message assertion failed: `(left == right)` and showing that left was 4 and right was 5. This message is useful and helps us start debugging: it means the left argument to assert_eq! was 4, but the right argument, where we had add_two(2), was 5.

Note that in some languages and test frameworks, the parameters to the functions that assert two values are equal are called expected and actual, and the order in which we specify the arguments matters. However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value that the code under test produces doesn’t matter. We could write the assertion in this test as assert_eq!(add_two(2), 4), which would result in a failure message that displays assertion failed: `(left == right)` and that left was 5 and right was 4.

The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely won’t be if our code is functioning as we intend. For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively. When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits. All the primitive types and most of the standard library types implement these traits. For structs and enums that you define, you’ll need to implement PartialEq to assert that values of those types are equal or not equal. You’ll need to implement Debug to print out the values when the assertion fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition. See Appendix C for more details about these and other derivable traits.

### Adding Custom Failure Messages
We can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros. Any arguments specified after the one required argument to assert! or the two required arguments to assert_eq! and assert_ne! are passed along to the format! macro (discussed in Chapter 8 in the “Concatenation with the + Operator or the format! Macro” section), so you can pass a format string that contains {} placeholders and values to go in those placeholders. Custom messages are useful to document what an assertion means; when a test fails, we’ll have a better idea of what the problem is with the code.

For example, let’s say we have a function that greets people by name, and we want to test that the name we pass into the function appears in the output:

Filename: `src/lib.rs`


```
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}
```
The requirements for this program haven’t been agreed upon yet, and we’re pretty sure the Hello text at the beginning of the greeting will change. We decided we don’t want to have to update the test for the name when that happens, so instead of checking for exact equality to the value returned from the greeting function, we’ll just assert that the output contains the text of the input parameter.

Let’s introduce a bug into this code by changing greeting to not include name to see what this test failure looks like:


```
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}
```
Running this test produces the following:

```
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
        thread 'tests::greeting_contains_name' panicked at 'assertion failed:
result.contains("Carol")', src/lib.rs:12:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.

failures:
    tests::greeting_contains_name
```
This result just indicates that the assertion failed and which line the assertion is on. A more useful failure message in this case would print the value we got from the greeting function. Let’s change the test function, giving it a custom failure message made from a format string with a placeholder filled in with the actual value we got from the greeting function:

```
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
Now when we run the test, we’ll get a more informative error message:


---- tests::greeting_contains_name stdout ----
        thread 'tests::greeting_contains_name' panicked at 'Greeting did not
contain name, value was `Hello!`', src/lib.rs:12:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.

### Checking for Panics with should_panic
In addition to checking that our code returns the correct values we expect, it’s also important to check that our code handles error conditions as we expect. For example, consider the Guess type that we created in Chapter 9, Listing 9-9. Other code that uses Guess depends on the guarantee that Guess instances will only contain values between 1 and 100. We can write a test that ensures that attempting to create a Guess instance with a value outside that range panics.

We do this by adding another attribute, should_panic, to our test function. This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.

Listing 11-8 shows a test that checks that the error conditions of Guess::new happen when we expect:

Filename: src/lib.rs


```
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
Listing 11-8: Testing that a condition will cause a panic!

We place the #[should_panic] attribute after the #[test] attribute and before the test function it applies to. Let’s look at the result when this test passes:

```
running 1 test
test tests::greater_than_100 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Looks good! Now let’s introduce a bug in our code by removing the condition that the new function will panic if the value is greater than 100:


```
// --snip--

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}
```
When we run the test in Listing 11-8, it will fail:

```
running 1 test
test tests::greater_than_100 ... FAILED

failures:

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
We don’t get a very helpful message in this case, but when we look at the test function, we see that it’s annotated with #[should_panic]. The failure we got means that the code in the test function did not cause a panic.

Tests that use should_panic can be imprecise because they only indicate that the code has caused some panic. A should_panic test would pass even if the test panics for a different reason than the one we were expecting to happen. To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text. For example, consider the modified code for Guess in Listing 11-9 where the new function panics with different messages depending on whether the value was too small or too large:

Filename: src/lib.rs

```

// --snip--

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
Listing 11-9: Testing that a condition will cause a panic! with a particular panic message

This test will pass because the value we put in the should_panic attribute’s expected parameter is a substring of the message that the Guess::new function panics with. We could have specified the entire panic message that we expect, which in this case would be Guess value must be less than or equal to 100, got 200. What you choose to specify in the expected parameter for should_panic depends on how much of the panic message is unique or dynamic and how precise you want your test to be. In this case, a substring of the panic message is enough to ensure that the code in the test function executes the else if value > 100 case.

To see what happens when a should_panic test with an expected message fails, let’s again introduce a bug into our code by swapping the bodies of the if value < 1 and the else if value > 100 blocks:

```
if value < 1 {
    panic!("Guess value must be less than or equal to 100, got {}.", value);
} else if value > 100 {
    panic!("Guess value must be greater than or equal to 1, got {}.", value);
}
```
This time when we run the should_panic test, it will fail:

```
running 1 test
test tests::greater_than_100 ... FAILED

failures:

---- tests::greater_than_100 stdout ----
        thread 'tests::greater_than_100' panicked at 'Guess value must be
greater than or equal to 1, got 200.', src/lib.rs:11:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
note: Panic did not include expected string 'Guess value must be less than or
equal to 100'

failures:
    tests::greater_than_100

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
The failure message indicates that this test did indeed panic as we expected, but the panic message did not include the expected string 'Guess value must be less than or equal to 100'. The panic message that we did get in this case was Guess value must be greater than or equal to 1, got 200. Now we can start figuring out where our bug is!

Now that you know several ways to write tests, let’s look at what is happening when we run our tests and explore the different options we can use with cargo test.

## Test Organization
As mentioned at the start of the chapter, testing is a complex discipline, and different people use different terminology and organization. The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to separately and together.

### Unit Tests
The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly pinpoint where code is and isn’t working as expected. We put unit tests in the src directory in each file with the code that they’re testing. The convention is that we create a module named tests in each file to contain the test functions, and we annotate the module with cfg(test).

### The Tests Module and #[cfg(test)]
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when we run cargo test, but not when we run cargo build. This saves compile time when we only want to build the library and saves space in the resulting compiled artifact because the tests are not included. You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as the code, we use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.

Recall that when we generated the new adder project in the first section of this chapter, Cargo generated this code for us:

Filename: src/lib.rs


```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
This code is the automatically generated test module. The attribute cfg stands for configuration and tells Rust that the following item should only be included given a certain configuration option. In this case, the configuration option is test, which is provided by Rust for compiling and running tests. By using the cfg attribute, Cargo compiles our test code only if we actively run the tests with cargo test. This includes any helper functions that might be within this module, in addition to the functions annotated with #[test].

Testing Private Functions
There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions. Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions. Consider the code in Listing 11-12 with the private function internal_adder:

Filename: src/lib.rs


```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```
Listing 11-12: Testing a private function

Note that the internal_adder function is not marked as pub, but because tests are just Rust code and the tests module is just another module, we can import and call internal_adder in a test just fine. If you don’t think private functions should be tested, there’s nothing in Rust that will compel you to do so.

### Integration Tests
In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. Their purpose is to test that many parts of your library work together correctly. Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. To create integration tests, you first need a tests directory.

### The tests Directory
We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want to in this directory, and Cargo will compile each of the files as an individual crate.

Let’s create an integration test. With the code in Listing 11-12 still in the src/lib.rs file, make a tests directory, create a new file named tests/integration_test.rs, and enter the code in Listing 11-13:

Filename: tests/integration_test.rs

```
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
Listing 11-13: An integration test of a function in the adder crate

We’ve added extern crate adder at the top of the code, which we didn’t need in the unit tests. The reason is that each test in the tests directory is a separate crate, so we need to import our library into each of them.

We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test. Run cargo test now:

```
$ cargo test
   Compiling adder v0.1.0 (file:///projects/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31 secs
     Running target/debug/deps/adder-abcabcabc

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-ce99bcc2479f4607

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
The three sections of output include the unit tests, the integration test, and the doc tests. The first section for the unit tests is the same as we’ve been seeing: one line for each unit test (one named internal that we added in Listing 11-12) and then a summary line for the unit tests.

The integration tests section starts with the line Running target/debug/deps/integration-test-ce99bcc2479f4607 (the hash at the end of your output will be different). Next, there is a line for each test function in that integration test and a summary line for the results of the integration test just before the Doc-tests adder section starts.

Recall that adding more unit test functions in any src file adds more test result lines to the unit tests section. Adding more test functions to the integration test file we created adds more lines to that file’s section. Each integration test file has its own section, so if we add more files in the tests directory, there will be more integration test sections.

We can still run a particular integration test function by specifying the test function’s name as an argument to cargo test. To run all the tests in a particular integration test file, use the --test argument of cargo test followed by the name of the file:

```
$ cargo test --test integration_test
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running target/debug/integration_test-952a27e0126bb565

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
This command runs only the tests in the tests/integration_test.rs file.

### Submodules in Integration Tests
As you add more integration tests, you might want to make more than one file in the tests directory to help organize them; for example, you can group the test functions by the functionality they’re testing. As mentioned earlier, each file in the tests directory is compiled as its own separate crate.

Treating each integration test file as its own crate is useful to create separate scopes that are more like the way end users will be using your crate. However, this means files in the tests directory don’t share the same behavior as files in src do, which you learned in Chapter 7 regarding how to separate code into modules and files.

The different behavior of files in the tests directory is most noticeable when you have a set of helper functions that would be useful in multiple integration test files and you try to follow the steps in the “Moving Modules to Other Files” section of Chapter 7 to extract them into a common module. For example, if we create tests/common.rs and place a function named setup in it, we can add some code to setup that we want to call from multiple test functions in multiple test files:

Filename: tests/common.rs


```
pub fn setup() {
    // setup code specific to your library's tests would go here
}
```
When we run the tests again, we’ll see a new section in the test output for the common.rs file, even though this file doesn’t contain any test functions, nor did we call the setup function from anywhere:

```
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/common-b8b07b6f1be2db70

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/debug/deps/integration_test-d993c68b431d39df

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Having common appear in the test results with running 0 tests displayed for it is not what we wanted. We just wanted to share some code with the other integration test files.

To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs. In the “Rules of Module Filesystems” section of Chapter 7, we used the naming convention module_name/mod.rs for files of modules that have submodules, and we don’t have submodules for common here, but naming the file this way tells Rust not to treat the common module as an integration test file. When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear. Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:

Filename: tests/integration_test.rs

```
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```
Note that the mod common; declaration is the same as the module declarations we demonstrated in Listing 7-4. Then in the test function, we can call the common::setup() function.

### Integration Tests for Binary Crates
If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and use extern crate to import functions defined in the src/main.rs file. Only library crates expose functions that other crates can call and use; binary crates are meant to be run on their own.

This is one of the reasons Rust projects that provide a binary have a straightforward src/main.rs file that calls logic that lives in the src/lib.rs file. Using that structure, integration tests can test the library crate by using extern crate to exercise the important functionality. If the important functionality works, the small amount of code in the src/main.rs file will work as well, and that small amount of code doesn’t need to be tested.

### Summary
Rust’s testing features provide a way to specify how code should function to ensure it continues to work as we expect even as we make changes. Unit tests exercise different parts of a library separately and can test private implementation details. Integration tests check that many parts of the library work together correctly, and they use the library’s public API to test the code in the same way external code will use it. Even though Rust’s type system and ownership rules help prevent some kinds of bugs, tests are still important to help reduce logic bugs having to do with how your code is expected to behave.

Let’s combine the knowledge you learned in this chapter and in previous chapters and work on a project in the next chapter!

## An I/O Project: Building a Command Line Program
This chapter is a recap of the many skills you’ve learned so far and an exploration of a few more standard library features. We’ll build a command line tool that interacts with file and command line input/output to practice some of the Rust concepts you now have under your belt.

Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command line tools, so for our project, we’ll make our own version of the classic command line tool grep (globally search a regular expression and print). In the simplest use case, grep searches a specified file for a specified string. To do so, grep takes as its arguments a filename and a string, and then reads the file and finds lines in that file that contain the string argument. It then prints those lines.

Along the way, we’ll show how to make our command line tool use features of the terminal that many command line tools use. We’ll read the value of an environment variable to allow the user to configure the behavior of our tool. We’ll also print to the standard error console stream (stderr) instead of standard output (stdout), so, for example, the user can redirect successful output to a file while still seeing error messages onscreen.

One Rust community member, Andrew Gallant, has already created a fully featured, very fast version of grep, called ripgrep. By comparison, our version of grep will be fairly simple, but this chapter will give you some of the background knowledge you need to understand a real-world project like ripgrep.

Our grep project will combine a number of concepts you’ve learned so far:
- Organizing code (using what you learned in modules, Chapter 7)
- Using vectors and strings (collections, Chapter 8)
- Handling errors (Chapter 9)
- Using traits and lifetimes where appropriate (Chapter 10)
- Writing tests (Chapter 11)
We’ll also briefly introduce closures, iterators, and trait objects, which Chapters 13 and 17 will cover in detail.

### Accepting Command Line Arguments
Let’s create a new project with, as always, cargo new. We’ll call our project minigrep to distinguish it from the grep tool that you might already have on your system.

```
$ cargo new --bin minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```
The first task is to make minigrep accept its two command line arguments: the filename and a string to search for. That is, we want to be able to run our program with cargo run, a string to search for, and a path to a file to search in, like so:


`$ cargo run searchstring example-filename.txt`
Right now, the program generated by cargo new cannot process arguments we give it. However, some existing libraries on Crates.io can help us with writing a program that accepts command line arguments, but because you’re just learning this concept, let’s implement this capability ourselves.

### Reading the Argument Values
To make sure minigrep is able to read the values of command line arguments we pass to it, we’ll need a function provided in Rust’s standard library, which is std::env::args. This function returns an iterator of the command line arguments that were given to minigrep. We haven’t discussed iterators yet (we’ll cover them fully in Chapter 13), but for now you only need to know two details about iterators: iterators produce a series of values, and we can call the collect function on an iterator to turn it into a collection, such as a vector, containing all the elements the iterator produces.

Use the code in Listing 12-1 to allow your minigrep program to read any command line arguments passed to it and then collect the values into a vector:

Filename: src/main.rs

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```
Listing 12-1: Collecting the command line arguments into a vector and printing them

First, we bring the std::env module into scope with a use statement so we can use its args function. Notice that the std::env::args function is nested in two levels of modules. As we discussed in Chapter 7, in cases where the desired function is nested in more than one module, it’s conventional to bring the parent module into scope rather than the function. As a result, we can easily use other functions from std::env. It’s also less ambiguous than adding use std::env::args and then calling the function with just args because args might easily be mistaken for a function that’s defined in the current module.

### The args Function and Invalid Unicode
Note that std::env::args will panic if any argument contains invalid Unicode. If your program needs to accept arguments containing invalid Unicode, use std::env::args_os instead. That function returns OsString values instead of String values. We’ve chosen to use std::env::args here for simplicity because OsString values differ per platform and are more complex to work with than String values.

On the first line of main, we call env::args, and immediately use collect to turn the iterator into a vector containing all the values produced by the iterator. We can use the collect function to create many kinds of collections, so we explicitly annotate the type of args to specify that we want a vector of strings. Although we very rarely need to annotate types in Rust, collect is one function you do often need to annotate because Rust isn’t able to infer the kind of collection you want.

Finally, we print the vector using the debug formatter, :?. Let’s try running the code with no arguments, and then with two arguments:

```
$ cargo run
--snip--
["target/debug/minigrep"]

$ cargo run needle haystack
--snip--
["target/debug/minigrep", "needle", "haystack"]
```
Notice that the first value in the vector is "target/debug/minigrep", which is the name of our binary. This matches the behavior of the arguments list in C, letting programs use the name by which they were invoked in their execution. It’s often convenient to have access to the program name in case we want to print it in messages or change behavior of the program based on what command line alias was used to invoke the program. But for the purposes of this chapter, we’ll ignore it and save only the two arguments we need.

### Saving the Argument Values in Variables
Printing the value of the vector of arguments illustrated that the program is able to access the values specified as command line arguments. Now we need to save the values of the two arguments in variables so we can use the values throughout the rest of the program. We do that in Listing 12-2:

Filename: src/main.rs

```
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```
Listing 12-2: Creating variables to hold the query argument and filename argument

As we saw when we printed the vector, the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1. The first argument minigrep takes is the string we’re searching for, so we put a reference to the first argument in the variable query. The second argument will be the filename, so we put a reference to the second argument in the variable filename.

We temporarily print the values of these variables to prove that the code is working as we intend. Let’s run this program again with the arguments test and sample.txt:

```
$ cargo run test sample.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep test sample.txt`
Searching for test
In file sample.txt
```
Great, the program is working! The values of the arguments we need are being saved into the right variables. Later we’ll add some error handling to deal with certain potential erroneous situations, such as when the user provides no arguments; for now, we’ll ignore that situation and work on adding file reading capabilities instead.

### Reading a File
Now we’ll add functionality to read the file that is specified in the filename command line argument. First, we need a sample file to test it with: the best kind of file to use to make sure minigrep is working is one with a small amount of text over multiple lines with some repeated words. Listing 12-3 has an Emily Dickinson poem that will work well! Create a file called poem.txt at the root level of your project, and enter the poem “I’m Nobody! Who are you?”

Filename: poem.txt

```
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
Listing 12-3: A poem by Emily Dickinson will make a good test case.

With the text in place, edit src/main.rs and add code to open the file, as shown in Listing 12-4:

Filename: src/main.rs

```
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```
Listing 12-4: Reading the contents of the file specified by the second argument

First, we add some more use statements to bring in relevant parts of the standard library: we need std::fs::File to handle files, and std::io::prelude::* contains various useful traits for doing I/O, including file I/O. In the same way that Rust has a general prelude that brings certain types and functions into scope automatically, the std::io module has its own prelude of common types and functions you’ll need when working with I/O. Unlike the default prelude, we must explicitly add a use statement for the prelude from std::io.

In main, we’ve added three statements: first, we get a mutable handle to the file by calling the File::open function and passing it the value of the filename variable. Second, we create a variable called contents and set it to a mutable, empty String. This will hold the content of the file after we read it in. Third, we call read_to_string on our file handle and pass a mutable reference to contents as an argument.

After those lines, we’ve again added a temporary println! statement that prints the value of contents after the file is read, so we can check that the program is working so far.

Let’s run this code with any string as the first command line argument (because we haven’t implemented the searching part yet) and the poem.txt file as the second argument:

```
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I’m nobody! Who are you?
Are you nobody, too?
Then there’s a pair of us — don’t tell!
They’d banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
Great! The code read and then printed the content of the file. But the code has a few flaws. The main function has multiple responsibilities: generally, functions are clearer and easier to maintain if each function is responsible for only one idea. The other problem is that we’re not handling errors as well as we could be. The program is still small so these flaws aren’t a big problem, but as the program grows, it will be harder to fix them cleanly. It’s good practice to begin refactoring early on when developing a program, because it’s much easier to refactor smaller amounts of code. We’ll do that next.

### Refactoring to Improve Modularity and Error Handling
To improve our program, we’ll fix four problems that have to do with the program’s structure and how it’s handling potential errors.

First, our main function now performs two tasks: it parses arguments and opens files. For such a small function, this isn’t a major problem. However, if we continue to grow our program inside main, the number of separate tasks the main function handles will increase. As a function gains responsibilities, it becomes more difficult to reason about, harder to test, and harder to change without breaking one of its parts. It’s best to separate functionality so each function is responsible for one task.

This issue also ties into the second problem: although query and filename are configuration variables to our program, variables like f and contents are used to perform the program’s logic. The longer main becomes, the more variables we’ll need to bring into scope; the more variables we have in scope, the harder it will be to keep track of the purpose of each. It’s best to group the configuration variables into one structure to make their purpose clear.

The third problem is that we’ve used expect to print an error message when opening the file fails, but the error message just prints file not found. Opening a file can fail in a number of ways besides the file being missing: for example, the file might exist, but we might not have permission to open it. Right now, if we’re in that situation, we’d print the file not found error message that would give the user the wrong information!

Fourth, we use expect repeatedly to handle different errors, and if the user runs our program without specifying enough arguments, they’ll get an index out of bounds error from Rust that doesn’t clearly explain the problem. It would be best if all the error handling code was in one place so future maintainers have only one place to consult in the code if the error handling logic needs to change. Having all the error handling code in one place will also ensure that we’re printing messages that will be meaningful to our end users.

Let’s address these four problems by refactoring our project.

### Separation of Concerns for Binary Projects
The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a type of guideline process for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

- Split your program into a main.rs and a lib.rs, and move your program’s logic to lib.rs.

- While your command line parsing logic is small, it can remain in main.rs.

- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to:

Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error
This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because we can’t test the main function directly, this structure lets us test all of our program’s logic by moving it into functions in lib.rs. The only code that remains in main.rs will be small enough to verify its correctness by reading it. Let’s rework our program by following this process.

Extracting the Argument Parser
We’ll extract the functionality for parsing arguments into a function that main will call to prepare for moving the command line parsing logic to src/lib.rs. Listing 12-5 shows the new start of main that calls a new function parse_config, which we’ll define in src/main.rs for the moment.

Filename: src/main.rs

````
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
````
Listing 12-5: Extracting a parse_config function from main

We’re still collecting the command line arguments into a vector, but instead of assigning the argument value at index 1 to the variable query and the argument value at index 2 to the variable filename within the main function, we pass the whole vector to the parse_config function. The parse_config function then holds the logic that determines which argument goes in which variable and passes the values back to main. We still create the query and filename variables in main, but main no longer has the responsibility of determining how the command line arguments and variables correspond.

This rework may seem like overkill for our small program, but we’re refactoring in small, incremental steps. After making this change, run the program again to verify that the argument parsing still works. It’s good to check your progress often, because that will help you identify the cause of problems when they occur.

### Grouping Configuration Values
We can take another small step to improve the parse_config function further. At the moment, we’re returning a tuple, but then we immediately break that tuple into individual parts again. This is a sign that perhaps we don’t have the right abstraction yet.

Another indicator that shows there’s room for improvement is the config part of parse_config, which implies that the two values we return are related and are both part of one configuration value. We’re not currently conveying this meaning in the structure of the data other than grouping the two values into a tuple: we could put the two values into one struct and give each of the struct fields a meaningful name. Doing so will make it easier for future maintainers of this code to understand how the different values relate to each other and what their purpose is.

Note: Some people call this anti-pattern of using primitive values when a complex type would be more appropriate primitive obsession.

Listing 12-6 shows the addition of a struct named Config defined to have fields named query and filename. We’ve also changed the parse_config function to return an instance of the Config struct and updated main to use the struct fields rather than having separate variables:

Filename: src/main.rs

```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let mut f = File::open(config.filename).expect("file not found");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```
Listing 12-6: Refactoring parse_config to return an instance of a Config struct

The signature of parse_config now indicates that it returns a Config value. In the body of parse_config, where we used to return string slices that reference String values in args, we now define Config to contain owned String values. The args variable in main is the owner of the argument values and is only letting the parse_config function borrow them, which means we’d violate Rust’s borrowing rules if Config tried to take ownership of the values in args.

We could manage the String data in a number of different ways, but the easiest, though somewhat inefficient, route is to call the clone method on the values. This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data. However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

### The Trade-Offs of Using clone
There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost. In Chapter 13, you’ll learn how to use more efficient methods in this type of situation. But for now, it’s okay to copy a few strings to continue making progress because we’ll make these copies only once, and our filename and query string are very small. It’s better to have a working program that’s a bit inefficient than to try to hyperoptimize code on your first pass. As you become more experienced with Rust, it’ll be easier to start with the most efficient solution, but for now, it’s perfectly acceptable to call clone.

We’ve updated main so it places the instance of Config returned by parse_config into a variable named config, and we updated the code that previously used the separate query and filename variables so it now uses the fields on the Config struct instead.

Now our code more clearly conveys that query and filename are related, and their purpose is to configure how the program will work. Any code that uses these values knows to find them in the config instance in the fields named for their purpose.

### Creating a Constructor for Config
So far, we’ve extracted the logic responsible for parsing the command line arguments from main and placed it in the parse_config function, which helped us to see that the query and filename values were related and that relationship should be conveyed in our code. We then added a Config struct to name the related purpose of query and filename, and to be able to return the values’ names as struct field names from the parse_config function.

So now that the purpose of the parse_config function is to create a Config instance, we can change parse_config from being a plain function to a function named new that is associated with the Config struct. Making this change will make the code more idiomatic: we can create instances of types in the standard library, such as String, by calling String::new, and by changing parse_config into a new function associated with Config, we’ll be able to create instances of Config by calling Config::new. Listing 12-7 shows the changes we need to make:

Filename: src/main.rs

```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```
Listing 12-7: Changing parse_config into Config::new

We’ve updated main where we were calling parse_config to instead call Config::new. We’ve changed the name of parse_config to new and moved it within an impl block, which associates the new function with Config. Try compiling this code again to make sure it works.

### Fixing the Error Handling
Now we’ll work on fixing our error handling. Recall that attempting to access the values in the args vector at index 1 or index 2 will cause the program to panic if the vector contains fewer than three items. Try running the program without any arguments; it will look like this:

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'index out of bounds: the len is 1
but the index is 1', src/main.rs:29:21
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
The line index out of bounds: the len is 1 but the index is 1 is an error message intended for programmers. It won’t help our end users understand what happened and what they should do instead. Let’s fix that now.

### Improving the Error Message
In Listing 12-8, we add a check in the new function that will verify that the slice is long enough before accessing index 1 and 2. If the slice isn’t long enough, the program panics and displays a better error message than the index out of bounds message:

Filename: src/main.rs

```
// --snip--
fn new(args: &[String]) -> Config {
    if args.len() < 3 {
        panic!("not enough arguments");
    }
    // --snip--
```
Listing 12-8: Adding a check for the number of arguments

This code is similar to the Guess::new function we wrote in Listing 9-9 where we called panic! when the value argument was out of the range of valid values. Instead of checking for a range of values here, we’re checking that the length of args is at least 3 and the rest of the function can operate under the assumption that this condition has been met. If args has fewer than three items, this condition will be true, and we call the panic! macro to end the program immediately.

With these extra few lines of code in new, let’s run the program without any arguments again to see what the error looks like now:

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep`
thread 'main' panicked at 'not enough arguments', src/main.rs:30:12
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```
This output is better: we now have a reasonable error message. However, we also have extraneous information we don’t want to give to our users. Perhaps using the technique we used in Listing 9-9 isn’t the best to use here: a call to panic! is more appropriate for a programming problem rather than a usage problem, as discussed in Chapter 9. Instead, we can use the other technique you learned about in Chapter 9—returning a Result that indicates either success or an error.

### Returning a Result from new Instead of Calling panic!
We can instead return a Result value that will contain a Config instance in the successful case and will describe the problem in the error case. When Config::new is communicating to main, we can use the Result type to signal there was a problem. Then we can change main to convert an Err variant into a more practical error for our users without the surrounding text about thread 'main' and RUST_BACKTRACE that a call to panic! causes.

Listing 12-9 shows the changes we need to make to the return value of Config::new and the body of the function needed to return a Result. Note that this won’t compile until we update main as well, which we’ll do in the next listing:

Filename: src/main.rs

```
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
Listing 12-9: Returning a Result from Config::new

Our new function now returns a Result with a Config instance in the success case and a &'static str in the error case. Recall from “The Static Lifetime” section in Chapter 10 that &'static str is the type of string literals, which is our error message type for now.

We’ve made two changes in the body of the new function: instead of calling panic! when the user doesn’t pass enough arguments, we now return an Err value, and we’ve wrapped the Config return value in an Ok. These changes make the function conform to its new type signature.

Returning an Err value from Config::new allows the main function to handle the Result value returned from the new function and exit the process more cleanly in the error case.

### Calling Config::new and Handling Errors
To handle the error case and print a user-friendly message, we need to update main to handle the Result being returned by Config::new, as shown in Listing 12-10. We’ll also take the responsibility of exiting the command line tool with a nonzero error code from panic! and implement it by hand. A nonzero exit status is a convention to signal to the process that called our program that the program exited with an error state.

Filename: src/main.rs

```
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
```
Listing 12-10: Exiting with an error code if creating a new Config fails

In this listing, we’ve used a method we haven’t covered before: unwrap_or_else, which is defined on Result<T, E> by the standard library. Using unwrap_or_else allows us to define some custom, non-panic! error handling. If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping. However, if the value is an Err value, this method calls the code in the closure, which is an anonymous function we define and pass as an argument to unwrap_or_else. We’ll cover closures in more detail in Chapter 13. For now, you just need to know that unwrap_or_else will pass the inner value of the Err, which in this case is the static string not enough arguments that we added in Listing 12-9, to our closure in the argument err that appears between the vertical pipes. The code in the closure can then use the err value when it runs.

We’ve added a new use line to import process from the standard library. The code in the closure that will be run in the error case is only two lines: we print the err value and then call process::exit. The process::exit function will stop the program immediately and return the number that was passed as the exit status code. This is similar to the panic!-based handling we used in Listing 12-8, but we no longer get all the extra output. Let’s try it:

```
$ cargo run
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48 secs
     Running `target/debug/minigrep`
Problem parsing arguments: not enough arguments
```
Great! This output is much friendlier for our users.

### Extracting Logic from main
Now that we’ve finished refactoring the configuration parsing, let’s turn to the program’s logic. As we stated in “Separation of Concerns for Binary Projects”, we’ll extract a function named run that will hold all the logic currently in the main function that isn’t involved with setting up configuration or handling errors. When we’re done, main will be concise and easy to verify by inspection, and we’ll be able to write tests for all the other logic.

Listing 12-11 shows the extracted run function. For now, we’re just making the small, incremental improvement of extracting the function. We’re still defining the function in src/main.rs:

Filename: src/main.rs

```
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--
```
Listing 12-11: Extracting a run function containing the rest of the program logic

The run function now contains all the remaining logic from main, starting from reading the file. The run function takes the Config instance as an argument.

### Returning Errors from the run Function
With the remaining program logic separated into the run function, we can improve the error handling, as we did with Config::new in Listing 12-9. Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong. This will let us further consolidate into main the logic around handling errors in a user-friendly way. Listing 12-12 shows the changes we need to make to the signature and body of run:

Filename: src/main.rs

````
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
````
Listing 12-12: Changing the run function to return Result

We’ve made three significant changes here. First, we changed the return type of the run function to Result<(), Box<Error>>. This function previously returned the unit type, (), and we keep that as the value returned in the Ok case.

For the error type, we used the trait object Box<Error> (and we’ve brought std::error::Error into scope with a use statement at the top). We’ll cover trait objects in Chapter 17. For now, just know that Box<Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases.

Second, we’ve removed the calls to expect in favor of ?, as we talked about in Chapter 9. Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.

Third, the run function now returns an Ok value in the success case. We’ve declared the run function’s success type as () in the signature, which means we need to wrap the unit type value in the Ok value. This Ok(()) syntax might look a bit strange at first, but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:

```
warning: unused `std::result::Result` which must be used
  --> src/main.rs:18:5
   |
18 |     run(config);
   |     ^^^^^^^^^^^^
= note: #[warn(unused_must_use)] on by default
```
Rust tells us that our code ignored the Result value, and the Result value might indicate that an error occurred. But we’re not checking to see whether or not there was an error, and the compiler reminds us that we probably meant to have some error handling code here! Let’s rectify that problem now.

Handling Errors Returned from run in main
We’ll check for errors and handle them using a technique similar to the way we handled errors with Config::new in Listing 12-10, but with a slight difference:

Filename: src/main.rs

```
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```
We use if let rather than unwrap_or_else to check whether run returns an Err value and call process::exit(1) if it does. The run function doesn’t return a value that we want to unwrap in the same way that Config::new returns the Config instance. Because run returns () in the success case, we only care about detecting an error, so we don’t need unwrap_or_else to return the unwrapped value because it would only be ().

The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.

Splitting Code into a Library Crate
Our minigrep project is looking good so far! Now we’ll split the src/main.rs file and put some code into the src/lib.rs file so we can test it and have a src/main.rs file with fewer responsibilities.

Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:

- The run function definition
- The relevant use statements
- The definition of Config
- The `Config::new` function definition
- The contents of src/lib.rs should have the signatures shown in Listing 12-13 (we’ve omitted the bodies of the functions for brevity). Note that this won't compile until we modify src/main.rs in the listing after this one:

Filename: src/lib.rs

```
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // --snip--
}
```
Listing 12-13: Moving Config and run into src/lib.rs

We’ve made liberal use of pub here: on Config, its fields and its new method, and on the run function. We now have a library crate that has a public API that we can test!

Now we need to bring the code we moved to src/lib.rs into the scope of the binary crate in src/main.rs, as shown in Listing 12-14:

Filename: src/main.rs

```
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```
Listing 12-14: Bringing the minigrep crate into the scope of src/main.rs

To bring the library crate into the binary crate, we use extern crate minigrep. Then we’ll add a use minigrep::Config line to bring the Config type into scope, and we’ll prefix the run function with our crate name. Now all the functionality should be connected and should work. Run the program with cargo run and make sure everything works correctly.

Whew! That was a lot of work, but we’ve set ourselves up for success in the future. Now it’s much easier to handle errors, and we’ve made the code more modular. Almost all of our work will be done in src/lib.rs from here on out.

Let’s take advantage of this newfound modularity by doing something that would have been difficult with the old code but is easy with the new code: we’ll write some tests!

### Developing the Library’s Functionality with Test Driven Development
Now that we’ve extracted the logic into src/lib.rs and left the argument collecting and error handling in src/main.rs, it’s much easier to write tests for the core functionality of our code. We can call functions directly with various arguments and check return values without having to call our binary from the command line. Feel free to write some tests for the functionality in the Config::new and run functions on your own.

In this section, we’ll add the searching logic to the minigrep program by using the Test Driven Development (TDD) process. This software development technique follows these steps:

Write a test that fails, and run it to make sure it fails for the reason you expected.
Write or modify just enough code to make the new test pass.
Refactor the code you just added or changed, and make sure the tests continue to pass.
Repeat from step 1!
This process is just one of many ways to write software, but TDD can help drive code design as well. Writing the test before you write the code that makes the test pass helps to maintain high test coverage throughout the process.

We’ll test drive the implementation of the functionality that will actually do the searching for the query string in the file contents and produce a list of lines that match the query. We’ll add this functionality in a function called search.

## Writing a Failing Test
Because we don’t need them anymore, let’s remove the println! statements from src/lib.rs and src/main.rs that we used to check the program’s behavior. Then, in src/lib.rs, we’ll add a test module with a test function, as we did in Chapter 11. The test function specifies the behavior we want the search function to have: it will take a query and the text to search for the query in, and will return only the lines from the text that contain the query. Listing 12-15 shows this test, which won't compile yet:

Filename: src/lib.rs


```
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```
Listing 12-15: Creating a failing test for the search function we wish we had

This test searches for the string “duct.” The text we’re searching is three lines, only one of which contains “duct.” We assert that the value returned from the search function contains only the line we expect.

We aren’t able to run this test and watch it fail because the test doesn’t even compile: the search function doesn’t exist yet! So now we’ll add just enough code to get the test to compile and run by adding a definition of the search function that always returns an empty vector, as shown in Listing 12-16. Then the test should compile and fail because an empty vector doesn’t match a vector containing the line "safe, fast, productive.".

Filename: src/lib.rs


```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```
Listing 12-16: Defining just enough of the search function so our test will compile

Notice that we need an explicit lifetime 'a defined in the signature of search and used with the contents argument and the return value. Recall in Chapter 10 that the lifetime parameters specify which argument lifetime is connected to the lifetime of the return value. In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).

In other words, we tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument. This is important! The data referenced by a slice needs to be valid for the reference to be valid; if the compiler assumes we’re making string slices of query rather than contents, it will do its safety checking incorrectly.

If we forget the lifetime annotations and try to compile this function, we’ll get this error:

```
error[E0106]: missing lifetime specifier
 --> src/lib.rs:5:51
  |
5 | pub fn search(query: &str, contents: &str) -> Vec<&str> {
  |                                                   ^ expected lifetime
parameter
  |
  = help: this function's return type contains a borrowed value, but the
  signature does not say whether it is borrowed from `query` or `contents`
```
Rust can’t possibly know which of the two arguments we need, so we need to tell it. Because contents is the argument that contains all of our text and we want to return the parts of that text that match, we know contents is the argument that should be connected to the return value using the lifetime syntax.

Other programming languages don’t require you to connect arguments to return values in the signature, so although this might seem strange, it will get easier over time. You might want to compare this example with the “Validating References with Lifetimes” section in Chapter 10.

Now let’s run the test:

```
$ cargo test
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
--warnings--
    Finished dev [unoptimized + debuginfo] target(s) in 0.43 secs
     Running target/debug/deps/minigrep-abcabcabc

running 1 test
test test::one_result ... FAILED

failures:

---- test::one_result stdout ----
        thread 'test::one_result' panicked at 'assertion failed: `(left ==
right)`
left: `["safe, fast, productive."]`,
right: `[]`)', src/lib.rs:48:8
note: Run with `RUST_BACKTRACE=1` for a backtrace.


failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

error: test failed, to rerun pass '--lib'
```
Great, the test fails, exactly as we expected. Let’s get the test to pass!

### Writing Code to Pass the Test
Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:

- Iterate through each line of the contents.
- Check whether the line contains our query string.
- If it does, add it to the list of values we’re returning.
- If it doesn’t, do nothing.
- Return the list of results that match.
Let’s work through each step, starting with iterating through lines.

### Iterating Through Lines with the lines Method
Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines, that works as shown in Listing 12-17. Note this won’t compile yet:

Filename: src/lib.rs

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```
Listing 12-17: Iterating through each line in contents

The lines method returns an iterator. We’ll talk about iterators in depth in Chapter 13, but recall that you saw this way of using an iterator in Listing 3-4, where we used a for loop with an iterator to run some code on each item in a collection.

### Searching Each Line for the Query
Next, we’ll check whether the current line contains our query string. Fortunately, strings have a helpful method named contains that does this for us! Add a call to the contains method in the search function, as shown in Listing 12-18. Note this still won’t compile yet:

Filename: src/lib.rs

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        if line.contains(query) {
            // do something with line
        }
    }
}
```
Listing 12-18: Adding functionality to see whether the line contains the string in query

### Storing Matching Lines
We also need a way to store the lines that contain our query string. For that, we can make a mutable vector before the for loop and call the push method to store a line in the vector. After the for loop, we return the vector, as shown in Listing 12-19:

Filename: src/lib.rs

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```
Listing 12-19: Storing the lines that match so we can return them

Now the search function should return only the lines that contain query, and our test should pass. Let’s run the test:

```
$ cargo test
--snip--
running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Our test passed, so we know it works!

At this point, we could consider opportunities for refactoring the implementation of the search function while keeping the tests passing to maintain the same functionality. The code in the search function isn’t too bad, but it doesn’t take advantage of some useful features of iterators. We’ll return to this example in Chapter 13 where we’ll explore iterators in detail and look at how to improve it.

Using the search Function in the run Function
Now that the search function is working and tested, we need to call search from our run function. We need to pass the config.query value and the contents that run reads from the file to the search function. Then run will print each line returned from search:

Filename: src/lib.rs

```
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```
We’re still using a for loop to return each line from search and print it.

Now the entire program should work! Let’s try it out, first with a word that should return exactly one line from the Emily Dickinson poem, “frog”:

```
$ cargo run frog poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38 secs
     Running `target/debug/minigrep frog poem.txt`
How public, like a frog
Cool! Now let’s try a word that will match multiple lines, like “body”:


$ cargo run body poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep body poem.txt`
I’m nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
And finally, let’s make sure that we don’t get any lines when we search for a word that isn’t anywhere in the poem, such as “monomorphization”:


$ cargo run monomorphization poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep monomorphization poem.txt`
```
Excellent! We’ve built our own mini version of a classic tool and learned a lot about how to structure applications. We’ve also learned a bit about file input and output, lifetimes, testing, and command line parsing.

To round out this project, we’ll briefly demonstrate how to work with environment variables and how to print to standard error, both of which are useful when you’re writing command line programs.

## Working with Environment Variables
We’ll improve minigrep by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable. We could make this feature a command line option and require that users enter it each time they want it to apply, but instead we’ll use an environment variable. Doing so allows our users to set the environment variable once and have all their searches be case insensitive in that terminal session.

Writing a Failing Test for the Case-Insensitive search Function
We want to add a new search_case_insensitive function that we’ll call when the environment variable is on. We’ll continue to follow the TDD process, so the first step is again to write a failing test. We’ll add a new test for the new search_case_insensitive function and rename our old test from one_result to case_sensitive to clarify the differences between the two tests, as shown in Listing 12-20:

Filename: src/lib.rs


```
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
Listing 12-20: Adding a new failing test for the case-insensitive function we’re about to add
```
Note that we’ve edited the old test’s contents too. We’ve added a new line with the text “Duct tape” using a capital D that shouldn’t match the query “duct” when we’re searching in a case-sensitive manner. Changing the old test in this way helps ensure that we don’t accidentally break the case-sensitive search functionality that we’ve already implemented. This test should pass now and should continue to pass as we work on the case-insensitive search.

The new test for the case-insensitive search uses “rUsT” as its query. In the search_case_insensitive function we’re about to add, the query “rUsT” should match the line containing “Rust:” with a capital R and also the line “Trust me.” even though both have different casing than the query. This is our failing test, and it will fail to compile because we haven’t yet defined the search_case_insensitive function. Feel free to add a skeleton implementation that always returns an empty vector, similar to the way we did for the search function in Listing 12-16 to see the test compile and fail.

Implementing the search_case_insensitive Function
The search_case_insensitive function, shown in Listing 12-21, will be almost the same as the search function. The only difference is that we’ll lowercase the query and each line so whatever the case of the input arguments, they’ll be the same case when we check whether the line contains the query:

Filename: src/lib.rs


```
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```
Listing 12-21: Defining the search_case_insensitive function to lowercase the query and the line before comparing them

First, we lowercase the query string and store it in a shadowed variable with the same name. Calling to_lowercase on the query is necessary so no matter whether the user’s query is “rust”, “RUST”, “Rust”, or “rUsT”, we’ll treat the query as if it was “rust” and be insensitive to the case.

Note that query is now a String rather than a string slice, because calling to_lowercase creates new data rather than referencing existing data. Say the query is “rUsT”, as an example: that string slice doesn’t contain a lowercase “u” or “t” for us to use, so we have to allocate a new String containing “rust”. When we pass query as an argument to the contains method now, we need to add an ampersand because the signature of contains is defined to take a string slice.

Next, we add a call to to_lowercase on each line before we check whether it contains query to lowercase all characters. Now that we’ve converted line and query to lowercase, we’ll find matches no matter what the case of the query is.

Let’s see if this implementation passes the tests:

```
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Great! They passed. Now, let’s call the new search_case_insensitive function from the run function. First, we’ll add a configuration option to the Config struct to switch between case-sensitive and case-insensitive search. Adding this field will cause compiler errors since we aren’t initializing this field anywhere yet:

Filename: src/lib.rs


```
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```
Note that we added the case_sensitive field that holds a Boolean. Next, we need the run function to check the case_sensitive field’s value and use that to decide whether to call the search function or the search_case_insensitive function, as shown in Listing 12-22. Note this still won’t compile yet:

Filename: src/lib.rs


```
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
Listing 12-22: Calling either search or search_case_insensitive based on the value in config.case_sensitive
```
Finally, we need to check for the environment variable. The functions for working with environment variables are in the env module in the standard library, so we want to bring that module into scope with a use std::env; line at the top of src/lib.rs. Then we’ll use the var method from the env module to check for an environment variable named CASE_INSENSITIVE, as shown in Listing 12-23:

Filename: src/lib.rs


```
use std::env;

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
Listing 12-23: Checking for an environment variable named CASE_INSENSITIVE
```
Here, we create a new variable case_sensitive. To set its value, we call the env::var function and pass it the name of the CASE_INSENSITIVE environment variable. The env::var method returns a Result that will be the successful Ok variant that contains the value of the environment variable if the environment variable is set. It will return the Err variant if the environment variable is not set.

We’re using the is_err method on the Result to check whether it’s an error and therefore unset, which means it should do a case-sensitive search. If the CASE_INSENSITIVE environment variable is set to anything, is_err will return false and will perform a case-insensitive search. We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking is_err rather than unwrap, expect, or any of the other methods we’ve seen on Result.

We pass the value in the case_sensitive variable to the Config instance so the run function can read that value and decide whether to call search or search_case_insensitive as we implemented in Listing 12-22.

Let’s give it a try! First, we’ll run our program without the environment variable set and with the query “to”, which should match any line that contains the word “to” in all lowercase:

```
$ cargo run to poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```
Looks like that still works! Now, let’s run the program with CASE_INSENSITIVE set to 1 but with the same query “to”; we should get lines that contain “to” that might have uppercase letters:

```
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
If you’re using PowerShell, you will need to set the environment variable and run the program in two commands rather than one:

```
$ $env.CASE_INSENSITIVE=1
$ cargo run to poem.txt
```
Excellent, we also got lines containing “To”! Our minigrep program can now do case-insensitive searching controlled by an environment variable. Now you know how to manage options set using either command line arguments or environment variables!

Some programs allow arguments and environment variables for the same configuration. In those cases, the programs decide that one or the other takes precedence. For another exercise on your own, try controlling case insensitivity through either a command line argument or an environment variable. Decide whether the command line argument or the environment variable should take precedence if the program is run with one set to case sensitive and one set to case insensitive.

The `std::env` module contains many more useful features for dealing with environment variables: check out its documentation to see what is available.

## Writing Error Messages to Standard Error Instead of Standard Output
At the moment we’re writing all of our output to the terminal using the println! function. Most terminals provide two kinds of output: standard output (stdout) for general information and standard error (stderr) for error messages. This distinction enables users to choose to direct the successful output of a program to a file but still print error messages to the screen.

The `println!` function is only capable of printing to standard output, so we have to use something else to print to standard error.

### Checking Where Errors Are Written to
First, let’s observe how the content printed by minigrep is currently being written to standard output, including any error messages we want to write to standard error instead. We’ll do that by redirecting the standard output stream to a file while also intentionally causing an error. We won’t redirect the standard error stream, so any content sent to standard error will continue to display on the screen.

Command line programs are expected to send error messages to the standard error stream so we can still see error messages on the screen even if we redirect the standard output stream to a file. Our program is not currently well-behaved: we’re about to see that it saves the error message output to a file instead!

The way to demonstrate this behavior is by running the program with > and the filename, output.txt, that we want to redirect the standard output stream to. We won’t pass any arguments, which should cause an error:


`$ cargo run > output.txt`
The `>` syntax tells the shell to write the contents of standard output to output.txt instead of the screen. We didn’t see the error message we were expecting printed on the screen, so that means it must have ended up in the file. This is what output.txt contains:


### Problem parsing arguments: not enough arguments
Yup, our error message is being printed to standard output. It’s much more useful for error messages like this to be printed to standard error and have only data from a successful run end up in the file when we redirect standard output this way. We’ll change that.

### Printing Errors to Standard Error
We’ll use the code in Listing 12-24 to change how error messages are printed. Because of the refactoring we did earlier in this chapter, all the code that prints error messages is in one function, main. The standard library provides the `eprintln!` macro that prints to the standard error stream, so let’s change the two places we were calling println! to print errors to use `eprintln!` instead:

Filename: src/main.rs

```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```
Listing 12-24: Writing error messages to standard error instead of standard output using eprintln!

After changing `println!` to `eprintln!`, let’s run the program again in the same way, without any arguments and redirecting standard output with >:


`$ cargo run > output.txt`
Problem parsing arguments: not enough arguments
Now we see the error onscreen and output.txt contains nothing, which is the behavior we expect of command line programs.

Let’s run the program again with arguments that don’t cause an error but still redirect standard output to a file, like so:


`$ cargo run to poem.txt > output.txt`
We won’t see any output to the terminal, and output.txt will contain our results:

Filename: output.txt

```
Are you nobody, too?
How dreary to be somebody!
```
This demonstrates that we’re now using standard output for successful output and standard error for error output as appropriate.

## Summary
In this chapter, we’ve recapped some of the major concepts you’ve learned so far and covered how to do common I/O operations in a Rust context. By using command line arguments, files, environment variables, and the eprintln! macro for printing errors, you’re now prepared to write command line applications. By using the concepts in previous chapters, your code will be well organized, store data effectively in the appropriate data structures, handle errors nicely, and be well tested.

Next, we’ll explore some Rust features that were influenced by functional languages: closures and iterators.

# Functional Language Features: Iterators and Closures
Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is functional programming. Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth. In this chapter, we won’t debate the issue of what functional programming is or isn’t but will instead discuss some features of Rust that are similar to features in many languages often referred to as functional.

More specifically, we’ll cover:

- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use these two features to improve the I/O project in Chapter 12
- The performance of these two features (Spoiler alert: they’re faster than you might think!)

Other Rust features are influenced by the functional style as well, such as pattern matching and enums, which we’ve covered in other chapters. Mastering closures and iterators is an important part of writing idiomatic, fast Rust code, so we’ll devote this entire chapter to them.

## Closures: Anonymous Functions that Can Capture Their Environment
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place, and then call the closure to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re called. We’ll demonstrate how these closure features allow for code reuse and behavior customization.

### Creating an Abstraction of Behavior with Closures
Let’s work on an example of a situation in which it’s useful to store a closure to be executed at a later time. Along the way, we’ll talk about the syntax of closures, type inference, and traits.

Consider this hypothetical situation: we work at a startup that’s making an app to generate custom exercise workout plans. The backend is written in Rust, and the algorithm that generates the workout plan takes into account many different factors, such as the app user’s age, body mass index, preferences, recent workouts, and an intensity number they specify. The actual algorithm used isn’t important in this example; what’s important is that this calculation takes a few seconds. We want to call this algorithm only when we need to and only call it once, so we don’t make the user wait more than necessary.

We’ll simulate calling this hypothetical algorithm with the simulated_expensive_calculation function shown in Listing 13-1, which will print calculating slowly..., wait for two seconds, and then return whatever number we passed in:

Filename: src/main.rs


```
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```
Listing 13-1: A function to stand in for a hypothetical calculation that takes about two seconds to run

Next is the main function that contains the parts of the workout app important for this example. This function represents the code that the app will call when a user asks for a workout plan. Because the interaction with the app’s frontend isn’t relevant to the use of closures, we’ll hardcode values representing inputs to our program and print the outputs.

The required inputs are:

An intensity number from the user, which is specified when they request a workout to indicate whether they want a low-intensity workout or a high-intensity workout.
A random number that will generate some variety in the workout plans.
The output will be the recommended workout plan. Listing 13-2 shows the main function we’ll use:

Filename: src/main.rs

```
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```
Listing 13-2: A main function with hardcoded values to simulate user input and random number generation

We’ve hardcoded the variable simulated_user_specified_value to 10 and the variable simulated_random_number to 7 for simplicity’s sake; in an actual program, we’d get the intensity number from the app frontend and we’d use the rand crate to generate a random number, as we did in the Guessing Game example in Chapter 2. The main function calls a generate_workout function with the simulated input values.

Now that we have the context, let’s get to the algorithm. The generate_workout function in Listing 13-3 contains the business logic of the app that we’re most concerned with in this example. The rest of the code changes in this example will be made to this function:

Filename: src/main.rs


```
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```
Listing 13-3: The business logic that prints the workout plans based on the inputs and calls to the simulated_expensive_calculation function

The code in Listing 13-3 has multiple calls to the slow calculation function. The first if block calls simulated_expensive_calculation twice, the if inside the outer else doesn’t call it at all, and the code inside the second else case calls it once.

The desired behavior of the generate_workout function is to first check whether the user wants a low-intensity workout (indicated by a number less than 25) or a high-intensity workout (a number of 25 or greater).

Low-intensity workout plans will recommend a number of push-ups and sit-ups based on the complex algorithm we’re simulating.

If the user wants a high-intensity workout, there’s some additional logic: if the value of the random number generated by the app happens to be 3, the app will recommend a break and hydration. If not, the user will get a number of minutes of running based on the complex algorithm.

The data science team has let us know that we’ll have to make some changes to the way we call the algorithm in the future. To simplify the update when those changes happen, we want to refactor this code so it calls the simulated_expensive_calculation function only once. We also want to cut the place where we’re currently unnecessarily calling the function twice without adding any other calls to that function in the process. That is, we don’t want to call it if the result isn’t needed, and we still want to call it only once.

### Refactoring Using Functions
We could restructure the workout program in many ways. First, we’ll try extracting the duplicated call to the expensive_calculation function into a variable, as shown in Listing 13-4:

Filename: src/main.rs


```
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result =
        simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result
        );
        println!(
            "Next, do {} situps!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}
```
Listing 13-4: Extracting the calls to simulated_expensive_calculation to one place and storing the result in the expensive_result variable

This change unifies all the calls to simulated_expensive_calculation and solves the problem of the first if block unnecessarily calling the function twice. Unfortunately, we’re now calling this function and waiting for the result in all cases, which includes the inner if block that doesn’t use the result value at all.

We want to define code in one place in our program, but only execute that code where we actually need the result. This is a use case for closures!

### Refactoring with Closures to Store Code
Instead of always calling the simulated_expensive_calculation function before the if blocks, we can define a closure and store the closure in a variable rather than storing the result, as shown in Listing 13-5. We can actually move the whole body of simulated_expensive_calculation within the closure we’re introducing here:

Filename: src/main.rs


```
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```
Listing 13-5: Defining a closure and storing it in the expensive_closure variable

The closure definition comes after the = to assign it to the variable expensive_closure. To define a closure, we start with a pair of vertical pipes (|), inside which we specify the parameters to the closure; this syntax was chosen because of its similarity to closure definitions in Smalltalk and Ruby. This closure has one parameter named num: if we had more than one parameter, we would separate them with commas, like |param1, param2|.

After the parameters, we place curly brackets that hold the body of the closure—these are optional if the closure body is a single expression. The end of the closure, after the curly brackets, needs a semicolon to complete the let statement. The value returned from the last line in the closure body (num) will be the value returned from the closure when it’s called, because that line doesn’t end in a semicolon; just like in function bodies.

Note that this let statement means expensive_closure contains the definition of an anonymous function, not the resulting value of calling the anonymous function. Recall that we’re using a closure because we want to define the code to call at one point, store that code, and call it at a later point; the code we want to call is now stored in expensive_closure.

With the closure defined, we can change the code in the if blocks to call the closure to execute the code and get the resulting value. We call a closure like we do a function: we specify the variable name that holds the closure definition and follow it with parentheses containing the argument values we want to use, as shown in Listing 13-6:

Filename: src/main.rs


```
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}
```
Listing 13-6: Calling the expensive_closure we’ve defined

Now the expensive calculation is called in only one place, and we’re only executing that code where we need the results.

However, we’ve reintroduced one of the problems from Listing 13-3: we’re still calling the closure twice in the first if block, which will call the expensive code twice and make the user wait twice as long as they need to. We could fix this problem by creating a variable local to that if block to hold the result of calling the closure, but closures provide us with another solution. We’ll talk about that solution in a bit. But first let’s talk about why there aren’t type annotations in the closure definition and the traits involved with closures.

### Closure Type Inference and Annotation
Closures don’t require you to annotate the types of the parameters or the return value like fn functions do. Type annotations are required on functions because they’re part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. But closures aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

Additionally, closures are usually short and only relevant within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler is reliably able to infer the types of the parameters and return type, similar to how it’s able to infer the types of most variables.

Making programmers annotate the types in these small, anonymous functions would be annoying and largely redundant with the information the compiler already has available.

Like variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary; annotating the types for the closure we defined in Listing 13-4 would look like the definition shown in Listing 13-7:

Filename: src/main.rs


```
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```
Listing 13-7: Adding optional type annotations of the parameter and return value types in the closure

The syntax of closures and functions looks more similar with type annotations. The following is a vertical comparison of the syntax for the definition of a function that adds one to its parameter, and a closure that has the same behavior. We’ve added some spaces to line up the relevant parts. This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:

```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```
The first line shows a function definition, and the second line shows a fully annotated closure definition. The third line removes the type annotations from the closure definition, and the fourth line removes the brackets that are optional, because the closure body has only one expression. These are all valid definitions that will produce the same behavior when they’re called.

Closure definitions will have one concrete type inferred for each of their parameters and for their return value. For instance, Listing 13-8 shows the definition of a short closure that just returns the value it receives as a parameter. This closure isn’t very useful except for the purposes of this example. Note that we haven’t added any type annotations to the definition: if we then try to call the closure twice, using a String as an argument the first time and a u32 the second time, we’ll get an error:

Filename: src/main.rs

```
let example_closure = |x| x;

let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
Listing 13-8: Attempting to call a closure whose types are inferred with two different types

The compiler gives us this error:

```
error[E0308]: mismatched types
 --> src/main.rs
  |
  | let n = example_closure(5);
  |                         ^ expected struct `std::string::String`, found
  integral variable
  |
  = note: expected type `std::string::String`
             found type `{integer}`
```
The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked in to the closure in example_closure, and we get a type error if we try to use a different type with the same closure.

### Storing Closures Using Generic Parameters and the Fn Traits
Let’s return to our workout generation app. In Listing 13-6, our code was still calling the expensive calculation closure more times than it needed to. One option to solve this issue is to save the result of the expensive closure in a variable for reuse and use the variable instead in each place we need the result instead of calling the closure again. However, this method could result in a lot of repeated code.

Fortunately, another solution is available to us. We can create a struct that will hold the closure and the resulting value of calling the closure. The struct will only execute the closure if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.

To make a struct that holds a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields. Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different. To define structs, enums, or function parameters that use closures, we use generics and trait bounds, as we discussed in Chapter 10.

The Fn traits are provided by the standard library. All closures implement one of the traits: Fn, FnMut, or FnOnce. We’ll discuss the difference between these traits in the next section on capturing the environment; in this example, we can use the Fn trait.

We add types to the Fn trait bound to represent the types of the parameters and return values the closures must have to match this trait bound. In this case, our closure has a parameter of type u32 and returns a u32, so the trait bound we specify is Fn(u32) -> u32.

Listing 13-9 shows the definition of the Cacher struct that holds a closure and an optional result value:

Filename: src/main.rs


```
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}
```
Listing 13-9: Defining a `Cacher` struct that holds a closure in calculation and an optional result in value

The `Cacher` struct has a calculation field of the generic type `T`. The trait bounds on `T` specify that it’s a closure by using the `Fn` trait. Any closure we want to store in the calculation field must have one `u32` parameter (specified within the parentheses after `Fn`) and must return a `u32` (specified after the `->`).

Note: Functions implement all three of the `Fn` traits too. If what we want to do doesn’t require capturing a value from the environment, we can use a function rather than a closure where we need something that implements an `Fn` trait.

The value field is of type `Option<u32>`. Before we execute the closure, value will be None. When code using a `Cacher` asks for the result of the closure, the `Cacher` will execute the closure at that time and store the result within a Some variant in the value field. Then if the code asks for the result of the closure again, instead of executing the closure again, the Cacher will return the result held in the Some variant.

The logic around the value field we’ve just described is defined in Listing 13-10:

Filename: src/main.rs


```
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}
```
Listing 13-10: The caching logic of Cacher

We want `Cacher` to manage the struct fields’ values rather than letting the calling code potentially change the values in these fields directly, so these fields are private.

The `Cacher::new` function takes a generic parameter `T`, which we’ve defined as having the same trait bound as the `Cacher` struct. Then Cacher::new returns a `Cacher` instance that holds the closure specified in the calculation field and a `None` value in the value field, because we haven’t executed the closure yet.

When the calling code wants the result of evaluating the closure, instead of calling the closure directly, it will call the value method. This method checks whether we already have a resulting value in self.value in a `Some`; if we do, it returns the value within the Some without executing the closure again.

If `self.value` is `None`, we call the closure stored in `self.calculation`, save the result in `self.value` for future use, and return the value as well.

Listing 13-11 shows how we can use this `Cacher` struct in the `generate_workout` function from Listing 13-6:

Filename: src/main.rs


```
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```
Listing 13-11: Using Cacher in the generate_workout function to abstract away the caching logic

Instead of saving the closure in a variable directly, we save a new instance of Cacher that holds the closure. Then, in each place we want the result, we call the value method on the Cacher instance. We can call the value method as many times as we want, or not call it at all, and the expensive calculation will be run a maximum of once.

Try running this program with the main function from Listing 13-2. Change the values in the simulated_user_specified_value and simulated_random_number variables to verify that in all the cases in the various if and else blocks, calculating slowly... only appears once and only when needed. The Cacher takes care of the logic necessary to ensure we aren’t calling the expensive calculation more than we need to, so generate_workout can focus on the business logic.

### Limitations of the Cacher Implementation
Caching values is a generally useful behavior that we might want to use in other parts of our code with different closures. However, there are two problems with the current implementation of Cacher that would make reusing it in different contexts difficult.

The first problem is that a Cacher instance assumes it will always get the same value for the parameter arg to the value method. That is, this test of Cacher will fail:

```
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
```
This test creates a new Cacher instance with a closure that returns the value passed into it. We call the value method on this Cacher instance with an arg value of 1 and then an arg value of 2, and we expect that the call to value with the arg value of 2 should return 2.

Run this test with the Cacher implementation in Listing 13-9 and Listing 13-10, and the test will fail on the assert_eq! with this message:

```
thread 'call_with_different_values' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/main.rs
```
The problem is that the first time we called c.value with 1, the Cacher instance saved Some(1) in self.value. Thereafter, no matter what we pass in to the value method, it will always return 1.

Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present. If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.

The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. We might want to cache the results of closures that take a string slice and return usize values, for example. To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.

### Capturing the Environment with Closures
In the workout generator example, we only used closures as inline anonymous functions. However, closures have an additional capability that functions don’t have: they can capture their environment and access variables from the scope in which they’re defined.

Listing 13-12 has an example of a closure stored in the variable equal_to_x that uses the variable x from the closure’s surrounding environment:

Filename: src/main.rs

```
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```
Listing 13-12: Example of a closure that refers to a variable in its enclosing scope

Here, even though x is not one of the parameters of equal_to_x, the equal_to_x closure is allowed to use the x variable that’s defined in the same scope that equal_to_x is defined in.

We can’t do the same with functions; if we try with the following example, our code won’t compile:

Filename: src/main.rs

```
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```
We get an error:

```
error[E0434]: can't capture dynamic environment in a fn item; use the || { ...
} closure form instead
 --> src/main.rs
  |
4 |     fn equal_to_x(z: i32) -> bool { z == x }
  |                                          ^
```

The compiler even reminds us that this only works with closures!

When a closure captures a value from its environment, it uses memory to store the values for use in the closure body. This use of memory is overhead that we don’t want to pay in more common cases where we want to execute code that doesn’t capture its environment. Because functions are never allowed to capture their environment, defining and using functions will never incur this overhead.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: taking ownership, borrowing immutably, and borrowing mutably. These are encoded in the three Fn traits as follows:

FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
Fn borrows values from the environment immutably.
FnMut can change the environment because it mutably borrows values.
When we create a closure, Rust infers which trait to use based on how the closure uses the values from the environment. In Listing 13-12, the equal_to_x closure borrows x immutably (so equal_to_x has the Fn trait) because the body of the closure only needs to read the value in x.

If we want to force the closure to take ownership of the values it uses in the environment, we can use the move keyword before the parameter list. This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.

We’ll have more examples of move closures in Chapter 16 when we talk about concurrency. For now, here’s the code from Listing 13-12 with the move keyword added to the closure definition and using vectors instead of integers, because integers can be copied rather than moved; note that this code will not yet compile:

Filename: src/main.rs

```
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```
We receive the following error:

```
error[E0382]: use of moved value: `x`
 --> src/main.rs:6:40
  |
4 |     let equal_to_x = move |z| z == x;
  |                      -------- value moved (into closure) here
5 |
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value used here after move
  |
  = note: move occurs because `x` has type `std::vec::Vec<i32>`, which does not
  implement the `Copy` trait
```
The x value is moved into the closure when the closure is defined, because we added the move keyword. The closure then has ownership of x, and main isn’t allowed to use x anymore in the println! statement. Removing println! will fix this example.

Most of the time when specifying one of the Fn trait bounds, you can start with Fn and the compiler will tell you if you need FnMut or FnOnce based on what happens in the closure body.

To illustrate situations where closures that can capture their environment are useful as function parameters, let’s move on to our next topic: iterators.

## Processing a Series of Items with Iterators
The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When we use iterators, we don’t have to reimplement that logic ourselves.

In Rust, iterators are lazy, meaning they have no effect until we call methods that consume the iterator to use it up. For example, the code in Listing 13-13 creates an iterator over the items in the vector v1 by calling the iter method defined on Vec. This code by itself doesn’t do anything useful:


```
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```
Listing 13-13: Creating an iterator

Once we’ve created an iterator, we can use it in a variety of ways. In Listing 3-4 in Chapter 3, we used iterators with for loops to execute some code on each item, although we glossed over what the call to iter did until now.

The example in Listing 13-14 separates the creation of the iterator from the use of the iterator in the for loop. The iterator is stored in the v1_iter variable, and no iteration takes place at that time. When the for loop is called using the iterator in v1_iter, each element in the iterator is used in one iteration of the loop, which prints out each value:


```
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```
Listing 13-14: Making use of an iterator in a for loop

In languages that don’t have iterators provided by their standard libraries, we would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it gets to the total number of items in the vector.

Iterators handle all that logic for us, cutting down on repetitive code we could potentially mess up. Iterators give us more flexibility to use the same logic with many different kinds of sequences, not just data structures we can index into, like vectors. Let’s examine how iterators do that.

### The Iterator Trait and the next Method
All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:


```
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```
Notice some new syntax that we haven’t covered yet: type Item and Self::Item, which are defining an associated type with this trait. We’ll talk about associated types in depth in Chapter 19. For now, all you need to know is that this code says implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method. In other words, the Item type will be the type returned from the iterator.

The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, it returns None.

We can call the next method on iterators directly; Listing 13-15 demonstrates what values are returned from repeated calls to next on the iterator created from the vector:

Filename: src/lib.rs


```
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```
Listing 13-15: Calling the next method on an iterator

Note that we needed to make v1_iter mutable: calling the next method on an iterator changes state that keeps track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

Also note that the values we get from the calls to next are immutable references to the values in the vector. The iter method produces an iterator over immutable references. If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

### Methods that Consume the Iterator
The Iterator trait has a number of different methods with default implementations provided for us by the standard library; you can find out about these methods by looking in the standard library API documentation for the Iterator trait. Some of these methods call the next method in their definition, which is why we’re required to implement the next method when implementing the Iterator trait.

Methods that call next are called consuming adaptors, because calling them uses up the iterator. One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete. Listing 13-16 has a test illustrating a use of the sum method:

Filename: src/lib.rs


```
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```
Listing 13-16: Calling the sum method to get the total of all items in the iterator

We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

### Methods that Produce Other Iterators
Other methods defined on the Iterator trait, known as iterator adaptors, allow us to change iterators into different kind of iterators. We can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, we have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

Listing 13-17 shows an example of calling the iterator adaptor method map, which takes a closure to call on each item to produce a new iterator. The closure here creates a new iterator in which each item from the vector has been incremented by 1. However, this code produces a warning:

Filename: src/main.rs


```
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```
Listing 13-17: Calling the iterator adaptor map to create a new iterator

The warning we get is:

```
warning: unused `std::iter::Map` which must be used: iterator adaptors are lazy
and do nothing unless consumed
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unused_must_use)] on by default
```
The code in Listing 13-17 doesn’t do anything; the closure we’ve specified never gets called. The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.

To fix this and consume the iterator, we’ll use the collect method, which you saw briefly in Chapter 12. This method consumes the iterator and collects the resulting values into a collection data type.

In Listing 13-18, we collect the results of iterating over the iterator that’s returned from the call to map into a vector. This vector will end up containing each item from the original vector incremented by 1:

Filename: src/main.rs


```
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```
Listing 13-18: Calling the map method to create a new iterator, and then calling the collect method to consume the new iterator and create a vector

Because map takes a closure, we can specify any operation we want to perform on each item. This is a great example of how closures let us customize some behavior while reusing the iteration behavior that the Iterator trait provides.

Using Closures that Capture Their Environment
Now that we’ve introduced iterators, we can demonstrate a common use of closures that capture their environment by using the filter iterator adaptor. The filter method on an iterator takes a closure that takes each item from the iterator and returns a Boolean. If the closure returns true, the value will be included in the iterator produced by filter. If the closure returns false, the value won’t be included in the resulting iterator.

In Listing 13-19 we use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size:

Filename: src/lib.rs


```
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
```
Listing 13-19: Using the filter method with a closure that captures shoe_size

The shoes_in_my_size function takes ownership of a vector of shoes and a shoe size as parameters. It returns a vector containing only shoes of the specified size.

In the body of shoes_in_my_size, we call into_iter to create an iterator that takes ownership of the vector. Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.

The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified. Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.

The test shows that when we call shoes_in_my_size, we only get back shoes that have the same size as the value we specified.

### Creating Our Own Iterators with Iterator
We’ve shown that we can create an iterator by calling iter, into_iter, or iter_mut on a vector. We can create iterators from the other collection types in the standard library, such as hash map. We can also create iterators that do anything we want by implementing the Iterator trait on our own types. As previously mentioned, the only method we’re required to provide a definition for is the next method. Once we’ve done that, we can use all other methods that have default implementations provided by the Iterator trait!

To demonstrate, let’s create an iterator that will only ever count from 1 to 5. First, we’ll create a struct to hold some values, and then we’ll make this struct into an iterator by implementing the Iterator trait and use the values in that implementation.

Listing 13-20 has the definition of the Counter struct and an associated new function to create instances of Counter:

Filename: src/lib.rs

```

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```
Listing 13-20: Defining the Counter struct and a new function that creates instances of Counter with an initial value of 0 for count

The Counter struct has one field named count. This field holds a u32 value that will keep track of where we are in the process of iterating from 1 to 5. The count field is private because we want the implementation of Counter to manage its value. The new function enforces the behavior of always starting new instances with a value of 0 in the count field.

Next, we’ll implement the Iterator trait for our Counter type by defining the body of the next method to specify what we want to happen when this iterator is used, as shown in Listing 13-21:

Filename: src/lib.rs


```
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```
Listing 13-21: Implementing the Iterator trait on our Counter struct

We set the associated Item type for our iterator to u32, meaning the iterator will return u32 values. Again, don’t worry about associated types yet, we’ll cover them in Chapter 19.

We want our iterator to add one to the current state, so we initialized count to 0 so it would return 1 first. If the value of count is less than 6, next will return the current value wrapped in Some, but if count is 6 or higher, our iterator will return None.

Using Our Counter Iterator’s next Method
Once we’ve implemented the Iterator trait, we have an iterator! Listing 13-22 shows a test demonstrating that we can use the iterator functionality of our Counter struct by calling the next method on it directly, just like we did with the iterator created from a vector in Listing 13-15:

Filename: src/lib.rs

```

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```
Listing 13-22: Testing the functionality of the next method implementation

This test creates a new Counter instance in the counter variable and then calls next repeatedly, verifying that we have implemented the behavior we want this iterator to have: returning the values from 1 to 5.

### Using Other Iterator Trait Methods
Because we implemented the Iterator trait by defining the next method, we can now use any Iterator trait method’s default implementations as defined in the standard library, because they all use the next method’s functionality.

For example, if for some reason we wanted to take the values produced by an instance of Counter, pair them with values produced by another Counter instance after skipping the first value, multiply each pair together, keep only those results that are divisible by three, and add all the resulting values together, we could do so, as shown in the test in Listing 13-23:

Filename: src/lib.rs


```
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}
```
Listing 13-23: Using a variety of Iterator trait methods on our Counter iterator

Note that zip produces only four pairs; the theoretical fifth pair (5, None) is never produced because zip returns None when either of its input iterators return None.

All of these method calls are possible because we specified how the next method works, and the standard library provides default implementations for other methods that call next.


## Improving Our I/O Project
With this new knowledge about iterators, we can improve the I/O project in Chapter 12 by using iterators to make places in the code clearer and more concise. Let’s look at how iterators can improve our implementation of the Config::new function and the search function.

Removing a clone Using an Iterator
In Listing 12-6, we added code that took a slice of String values and created an instance of the Config struct by indexing into the slice and cloning the values, allowing the Config struct to own those values. In Listing 13-24, we’ve reproduced the implementation of the Config::new function as it was in Listing 12-23 at the end of Chapter 12:

Filename: src/lib.rs

```
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
Listing 13-24: Reproduction of the Config::new function from the end of Chapter 12
```
At the time, we said not to worry about the inefficient clone calls because we would remove them in the future. Well, that time is now!

We needed clone here because we have a slice with String elements in the parameter args, but the new function doesn’t own args. To return ownership of a Config instance, we had to clone the values from the query and filename fields of Config so the Config instance can own its values.

With our new knowledge about iterators, we can change the new function to take ownership of an iterator as its argument instead of borrowing a slice. We’ll use the iterator functionality instead of the code that checks the length of the slice and indexes into specific locations. This will clarify what the Config::new function is doing because the iterator will access the values.

Once Config::new takes ownership of the iterator and stops using indexing operations that borrow, we can move the String values from the iterator into Config rather than calling clone and making a new allocation.

### Using the Returned Iterator Directly
Open your I/O project’s src/main.rs file, which should look like this:

Filename: src/main.rs

```
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```
We’ll change the start of the main function that we had in Listing 12-24 at the end of Chapter 12 to the code in Listing 13-25. This won’t compile yet until we update Config::new as well:

Filename: src/main.rs

```
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```
Listing 13-25: Passing the return value of env::args to Config::new

The env::args function returns an iterator! Rather than collecting the iterator values into a vector and then passing a slice to Config::new, now we’re passing ownership of the iterator returned from env::args to Config::new directly.

Next, we need to update the definition of Config::new. In your I/O project’s src/lib.rs file, let’s change the signature of Config::new to look like Listing 13-26. This still won’t compile yet because we need to update the function body:

Filename: src/lib.rs

```
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // --snip--
Listing 13-26: Updating the signature of Config::new to expect an iterator
```
The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args. We’ve updated the signature of the Config::new function so the parameter args has the type std::env::Args instead of &[String]. Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we can add the mut keyword into the specification of the args parameter to make it mutable.

### Using Iterator Trait Methods Instead of Indexing
Next, we’ll fix the body of Config::new. The standard library documentation also mentions that std::env::Args implements the Iterator trait, so we know we can call the next method on it! Listing 13-27 updates the code from Listing 12-23 to use the next method:

Filename: src/lib.rs


```
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
Listing 13-27: Changing the body of Config::new to use iterator methods
```
Remember that the first value in the return value of env::args is the name of the program. We want to ignore that and get to the next value, so first we call next and do nothing with the return value. Second, we call next on the value we want to put in the query field of Config. If next returns a Some, we use a match to extract the value. If it returns None, it means not enough arguments were given and we return early with an Err value. We do the same thing for the filename value.

### Making Code Clearer with Iterator Adaptors
We can also take advantage of iterators in the search function in our I/O project, which is reproduced here in Listing 13-28 as it was in Listing 12-19 at the end of Chapter 12:

Filename: src/lib.rs

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
Listing 13-28: The implementation of the search function from Chapter 12
```
We can write this code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector. The functional programming style prefers to minimize the amount of mutable state to make code clearer. Removing the mutable state might make it easier for us to make a future enhancement to make searching happen in parallel, because we wouldn’t have to manage concurrent access to the results vector. Listing 13-29 shows this change:

Filename: src/lib.rs

```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
Listing 13-29: Using iterator adaptor methods in the implementation of the search function
```
Recall that the purpose of the search function is to return all lines in contents that contain the query. Similar to the filter example in Listing 13-19, we can use the filter adaptor to keep only the lines that line.contains(query) returns true for. We then collect the matching lines into another vector with collect. Much simpler! Feel free to make the same change to use iterator methods in the search_case_insensitive function as well.

The next logical question is which style you should choose in your own code and why: the original implementation in Listing 13-28 or the version using iterators in Listing 13-29. Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand. Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop. This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, such as the filtering condition each element in the iterator must pass.

But are the two implementations truly equivalent? The intuitive assumption might be that the more low-level loop will be faster. Let’s talk about performance.

## Comparing Performance: Loops vs. Iterators
To determine whether to use loops or iterators, we need to know which version of our search functions is faster: the version with an explicit for loop or the version with iterators.

We ran a benchmark by loading the entire contents of The Adventures of Sherlock Holmes by Sir Arthur Conan Doyle into a String and looking for the word “the” in the contents. Here are the results of the benchmark on the version of search using the for loop and the version using iterators:

```
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```
The iterator version was slightly faster! We won’t explain the benchmark code here, because the point is not to prove that the two versions are equivalent but to get a general sense of how these two implementations compare performance-wise.

For a more comprehensive benchmark, you should check various texts of various sizes, different words, words of different lengths, and all kinds of other variations. The point is this: iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself. Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead in the same way that Bjarne Stroustrup, the original designer and implementor of C++, defines zero-overhead:

In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

`Bjarne Stroustrup’s “Foundations of C++”`

As another example, the following code is taken from an audio decoder. The decoding algorithm uses the linear prediction mathematical operation to estimate future values based on a linear function of the previous samples. This code uses an iterator chain to do some math on three variables in scope: a buffer slice of data, an array of 12 coefficients, and an amount by which to shift data in qlp_shift. We’ve declared the variables within this example but not given them any values; although this code doesn’t have much meaning outside of its context, it’s still a concise, real-world example of how Rust translates high-level ideas to low-level code:

```
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```
To calculate the value of prediction, this code iterates through each of the 12 values in coefficients and uses the zip method to pair the coefficient values with the previous 12 values in buffer. Then, for each pair, we multiply the values together, sum all the results, and shift the bits in the sum qlp_shift bits to the right.

Calculations in applications like audio decoders often prioritize performance most highly. Here, we’re creating an iterator, using two adaptors, and then consuming the value. What assembly code would this Rust code compile to? Well, as of this writing, it compiles down to the same assembly you’d write by hand. There’s no loop at all corresponding to the iteration over the values in coefficients: Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

All of the coefficients get stored in registers, which means it’s very fast to access the values. There are no bounds checks on the array access at runtime. All these optimizations Rust is able to apply make the resulting code extremely efficient. Now that you know this, you can use iterators and closures without fear! They make code seem like it’s higher level but don’t impose a runtime performance penalty for doing so.

### Summary
Closures and iterators are Rust features inspired by functional programming language ideas. They contribute to Rust’s capability to clearly express high-level ideas at low-level performance. The implementations of closures and iterators are such that runtime performance is not affected. This is part of Rust’s goal to strive to provide zero-cost abstractions.

Now that we’ve improved the expressiveness of our I/O project, let’s look at some more features of cargo that will help us share the project with the world.
