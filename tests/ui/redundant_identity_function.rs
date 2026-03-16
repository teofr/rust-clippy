#![warn(clippy::redundant_identity_function)]
#![allow(dead_code, unused_variables, clippy::needless_return, clippy::let_and_return)]

// ============================
// Positive cases (should lint)
// ============================

fn simple_id<T>(x: T) -> T {
    //~^ redundant_identity_function
    x
}

fn concrete_id(x: u32) -> u32 {
    //~^ redundant_identity_function
    x
}

fn with_return<T>(x: T) -> T {
    //~^ redundant_identity_function
    return x;
}

fn with_block<T>(x: T) -> T {
    //~^ redundant_identity_function
    { x }
}

fn with_block_and_return<T>(x: T) -> T {
    //~^ redundant_identity_function
    {
        return x;
    }
}

fn with_let_chain<T>(x: T) -> T {
    //~^ redundant_identity_function
    let y = x;
    y
}

fn with_let_and_return<T>(x: T) -> T {
    //~^ redundant_identity_function
    let y = x;
    return y;
}

fn with_multi_let<T>(x: T) -> T {
    //~^ redundant_identity_function
    let y = x;
    let z = y;
    z
}

fn string_id(s: String) -> String {
    //~^ redundant_identity_function
    s
}

// =============================
// Negative cases (should NOT lint)
// =============================

// Trait implementations — required by contract
trait MyTrait {
    fn process(x: u32) -> u32;
}

struct Foo;

impl MyTrait for Foo {
    fn process(x: u32) -> u32 {
        x
    }
}

// Default trait methods
trait AnotherTrait {
    fn default_id(x: u32) -> u32 {
        x
    }
}

// Multiple parameters — not a simple identity
fn two_params(x: u32, _y: u32) -> u32 {
    x
}

// No parameters
fn no_params() -> u32 {
    42
}

// Transforms the value
fn add_one(x: u32) -> u32 {
    x + 1
}

fn to_string(x: u32) -> String {
    x.to_string()
}

// extern "C" ABI
extern "C" fn c_identity(x: u32) -> u32 {
    x
}

// #[no_mangle]
#[unsafe(no_mangle)]
fn exported_id(x: u32) -> u32 {
    x
}

// #[export_name]
#[unsafe(export_name = "my_identity")]
fn named_export_id(x: u32) -> u32 {
    x
}

// Unsafe function
unsafe fn unsafe_id<T>(x: T) -> T {
    x
}

// Closures are handled by other lints
fn closures_not_linted() {
    let _f = |x: u32| x;
    let _g = |x: u32| -> u32 { x };
}

// Function that returns a different variable
fn not_identity(x: u32) -> u32 {
    let _y = x;
    42
}

// Function with side effects (not a pure identity)
fn with_side_effect(x: u32) -> u32 {
    println!("hello");
    x
}
