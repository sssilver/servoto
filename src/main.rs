extern crate rotor;
extern crate rotor_http;

mod context;
mod waldo_service;

use rotor_http::server::Fsm;
use rotor::mio::tcp::TcpListener;
use std::env;
use std::thread;

use waldo_service::WaldoService;
use context::Context;


fn main() {
    let listen_address = "0.0.0.0:3000";

    println!("Listening for HTTP connections on {}", listen_address);
    let listen = TcpListener::bind(&listen_address.parse().unwrap()).unwrap();
    let num_threads: u32 = env::var("THREADS").unwrap_or("1".to_string()).parse().unwrap();

    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let listener = listen.try_clone().unwrap();
        threads.push(thread::spawn(move || {
            let event_loop = rotor::Loop::new(&rotor::Config::new()).unwrap();
            let mut loop_inst = event_loop.instantiate(Context { });
            loop_inst.add_machine_with(|scope| {
                Fsm::<WaldoService, _>::new(listener, (), scope)
            }).unwrap();
            loop_inst.run().unwrap();
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
}