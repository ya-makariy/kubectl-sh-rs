use clap::Parser;

mod engine;
mod entity;
use crate::engine::shell;
use crate::entity::container::Container;
use crate::entity::namespace::Namespace;
use crate::entity::pod::Pod;

/// Simple program to access shell of pod's container through interactive selections
#[derive(Parser, Debug)]
#[clap(author="Author: Makariy Balashov, tg: @ya-makariy", version, about, long_about = None)]
struct Args {
    /// If present, use the requested namespace.
    #[clap(short, long)]
    namespace: Option<String>,
}

fn main() {

    let args = Args::parse();

    let n = match args.namespace {
        None => Namespace::select(),
        Some(namespace) => {
            match Namespace::test(&namespace) {
                true => Namespace {
                    name: namespace
                },
                false => {
                    println!("{} did not match any namespace", namespace);
                    Namespace::select()
                }
            }
        }
    };

    println!("Select pod: ");
    let p = Pod::select(n);

    println!("Select container: ");
    let c = Container::select(p);

    shell(c);
}