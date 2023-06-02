#![no_main]
use libfuzzer_sys::fuzz_target;
use lexpr::{sexp, Value};

fuzz_target!(|input: u64| {


    assert_eq!(sexp!(123), Value::from(123));
    assert_eq!(sexp!(123.4), Value::from(123.4));
    assert_eq!(sexp!(-123), Value::from(-123));
    assert_eq!(sexp!(-64.0), Value::from(-64.0));

    // test to break number parse
    sexp!(u64);


});


