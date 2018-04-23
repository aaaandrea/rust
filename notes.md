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

The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures, and Rust doesn’t know which you want unless you specify. For the type parameters for the key and value types, however, we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the vectors.

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

This code will print {"world": 2, "hello": 1, "wonderful": 1}. The or_insert method actually returns a mutable reference (&mut V) to the value for this key. Here we store that mutable reference in the count variable, so in order to assign to that value we must first dereference count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

#### Hashing Function
By default, HashMap uses a cryptographically secure hashing function that can provide resistance to Denial of Service (DoS) attacks. This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it. If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. We’ll talk about traits and how to implement them in Chapter 10. You don’t necessarily have to implement your own hasher from scratch; crates.io has libraries shared by other Rust users that provide hashers implementing many common hashing algorithms.

#### Summary
Vectors, strings, and hash maps will provide a large amount of functionality that you need in programs where you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

Given a list of integers, use a vector and return the mean (average), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!

We’re getting into more complex programs in which operations can fail; so, it’s a perfect time to discuss error handling next!
