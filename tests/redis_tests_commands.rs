use quiq_programming_exercise::{eval, print, read};

#[test]
fn command_test_set() {
    let input = String::from("SET mykey Hello");

    let tokens = read();
    let result = eval(tokens);
    print(result);
}

#[test]
fn command_test_get() {}

#[test]
fn command_test_del() {}
