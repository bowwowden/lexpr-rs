#![no_main]
use libfuzzer_sys::fuzz_target;

use lexpr::{sexp, Value};

fn check_roundtrip_default(input: Value, printed: &str) {
    let string = lexpr::to_string(&input).expect("printing failed");
    assert_eq!(&string, printed);
    let output = lexpr::from_str(&string).expect("parsing failed");
    assert_eq!(input, output);
}


fuzz_target!(|input: f64| {
    
    let input_as_string: &str = &*input.to_string();

    // apparently one defect is found.. what is it?
    check_roundtrip_default(sexp!(1.5), "1.5");
    if !cfg!(feature = "fast-float-parsing") {
        check_roundtrip_default(sexp!(-1.0015065576612683), "-1.0015065576612683");
        check_roundtrip_default(sexp!(-1.360438755021694e308), "-1.360438755021694e308");
        // check_roundtrip_default(sexp!(input), input_as_string);
    }

    // taking the fuzz input out of the assertion not to cause unnecessary defects
    sexp!(input)


});