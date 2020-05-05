// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return the default of Person
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person object with the results
impl From<&str> for Person {
    fn from(s: &str) -> Person {
        let mut sp = s.split(",");
        if let Some(name_str) = sp.next() {
            if name_str.len() == 0 {
                return Person::default();
            }
            let name = name_str.to_string();
            if let Some(age_str) = sp.next() {
                if let Ok(age) = age_str.parse() {
                    return Person { name, age };
                }
            }
        }
        Person::default()
    }
}
use std::str::FromStr;
use std::fmt::Error;
impl FromStr for Person {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(",");
        if let Some(name_str) = sp.next() {
            if name_str.len() == 0 {
                return Err(Error);
            }
            let name = name_str.to_string();
            if let Some(age_str) = sp.next() {
                if let Ok(age) = age_str.parse() {
                    return Ok(Person { name, age });
                }
            }
        }
        Err(Error)
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    let p3: Person = "Tom,42".parse().unwrap();
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_good_parse() {
        // Test that "Mark,20" works
        let pr: Result<Person, Error> = "Mark,20".parse();
        assert!(pr.is_ok());
        let p = pr.unwrap();
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    fn test_bad_parse() {
        // Test that "Mark,twenty" fails
        let pr: Result<Person, Error> = "Mark,20".parse();
        assert!(pr.is_err());
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark.twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
