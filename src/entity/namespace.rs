use crate::engine::{run, select};

pub struct Namespace {
    pub name: String,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace { name: "".to_string() }
    }

    fn current() -> Namespace{
        Namespace {
            name: run("kubectl", &["config", "view", "--minify", "-o", "jsonpath={..namespace}"])
        }
    }

    pub fn select() -> Namespace {
        println!("Select namespace: ");

        let namespaces = run("kubectl", &["get", "ns",
            "--no-headers", "-o", "custom-columns=:metadata.name",]);
        let namespaces = namespaces.split("\n").collect::<Vec<&str>>();

        let mut ns = Namespace::current();

        ns.name = match select(namespaces, 0) {
            Ok(name) => {String::from(name)}
            Err(_) => { panic!("ERROR: No namespace selected")}
        };
        ns
    }

    pub fn test(namespace: &String) -> bool {
        let namespaces = run("kubectl", &["get", "ns",
            "--no-headers", "-o", "custom-columns=:metadata.name",]);

        namespaces.contains(namespace)
    }
}