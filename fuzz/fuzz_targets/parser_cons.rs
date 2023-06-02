#![no_main]
use libfuzzer_sys::fuzz_target;
use lexpr::{sexp, Value};

fuzz_target!(|input: u16| {

    assert_eq!(
        sexp! {(a . 64)},
        Value::cons(Value::symbol("a"), Value::from(64))
    );

    // test fuzz
    assert_eq!(
        sexp!({a . input}),
        Value::cons(Value::symbol("a", Value::from(input)))
    );
    

});