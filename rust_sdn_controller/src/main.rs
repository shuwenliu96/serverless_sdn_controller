use std::net::{TcpListener};
use rust_sdn_controller::rust_controller::RustController;
use rust_sdn_controller::ofp_controller::OfpController;
use rust_sdn_controller::database::get_policy;

extern crate rust_ofp;



fn main() {
    let mut policy = get_policy().unwrap();
    policy.insert(3232235560, 3232235816);
    println!("Got policy: {:?}", policy);

    let listener = TcpListener::bind(("0.0.0.0", 6633)).unwrap();
    for stream in listener.incoming() {
        println!("{:?}", stream);
        match stream {
            Ok(mut stream) => {
                let policy_clone = policy.clone();
                std::thread::spawn(move || RustController::handle_client_connected(&mut stream, policy_clone));
            }
            Err(_) => {
                // connection failed
                panic!("Connection failed")
            }
        }
    }
}

