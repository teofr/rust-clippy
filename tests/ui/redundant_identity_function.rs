#![warn(clippy::redundant_identity_function)]
#![allow(
    dead_code,
    unused_variables,
    clippy::needless_return,
    clippy::let_and_return,
    clippy::redundant_locals,
    clippy::needless_lifetimes,
    clippy::extra_unused_type_parameters
)]

use std::fmt::Display;

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

fn ref_id(x: &u32) -> &u32 {
    //~^ redundant_identity_function
    x
}

fn lifetime_id<'a>(x: &'a str) -> &'a str {
    //~^ redundant_identity_function
    x
}

const fn const_id(x: u32) -> u32 {
    //~^ redundant_identity_function
    x
}

// Inherent impl method — should lint
struct Bar;

impl Bar {
    fn inherent_id(x: u32) -> u32 {
        //~^ redundant_identity_function
        x
    }
}

// #[expect] test
#[expect(clippy::redundant_identity_function)]
fn expected_id(x: u32) -> u32 {
    x
}

// =============================
// Negative cases (should NOT lint)
// =============================

// --- Trait impls (required by contract) ---

trait MyTrait {
    fn process(x: u32) -> u32;
}

struct Foo;

impl MyTrait for Foo {
    fn process(x: u32) -> u32 {
        x
    }
}

// --- Default trait methods ---

trait AnotherTrait {
    fn default_id(x: u32) -> u32 {
        x
    }
}

// --- Multiple parameters ---

fn two_params(x: u32, _y: u32) -> u32 {
    x
}

// --- No parameters ---

fn no_params() -> u32 {
    42
}

// --- Transforms the value ---

fn add_one(x: u32) -> u32 {
    x + 1
}

fn to_string_fn(x: u32) -> String {
    x.to_string()
}

// --- extern "C" ABI ---

extern "C" fn c_identity(x: u32) -> u32 {
    x
}

// --- #[no_mangle] ---

#[unsafe(no_mangle)]
fn exported_id(x: u32) -> u32 {
    x
}

// --- #[export_name] ---

#[unsafe(export_name = "my_identity")]
fn named_export_id(x: u32) -> u32 {
    x
}

// --- Unsafe function ---

unsafe fn unsafe_id<T>(x: T) -> T {
    x
}

// --- Closures (handled by other lints) ---

fn closures_not_linted() {
    let _f = |x: u32| x;
    let _g = |x: u32| -> u32 { x };
}

// --- Not actually identity ---

fn not_identity(x: u32) -> u32 {
    let _y = x;
    42
}

fn with_side_effect(x: u32) -> u32 {
    println!("hello");
    x
}

// --- Async functions (return a Future, not the value) ---

async fn async_id(x: u32) -> u32 {
    x
}

// --- Functions with trait bounds (not equivalent to identity) ---

fn bounded_id<T: Clone>(x: T) -> T {
    x
}

fn display_id<T: Display>(x: T) -> T {
    x
}

fn where_clause_id<T>(x: T) -> T
where
    T: Clone + Default,
{
    x
}

// --- impl Trait in argument position ---

fn impl_trait_arg(x: impl Display) -> impl Display {
    x
}

// --- impl Trait only in return position ---

fn impl_trait_return(x: u32) -> impl Display {
    x
}

// --- Multiple generic type params with only one used ---

fn extra_generic<T, U>(x: T) -> T {
    x
}

// --- Lifetime bounds that narrow ---

fn lifetime_narrowing<'a: 'b, 'b>(x: &'a str) -> &'b str {
    x
}

// --- Function with #[inline] (intentional for codegen) ---

#[inline(always)]
fn inlined_id(x: u32) -> u32 {
    //~^ redundant_identity_function
    x
}
