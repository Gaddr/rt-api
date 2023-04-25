#[macro_use] extern crate diesel;
mod schema;
mod routes;

fn main() -> std::io::Result<()> {
    routes::main()
}
