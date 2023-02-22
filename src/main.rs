#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server
        .listen("127.0.0.1:6767")
        .expect("Could not listen on port");
}

fn say_hello() -> String {
    String::from("Hello world!")
}
