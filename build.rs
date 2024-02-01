use std::env;  
use copy_to_output::copy_to_output;  

fn main() {
    slint_build::compile("ui/appwindow.slint").unwrap();

    copy_to_output("res", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
