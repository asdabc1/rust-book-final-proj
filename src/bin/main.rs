extern crate rust_book_server_project;

use std::net::TcpListener;
use rust_book_server_project::*;
use rust_book_server_project::thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();

        pool.execute(|| handle_connection(stream))
    }
}
