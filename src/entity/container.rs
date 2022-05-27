use crate::entity::pod::Pod;
use crate::engine::{run, select};

pub struct Container {
    pub name: String,
    pub pod: Pod,
}

impl Container {
    pub fn new() -> Container {
        Container {
            name: "".to_string(),
            pod: Pod::new(),
        }
    }

    pub fn select(pod: Pod) -> Container {
        let containers = run("kubectl", &["get", "pods",
            &pod.name, "-o", "jsonpath={.spec.containers[*].name}",
            "-n", &pod.namespace.name]);
        let containers = containers.split(" ").collect::<Vec<&str>>();

        let mut container = Container::new();
        container.pod = pod;
        let default = 0;
        container.name = match select(containers, default) {
            Ok(name) => {String::from(name)}
            Err(_) => { panic!("ERROR: No namespace selected")}
        };
        container
    }
}