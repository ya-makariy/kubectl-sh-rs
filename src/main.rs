extern crate dialoguer;

mod engine;
mod entity;
use crate::engine::shell;
use crate::entity::container::Container;
use crate::entity::namespace::Namespace;
use crate::entity::pod::Pod;

fn main() {
    println!("Select namespace: ");
    let n = Namespace::select();

    println!("Select pod: ");
    let p = Pod::select(n);

    println!("Select container: ");
    let c = Container::select(p);

    shell(c);
}