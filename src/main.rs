use project_structure::{
    entity::{entity_bar, entity_foo},
    hello_world::hello,
};

fn main() {
    hello::hello_world();
    let foo = entity_foo::Foo {
        name: "Foo".to_string(),
    };
    println!("{0}", foo.name);
    let bar = entity_bar::Bar {
        name: "Bar".to_string(),
    };
    println!("{0}", bar.name);
}
