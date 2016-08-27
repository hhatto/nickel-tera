extern crate nickel;
extern crate tera;
extern crate typemap;
extern crate plugin;

pub use middleware::{TeraMiddleware, TeraRequestExtensions};

mod middleware;
