use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
enum Phonetype {
    Mobile,
    Work,
    Home,
}

#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: u8,
    phone: Vec<String>,
    phonetype: Phonetype,
}

impl<'a> Person<'a> {
    pub fn new(first_name: &'a str, last_name: &'a str, age: u8) -> Person<'a> {
        Person {
            first_name,
            last_name,
            age,
            phone: vec![],
            phonetype: Phonetype::Work,
        }
    }
    pub fn add_phone(&mut self, phone: &'a str) {
        self.phone.push(phone.to_string());
    }
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person name: {}, {}", self.first_name, self.last_name)
    }
}

fn main() {
    let mut contacts: Vec<Person> = Vec::new();

    let mut dude = Person::new("John", "Doe", 45);
    dude.add_phone("1010101010");
    dude.add_phone("1111111111");
    dude.add_phone("2222222222");
    dude.add_phone("3333333333");
    dude.add_phone("0565565656");

    contacts.push(dude);

    println!("{:#?}", contacts);
    println!("{}", &contacts[0]);
}
