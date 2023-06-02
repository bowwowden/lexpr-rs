#![no_main]
use libfuzzer_sys::fuzz_target;
use lexpr::{sexp, Value};

fuzz_target!(|input: u16| {

    assert_eq!(sexp!(123), Value::from(123));
    assert_eq!(sexp!(123.4), Value::from(123.4));
    assert_eq!(sexp!(-123), Value::from(-123));
    assert_eq!(sexp!(-64.0), Value::from(-64.0));

    // test to break number parse
    assert_eq!(sexp!(input), Value::from(input));

    // testing array
    // taken from sexp-macro - to show it takes variables inside the sexp! macro.
    let three = 3;
    assert_eq!(sexp!((1 2 ,three)), Value::list(vec![1, 2, 3]));

    // fails with random numbers though why?
    assert_eq!(sexp!((1 2 ,input)), Value::list(vec![1, 2, input]));
    

});


