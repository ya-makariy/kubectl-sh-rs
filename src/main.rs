extern crate dialoguer;

use std::process::Command;
use dialoguer::{Select, theme::ColorfulTheme};
use console::Term;

mod entity;
use entity::container;
use crate::container::Container;
use crate::entity::namespace::Namespace;
use crate::entity::pod::Pod;

fn main() {
    println!("Select namespace: ");
    let n = Namespace::select();

    println!("Select pod: ");
    let p = Pod::select(n);

    println!("Select container: ");
    let c = Container::select(p);

    let _ = Command::new("kubectl")
        .args(&["exec", "-i", "-t", "-n", &c.pod.namespace.name,
        &c.pod.name, "-c", &c.name,
        "--", "sh", "-c", "clear; (bash || sh)"])
        .status().expect("Failed to run shell");
}

fn run(cmd: &str, args: &[&str]) -> String {
    let output = Command::new(cmd).args(args).output().expect(
        &format!("failed to execute {}", cmd)
    );

    assert!(
        output.status.success(),
        "{}", format!("error running {} {:?}", cmd, args),
    );

    String::from_utf8(output.stdout)
        .map(|x| x.trim().to_string())
        .expect("couldn't convert stdout")
}

fn select(s: Vec<&str>, default: usize) -> std::io::Result<String> {

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&s)
        .default(default)
        .interact_on_opt(&Term::stderr())?.unwrap();

    let selection = String::from(s[selection]);
    Ok(selection)
}