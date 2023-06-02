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

    // are there floats that can break the parser?
    check_roundtrip_default(sexp!(1.5), "1.5");
    if !cfg!(feature = "fast-float-parsing") {
        check_roundtrip_default(sexp!(-1.0015065576612683), "-1.0015065576612683");
        check_roundtrip_default(sexp!(-1.360438755021694e308), "-1.360438755021694e308");
        check_roundtrip_default(sexp!(input), input_as_string);
    }

    // let bytes = lexpr::Value::bytes(vec![123u8; 1024]);
    // iter(|| black_box)

    // c.bench_function("byte vector serialization", |b| {
    //     b.iter(|| black_box(lexpr::to_string(&bytes)))
    // });

});