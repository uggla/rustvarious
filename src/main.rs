use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
    age: u8,
    phone: Vec<String>,
}

impl<'a> Person<'a> {
    pub fn new(first_name: &'a str, last_name: &'a str, age: u8) -> Person<'a> {
        Person {
            first_name,
            last_name,
            age,
            phone: vec![],
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

    //contacts.push(create_person("René", "Ribaud", 43));
    let mut dude = Person::new("René", "Ribaud", 43);
    dude.add_phone("0629215133");
    dude.add_phone("0476267003");

    contacts.push(dude);

    println!("{:#?}", contacts);
    println!("{}", &contacts[0]);
}
