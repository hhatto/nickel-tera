#[macro_use]
extern crate nickel;
extern crate nickel_tera;
extern crate tera;

use std::env;
use std::path::Path;
use tera::Context;
use nickel::{Nickel, HttpRouter};
use nickel_tera::{TeraMiddleware, TeraRequestExtensions};

fn main() {
    let mut server = Nickel::new();

    let root_path = env::current_dir().unwrap();
    let template_dir = root_path.join(Path::new("templates/*.tera"));
    server.utilize(TeraMiddleware::new(template_dir.to_str().unwrap()));

    server.get("/*",
               middleware! { |req, res|
        let mut ctx = Context::new();
        let foo = "";
        let hello = "world";
        let vector = vec![1, 3, 6];
        ctx.add("foo", &foo);
        ctx.add("hello", &hello);
        ctx.add("vector", &vector);
        req.template_engine().render("index.tera", ctx).unwrap()
    });

    server.listen("127.0.0.1:6767");
}
