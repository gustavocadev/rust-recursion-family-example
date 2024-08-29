#[derive(Clone)]
struct Person {
    name: String,
    age: u8,
    children: Vec<Person>,
}

fn print_family_names(people: Vec<Person>) -> () {
    for p in &people {
        println!("{} {}", p.name, p.age);

        print_family_names(p.children.to_vec());
    }
}

fn main() {
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            children: vec![],
        },
        Person {
            name: "Bob".to_string(),
            age: 35,
            children: vec![Person {
                name: "Charlie".to_string(),
                age: 5,
                children: vec![],
            }],
        },
        Person {
            name: "Dave".to_string(),
            age: 55,
            children: vec![Person {
                name: "Eve".to_string(),
                age: 25,
                children: vec![Person {
                    name: "Grace".to_string(),
                    age: 2,
                    children: vec![],
                }],
            }],
        },
    ];

    print_family_names(people)
}
