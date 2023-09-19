use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    // hello world
    println!("Hi Rust!");

    // variable
    let my_string: &str = "This is a string";
    let mut my_string_mutable: &str = "This is a mutable string";
    //string
    println!("Variable: {}", my_string);
    println!("{my_string}");
    //mutable string
    println!("{my_string_mutable}");
    my_string_mutable = "Mut!";
    println!("{my_string_mutable}");

    let my_string2: String = String::from("This is other type of string");
    println!("{my_string2}");

    //numbers
    let mut my_int = 7;
    my_int = my_int + 5;

    println!("{my_int}");

    let my_float = 6.5;
    println!("{my_float}");

    let mut my_bool = false;
    println!("{my_bool}");
    my_bool = true;

    println!("{my_bool}");

    // Constants
    const MY_CONST: &str = "This is a constant";
    println!("{MY_CONST}");

    // control flow

    if my_int == 11 {
        println!("{my_int}")
    } else {
        println!("No if {my_int}")
    }

    // list

    let my_list = vec!["Etaelith", "Laryhan", "@etaelith", "36"];
    println!("{:#?}", my_list);
    println!("{:?}", my_list);
    println!("{}", my_list[0]);

    // sets

    let my_set: HashSet<&str> = vec!["Etaelith", "Laryhan", "@etaelith", "36"]
        .into_iter()
        .collect();
    println!("{:#?}", my_set);

    //maps

    let mut my_map: HashMap<&str, i32> = vec![("Etaelith", 28), ("Laryhan", 45), ("Eta", 25)]
        .into_iter()
        .collect();

    my_map.insert("Pato_perro", 22);
    println!("{:#?}", my_map);

    //bucles

    for value in &my_list {
        println!("{}", value);
    }

    for value in &my_set {
        println!("{}", value);
    }

    for (key, value) in &my_map {
        println!("{}: {}", key, value)
    }

    let mut my_counter = 0;
    while my_counter < my_list.len() {
        println!("{}", my_list[my_counter]);
        my_counter += 1;
    }

    my_function();

    let my_struct: MyStruct = MyStruct::new("Etaelith", 22);
    println!("{}: {}", my_struct.name, my_struct.age);
}

fn my_function() {
    println!("This is a function");
}

struct MyStruct {
    name: String,
    age: i32,
}

impl MyStruct {
    fn new(name: &str, age: i32) -> MyStruct {
        MyStruct {
            name: String::from(name),
            age,
        }
    }
}
