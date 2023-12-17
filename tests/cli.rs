use std::process::Command;

use assert_cmd::Command as ACMD;



#[test]
fn run_cmd_omit_newline(){
    let old_res = Command::new("echo")
    .arg("-n")
    .arg("hello world")
    .output()
    .unwrap();

    let new_res = ACMD::cargo_bin("n_echo").unwrap()
    .arg("-n")
    .arg("hello world")
    .output()
    .unwrap();

    assert_eq!(String::from_utf8(old_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}
#[test]
fn run_cmd_with_newline(){
    let old_res = Command::new("echo")
    .arg("$PATH")
    .output()
    .unwrap();

    let new_res = ACMD::cargo_bin("n_echo").unwrap()
    .arg("$PATH")
    .output()
    .unwrap();

    assert_eq!(String::from_utf8(old_res.stdout).unwrap(),String::from_utf8(new_res.stdout).unwrap())
}

#[test]
fn run_cmd_with_text(){
    ACMD::cargo_bin("n_echo").unwrap()
    .assert()
    .failure();
}