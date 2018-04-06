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

- Variable bindings
```
let a = true;
let b: bool = true;

let (x, y) = (1, 2);

let mut z = 5;
z = 6;
```
The `let` keyword is used in binding expressions. We can bind a name to a value || function. Also, because the left-hand side of a `let` expression is a "pattern", you can bind multiple names to set of values or function values.

- Constants
```
const N: i32 = 5;
```
The `const` keyword is used to define constants. It lives for the entire lifetime of a program, but has no fixed address in memory.


- Statics
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
- **bool**: true or false
    ```
    let x = true;
    let y: bool = false;
    // no TRUE, FALSE, 1, 0
    ```

- **char**: a single Unicode scalar value
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
let e = a / 2; // ⭐️ 2 not 2.5
let f = a % 2; //1

let g = 5.0 / 2.0; //2.5
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
The = operator is used to assign a name to a value or a function. Compound Assignment Operators are created by composing one of `+ - * / % & | ^ << >>` operators with `=` operator.

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
