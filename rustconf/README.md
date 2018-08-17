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






## mentat








## Embedding Rust in C/C++







## C2Rust: Migrating Legacy Code to Rust
