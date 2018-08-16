/// This is documentation
pub fn main() {
    let mut apples = 100;
    add_fifty(apples);
    apples += 50;
    println!("I have {} apples", apples);
    // let basic_array = [0,1,2,3,4,5];
    // println!("{:?}", basic_array);
    // println!("{:#?}", basic_array);

    // lambda declaraion
    let foo = [0,1,2,3,4,5];
    let f = |_x| println!("{:?}", foo);
    f(1);
    demonstrate_match();
    demonstrate_enum();
}

fn add_fifty(n: i32) -> i32 {
    // for i in 0..10 {
    //     println!("Number {:?}", i);
    // }
    // let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
    // for name in names.iter() {
    //     println!("Hi {:?}!", name);
    // }
    n + 50
}

// variables4.rs
// Make me compile! Scroll down for hints :)

fn demonstrate_match() {
    let age = 167u32;
    match age {
        0...150 => println!("You're too small to go on the rollercoaster."),
        150...200 => println!("You may go on the rollercoaster!"),
        _ => {
            println!("You're too tall to go on the rollercoaster.");
        },
    }

    // demonstrate_array()
    let mut color = [255, 0 , 255];
    color[0] = 100;
    let index = 3;
    println!("The color is {:?}", color[index]);
}

enum GameType {
    SinglePlayer,
    MultiPlayer(u32),
}

fn demonstrate_enum() -> GameType {
    let game = GameType::MultiPlayer(4);
    match game {
        GameType::SinglePlayer => println!("How about solitaire?"),
        GameType::MultiPlayer(2) => println!("How about checkers?"),
        GameType::MultiPlayer(4) => println!("How about bridge?"),
        GameType::MultiPlayer(num) => {
            println!("How about {}-player tag?", num)
        },
    }

    let a = Some("Arisha");

    return game;
}

#[test]
fn new_person_setup() {
    // let p1 = Person::new();
    assert!(true);
    assert_eq!(true, true);

    // let p2 = Person::new();
    assert_ne!(true, false);
}
