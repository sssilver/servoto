#![feature(proc_macro)]
#[macro_use(bson, doc)]
extern crate bson;
extern crate crossbeam;
extern crate curl;
extern crate mongodb;
extern crate num_cpus;
extern crate rexiv2;
extern crate rotor;
extern crate rotor_http;
#[macro_use]
extern crate serde_derive;
extern crate xmltree;

mod context;
mod download;
mod error;
mod photo;
mod process;
mod storage;
mod storage_class;
mod waldo_service;

use context::Context;
use process::Processor;
use rotor::mio::tcp::TcpListener;
use rotor_http::server::Fsm;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use storage::Storage;
use waldo_service::WaldoService;


fn main() {
    let listen_address: SocketAddr = "0.0.0.0:3000".parse()
        .expect("Unable to parse socket address to listen to");

    println!("Listening for HTTP connections on {:?}", listen_address);
    let listen = TcpListener::bind(&listen_address)
        .expect(&format!("Unable to listen on the socket for {:?}", listen_address));

    let num_threads: u32 = env::var("WALDO_THREADS").unwrap_or("1".to_string()).parse().unwrap();

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let listener = listen.try_clone()
            .expect("Unable to clone the listening socket");

        threads.push(thread::spawn(move || {
            let event_loop = rotor::Loop::new(&rotor::Config::new())
                .expect("Unable to create the event loop");

            // Create one storage connection per thread
            let storage = Storage::new("localhost", 27017)
                .expect("Unable to connect to the storage");
            let storage = Arc::new(storage);

            let processor = Processor::new(storage.clone(), num_cpus::get());

            let mut loop_inst = event_loop.instantiate(Context {
                database: storage,
                processor: processor
            });

            loop_inst.add_machine_with(|scope| {
                Fsm::<WaldoService, _>::new(listener, (), scope)
            }).expect("Unable to create the WaldoService machine");

            loop_inst.run()
                .expect("Unable to start the event loop");
        }));
    }

    for thread in threads {
        thread.join()
            .expect("Thread panic!");
    }
}