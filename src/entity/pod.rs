use crate::entity::namespace::Namespace;
use crate::engine::{run, select};

pub struct Pod {
    pub name: String,
    pub namespace: Namespace,
}

impl Pod {
    pub fn new() -> Pod {
        Pod {
            name: "".to_string(),
            namespace: Namespace::new()
        }
    }

    pub fn select(namespace: Namespace) -> Pod {
        let pods = run("kubectl", &["get", "pods",
            "--no-headers", "-o", "custom-columns=:metadata.name",
            "-n", &namespace.name]);
        let pods = pods.split("\n").collect::<Vec<&str>>();

        let mut pod = Pod::new();
        pod.namespace = namespace;
        let default = 0;
        pod.name = match select(pods, default) {
            Ok(name) => {String::from(name)}
            Err(_) => { panic!("ERROR: No namespace selected")}
        };
        pod
    }
}