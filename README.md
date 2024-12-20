## BOBO  
an elegant and powerful rust development tool library  
> In development, not recommended. The features listed in the documentation are implemented.  

### Quick Start  

Install: `cargo add bobo`  

```rust
use bobo::oop::*;

class! {
    Person {
        name: String
        age: u32

        fn greet() {
            println!("{}", format!("Hello, my name is {}.", self.name));
        }
    }
}

fn main() {
    let person = Person {
        name: String::from("Tom"),
    };

    person.greet();
}
```

## A more complex example  

Create multiple classes with multiple properties and methods.  

```rust
use bobo::oop::*;

class! {

    Person {

        name: String
        age: u32

        fn greet() {
            println!("{}", format!("Hello, my name is {}.", self.name));
        }

        fn get_age(years: u32) -> u32 {
            self.age + years
        }
    }

    Animal {

        species: String
        age: u32

        fn speak() {
            println!("The {} makes a sound.", self.species);
        }

        fn age_in_human_years() -> u32 {
            self.age * 7
        }
    }

}
```

Create a class using a constructor named `new`:   

```rust
use bobo::oop::*;

fn main() {
    let person = Person::new("Alice", 30);
    person.greet();
}

class! {
    Person {
        name: String
        age: u32

        fn new(name: &str, age: u32) -> Self {
            Self {
                name: name.to_string(),
                age
            }
        }

        fn greet() {
            println!("{}", format!("I'm {}.", self.name));
        }
    }
}
```