#[derive(Debug, Clone)]
struct w3b {
    name: String,
}



fn main() {
    let x: String = String::from("Hello World");
    println!("The value of x here is {x}");
    let x: u8 = 9;
    println!("The value of x is {}", x);

    let m = the_name_of_the_function(2, 4);
    println!("The function returned {m}");

    let gee = w3b { name: String::from("Test")};

    change_scope(gee.clone());

    change_scope(gee);

    println!("{}", m);
}

fn the_name_of_the_function(x:u8, y:u8) -> u16 {
    let z: u16 = (x * y).into();
    z
}

fn change_scope(x: w3b) {
    println!("{:?}", x);
}
