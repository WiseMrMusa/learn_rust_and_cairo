

fn test_returned_val(value: &str) -> String {
    println!("This is the string value: {:?}", value);
    String::from(value)
}


#[derive(Debug)]
struct Person <'a> {
    email: &'a str,
    password: &'a str,
}

trait PersonTrait<'a> {
    fn new() -> Person<'a>;
    fn run() -> ();
    fn walk() -> i32;
    fn sound() -> String;
}

impl<'a> PersonTrait<'a> for Person<'a> {
    fn new() -> Person<'a> {
        Self {
            email: "Test@gmail.com",
            password: "Test",
        }
    }

    fn run() -> () {

    }

    fn walk() -> i32 {
        5
    }

    fn sound() -> String {
        "Cry".to_owned()
    }
}



impl <'a> Person<'a> {
    fn new(email: &'a str, password: &'a str) -> Self {
        Self {
            email,
            password
        }
    }
}


fn main() {

    let mut owned_string: String = String::from("Owned String");

    let mut string_slice: &str = " Test";

    owned_string.push_str(string_slice);
    
    println!("{:?}", owned_string);

    let mut convert_str_to_String: String = " Test".to_owned();

    let mut convert_String_to_str: &str = String::from("Test String").as_ref();
    

    let returned_string: String = test_returned_val("Web 3");

    println!("{:?}", returned_string);

    let new_user: Person = Person::new("John Doe", "Johndoe@email.com");

    println!("User 1: {:?}", new_user);

}
