use bobo::oop::*;

class! {
    Person {
        name: String
        age: u32

        fn greet(){
            println!("{}", format!("Hello, my name is {}.", self.name));
        }

        fn get_age(years: u32) -> u32 {
            self.age + years
        }
    }
}

fn main() {
    
    let person = Person {
        name: String::from("Tom"),
        age: 30
    };

    person.greet();

    println!("I am {} years old.", person.get_age(5));
}
