use crate::{run, select};

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
        let namespaces = run("kubectl", &["get", "ns", "-o", "jsonpath={..name}"]);
        let namespaces = namespaces.split(" ").collect::<Vec<&str>>();

        let mut ns = Namespace::current();
        /*let default = namespaces.iter()
            .position(|&x|x == ns.name)
            .unwrap();
        */
        ns.name = match select(namespaces, 0) {
            Ok(name) => {String::from(name)}
            Err(_) => { panic!("ERROR: No namespace selected")}
        };
        ns
    }
}