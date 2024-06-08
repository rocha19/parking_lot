use parking_lot::infra::http::framework::{actix, axum};

fn main() {
    axum::server().unwrap_or_else(|err| {
        eprintln!("Error starting server: {}", err);
        std::process::exit(1);
    });
}
