use std::fmt;

use procfs::{Current, CurrentSI, KernelStats};

#[allow(dead_code)]
#[derive(Debug)]
enum Phonetype {
    Mobile,
    Work,
    Home,
}

#[allow(dead_code)]
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

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person name: {}, {}", self.first_name, self.last_name)
    }
}

trait Topable {
    fn top(&self, n: usize) -> Self;
}

impl<T: std::cmp::Ord + Clone> Topable for Vec<T> {
    fn top(&self, n: usize) -> Self {
        let mut top = self.clone();
        top.sort();
        top.reverse();
        top.truncate(n);
        top
    }
}

fn main() {
    let mut contacts: Vec<Person> = Vec::new();
    let mut dude = Person::new("John", "Doe", 45);

    dude.add_phone("dsds");
    dude.add_phone("1010101010");
    dude.add_phone("1111111111");
    dude.add_phone("2222222222");
    dude.add_phone("3333333333");
    dude.add_phone("0565565656");
    dude.add_phone("0565565657");
    let mut dude2 = Person::new("Marie", "Doe", 45);
    dude2.add_phone("1010101010");
    contacts.push(dude);

    contacts.push(dude2);

    // for i in &mut contacts {
    //     i.first_name = "RRR";
    // }
    //
    // for i in &mut dude.phone {
    //     *i = "ddd".to_string();
    // }

    println!("{:#?}", contacts);
    println!("{}", &contacts[0]);

    let cpus = procfs::CpuInfo::current().unwrap();
    println!("Cpu cores: {}", cpus.num_cores());
    println!("Cpu model:{}", cpus.model_name(0).unwrap());
    let k = KernelStats::current().unwrap();
    println!("Kernel user time: {}", k.total.user_ms());

    let truc = vec![1, 2, 3, 4, 5];
    dbg!(&truc);

    let truc2 = truc.top(3);
    dbg!(&truc2);
}
