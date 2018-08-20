# RustConf 2018 Notes
## Keynote
People Systems Programming
- Unleashing Latent Energy
    - RFC: Scaling Rust Governance
        - Bringing people in who didn't have a formal place in Rust
        - climbed from 20 to 120 people in Rust governance since 2015
        - introduced bots, infrastructure and policy? teams
        - Step 1: Asking for help in release management
            - Impl Period
            - weren't sure this could even be open source
            - started with asking for help and cc'd a lot of people
        - Step 2: Make a Spaceroups
            - What things were blocking others from getting involved.
        - Step 3: Public Recognition. Thanks.
    - Success can be painful
        - Sometimes feels like people are trying to take something from you, or getting in their way.
        - The core team must, but the core team can't
        - Do vs Support
            - Control vs Enjoying the Ride
                - eg. adding `deref` Trait
                    - thought they'd do it himself, but a PR came forward and it was done better than he would've
            - Latency vs Throughput
                - no easy answer, but both sides are possible
    - Itching the same
        - Every good work of software starts by scratching a developer's personal itch
    - Serendipity
        - If you only run your repos by taking into account those who open
        - Itching everyone the same way so everyine can be involved
- Roadmaps and vision docs

- True Openness
    - Pluralism and Positive Sums
        - Pluralism where 2 or more states can coexist
        - Game Theory
            - Zero sum game - where every participants win is balanced by someone's lost
            - Positive sum game - where the whole is greater than the sum of its parts
    - The Module System Debate
        - crate system debate with multiple RFCs
        - momentum
        - urgency
        - fatigue and scale

- Oppositions
    - Memory safety without garbage collection
    - in formal logic, when we reach a contradiction we are wrong
    - but in a dialictic, we can find that change and progress comes from time and progression in dialictics
    - Thesis -> Antithesis -> synthesis -> Antithesis -> etc
Future of Rust
    - Rust today is not Rust tomorrow
    - conflict doesn't have to generate heat. Can work through them in a positive sum game
    - synthesis means listening and diversity.


## Benchmarking and Optimization of Rust Libraries
- Micro vs Macro benchmarking
    - small unit vs customer workloads
- cargo bench, leverages the test crate
    - cargo-benchcmp allows for comparisons over time
        - sensitive to thresholds
    - nightly only
- criterion.rs
    - inspired by haskells criterion library
    - nanosecond outputs graphs
- Obtaining realistic workloads
- use instrumentation to understand relatively expensive calls as well as high invocation functions - What is actually happening given certain inputs
    - try Instruments.app on MacOS
- Improving performance
    - Fundamentals
        - Early exit contidtions
        - operational effitiancies
        - parallelism
        - eg. postgres - 3.14
            - maybe chunk into groups of 3
    - Don't clone to get around the borrow checker
        - stack vs heap - consider the size of the data
    - what about `unsafe`
        - avoid
        - generally not any better


## Getting Something for Nothing
_embedded systems: how we cna teach the rust compiler to enforce what we want_
- zero-cost abstractions is not new.
- LLVM compiler backend
- @![no_std] - where we are going we don't need OS
- `rustup default stable`
- `cargo build --target <name>`
=> working microcontroller

### Embedded Systems
_computers you don't sit in front of_
- combos of hardware<>software to do one thing well
- peripherals -
- hardware API - herding bits

- using the borrow checker to track ownership and borrowing
    - singleton pattern to be only one version of a class
        - eg global mutable variable
            - but with rust this would mean everything done with it would be unsafe
        - INSTEAD write a struct called peripherals that writes an option and returns an option
        - try rtfm that handles it for you in macros - japaric.io
- why singletons make a difference
    - its the reference to self
    - if you have a reference to self, it forces you to pass the ownership and borrowing around
        - enforces in the compiler, not at run time
        - * only works in one application
- type system
    - `GPIOS` - general purpose input/output - smallest work for a microcontroller
    - start with configure: input/output gpio


## The Dark Secrets Lurking inside `cargo doc`
_obscure things you can do with documentation_
- `cargo test --doc`: verifies that doc samples show code that works with compiled code

- The #[doc] attribute
    - html_root_url
    - test(attr)
    - inflie/no_inline
        - tells whether I want the pub use, or the docs
    - hidden
        - control whats visible. eg. won't appear if it's something not meant to be used, and work with the code directly.
    - include
        - needs feature attribute.
        - if you habe a lot of docs, it blows up your source files.
        - with this you can write them in a seperate file and have it copy-pasta in, but still be tracked separately
    - cfg
        - std::os
        - conditional compilation vs. your docs
    - can add additional content with html-after-content


## Writing Crates for Complete Beginners: A Tour of Turtle
slides.com/sunjay/tour-of-turtle
### Overview
### Highlits
### Lessons learns + tips

Resources
- New Rustacean
- Rusty Spile Podcast
- The official Rust Youtube Channel



## mentat
- embedded rust store
- part of the browser architecture user data team
- goals are to figure out the necessary changes they need to make for the five year goals

### problems
- storing the data
    - each team makes its own decisions about data storage layers
        - often driven by immediate needs
    - not thinking much about sharing data
        - what about the needs beyond their immediate needs also
    - simple stores don't scale well
    - structured stores don't evolve well
        - eg sqllite store will still need to handle many migrations
- sharing the data
    - end to end encryption
    - multiple clients and growing
    - lack of portability to clients
        - each client has their data stored in a variety of store types
    - data structure for querying not syncing
    - no history to help with conflict resolution

### example: existing sync

client 1 ---> server <--- client 2

### resolution
- CQRS: Command Query Responsibility Segregation
    - separate the query from the command which addresses the needs to each side of that equation

- Mentat: Buy or Build
    - Range of solutions explored
        - KV stores, Graph DBs, Document Stores, Relational Stores
    - Missing some key features
        - embedding, full tesxt indexing, defined scemas
    - Came across datomic
        - Pros
            - transactional
            - strongly typed
            - has a log
        - Cons
            - only server side
            - not open source
    - DataScript
    - Build your own
        - SQLite to store the data
            - embedded, FTS capable
        - layering datomics pros
            - eg transactio  log, mutable schema

        - first stabs written in clojurescript
        - changesd to rust
            - modern lang expression
            - low level perf
            - predictability
            - portability
            - concurrency
            - implemented to be something more like `rebase` for syncing and maintaining a clean timeline with conflict resolution algorithms


## Embedding Rust in C/C++
- ABI: application binary interface
    - In Rust we use `extern "C" fn foo {/*...*/}`
    - Open issue for defining an ABI for C or C++

### C Code from Rust aka Boring FFI
- Bind to native api with extern functions
- wrap calls in unsafe because the compiler can't verify
- make data C-compatible
- two modules os::raw vs ffi
    - uses primatives becomeing their own
    - void => c_void etc

### Turning Tables
- Same extern code  
- Take data in c-form
- muse use `#[no_mangle]` to preserve function name
- Cargo.toml
```
[library]
name = reverso
etc
```
- all rust functions need to be then declared in C header so C can use it

### BUT some problems
- don't want to write headers
    - `cbindgen` to generate header files in the build.rs OR using the CLI for your build system
- how to deal with anything going wrong
- real memory management
- how to build pretty APIs

### Tooling
- building system support
    - memory management
        creating an object in rust creates it on the sstack rather than the heap
        - not a great idea to use the stack as perm data storage
            - try putting it into `Box::new(TheThing)...`
            - Boxes store the pointer and retain type information
            - C is norw responsible for the memory: remember you can't make the native code memory safe
    - communicating errors
        - result (returns an ok or err) and option (returns something or nothing)
        - errors in C
            - build a layer to C
            - can emulate a result in C
                - ```
                    #[repr(C)]
                    pub struct rvalue_t<T> {
                        ...
                    }
                  ```
        - errors in C++
            - build a layer to C
            - `try - throw - catch`
                - try - landing pad determines how to continues
                - catch - is a filter
                - throw - replaced by calls into libc++ that allocate excetions and throw the exception
                - INTRODUCING `exception.rs`
                - need to compile wiht a a c++ compiler invoked in g++
        - using double pointers
            - provide the pointer to the pointer to something
                - in the function we can reference the first point to then handle witht he next pointer
                - allows you to handle something; if no error the pointer variable will point to a valid client
            - Rust has pointers, but they are not `safe`


kookie@spacekookie.de
- Ferrous Systems


## C2Rust: Migrating Legacy Code to Rust
### Why
migrations are far from perfect
1. reduce the tedium of initial translation
2. catch errors during the refactoring

### Transpiling
- Design Goals
    - Robust C and C++ parsing
    - preserve fxn of input code (since its probs well-tested)
    - generate output for humans
    - write back end in rust; reuse rust internals in the compiler

### Other efforts
- corrode
    - uses haskell c parsing library
- citrus-rs
    - uses clang for parsing; doesn't generate code that will write right away.

### The transpiler: C++ frontend with Rust backend
Compile commands -> C sources -< transpile.py (some stuff) -> rust sources

AST importer
.cbor -> clang AST -> importer AST -> rustc AST

preprocessor diretives
- translating after preprocessing
- how was compiler invoked?
    - can get it depending on the build system
- recording compile commands
    -cmake

### simple loops
- with unstructured control flow they generate while loops
- gotos -> replooping -> funky unsafe while loop

### translator limitations
- unimplemented
    - variadic function defs (Rust RFCs blocking issue)
    - bitfields (currently has a rust RFC)
    - long doublbe/ Complex
    - macros: translation happens after macro expansion
        - would prefer flexibility of C code in macro cases with reasonaible expansion into rust.
        - sometimes requires a pair of macros to allow for reasobalbe portability

### Cross-checking
- instrument in original C and translated to rustc
- run programs with identical output
- options
    - online
        - using reMon MVEE
        - no log files
        - replicates program intput
        - limited compatibility
    - offline
        - broad compatibility
- multi variant execution env
- variant1 -> monitor (kernal) -> variant2
- clang plugin for C
- rustc plugin for Rust


## Closing Keynote: How to build a game in Rust and What I can learn from it
an answerL the design of a medium sized game engine
- Rust makes certain patterns more painful than others, which is a good thing
- The pstterns easiest for Rust are very often the easiest generally
- I had to learn good patterns the hard way
- For games, one of these is ECS design
- Rust rewards data-oriented design with clear ownership

- starting with a game
    - simplest possible engine is a global mutable game state with 'systems'
    - but then everything is in a global mutable megastate that can cause procedural issues eg mutating the physics state

- improving the design poorly
    - eg object oriented design
        - with single responsibility principle
        - encapsulation - lean towards privacy
        - abstraction - rather than depending on a class, you should lean toward an abstract interface
        - minimal coupling - dont couple things, only interact with these interaces
