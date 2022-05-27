use crate::engine::{run, select};

pub struct Namespace {
    pub name: String,
}

impl Namespace {
    pub fn new() -> Namespace {
        Namespace { name: "".to_string() }
    }

    pub fn current() -> Namespace{
        Namespace {
            name: run("kubectl", &["config", "view", "--minify", "-o", "jsonpath={..namespace}"])
        }
    }

    pub fn select() -> Namespace {
        let namespaces = run("kubectl", &["get", "ns",
            "--no-headers", "-o", "custom-columns=:metadata.name",]);
        let namespaces = namespaces.split("\n").collect::<Vec<&str>>();

        let mut ns = Namespace::current();
        let default = namespaces.iter()
            .position(|&x|x == ns.name)
            .expect("No default ns");

        ns.name = match select(namespaces, default) {
            Ok(name) => {String::from(name)}
            Err(_) => { panic!("ERROR: No namespace selected")}
        };
        ns
    }
}