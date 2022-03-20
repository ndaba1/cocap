use std::env::vars;

pub fn check_config() {
    let vals = vars();

    dbg!(vals);
}
