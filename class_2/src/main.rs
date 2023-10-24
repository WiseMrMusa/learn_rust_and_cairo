fn main() {
    another_function();
    array();
    complex();
}

fn another_function() {
    let _a = -4;
    let _b : u8 = 72;
    let _c: u128 = 78;
    let _d: u32 = 647;
    println!("{_b} Hello, {_a} world! {_c}");
    // Structures (class) and Traits (behaviour, methods)
    // Data Structure and Algorithm
}

// (a + bi)
#[derive(Debug)]
struct ComplexNumber {
    a: u8,
    b: u8
}

fn array() {
    let a: [u8; 3] = [3,5,6];
    println!("This is an array: {a:?}");
    // {} -> std::fmt::Display
    // {:?} -> std::fmt::Debug
}

fn complex() {
    let b: ComplexNumber = ComplexNumber { a: 7, b: 8 };
    print!("This is a complex number: {b:?}");
}