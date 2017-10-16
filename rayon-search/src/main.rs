#[macro_use]
extern crate error_chain;
extern crate rayon;

use rayon::prelude::*;

error_chain!{}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u32,
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.first_name == other.first_name &&
            self.last_name == other.last_name &&
            self.age == other.age
    }
}
fn run() -> Result<()> {
    let v = vec![6, 2, 1, 9, 3, 8, 11];
    // Note: |v.par_iter()| gives us an iterator that references
    // |v|, and the closure references those elements, thus we have
    // a reference to a reference of an element.
    let found = v.par_iter().find_any(|&&x| x == 9);
    let also_found = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);

    if let Some(found_val) = found {
        assert_eq!(*found_val, 9);
    }

    if let Some(also_found_val) = also_found {
        assert_eq!(*also_found_val, 8);
    }

    // Person example
    let v: Vec<Person> = vec![Person {first_name: String::from("John"),
                                      last_name: String::from("Smith"),
                                      age: 48},
                              Person {first_name: String::from("Will"),
                                      last_name: String::from("Smith"),
                                      age: 22},
                              Person {first_name: String::from("Will"),
                                      last_name: String::from("Iam"),
                                      age: 32},
                              Person {first_name: String::from("Jane"),
                                      last_name: String::from("Doe"),
                                      age: 29}];
    let p = v.par_iter().find_any(|&ref x| x.first_name == "Will" && x.age > 30);
    if let Some(found_p) = p {
        assert_eq!(Person {first_name: String::from("Will"),
                           last_name: String::from("Iam"),
                           age: 32},
                   *found_p);
    }
    Ok(())
}

quick_main!(run);
