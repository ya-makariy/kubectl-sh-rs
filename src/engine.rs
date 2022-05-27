
use std::process::Command;
use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use console::Term;
use crate::Container;

pub fn shell(container: Container) {
    Command::new("kubectl")
        .args(&["exec", "-i", "-t", "-n", &container.pod.namespace.name,
            &container.pod.name, "-c", &container.name,
            "--", "sh", "-c", "clear; (bash || sh)"])
        .status().expect("Failed to run shell");
}

pub fn run(cmd: &str, args: &[&str]) -> String {
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

pub fn select(s: Vec<&str>, default: usize) -> std::io::Result<String> {

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&s)
        .default(default)
        .interact_on_opt(&Term::stderr())?.unwrap();

    let selection = String::from(s[selection]);
    Ok(selection)
}