use lazy_static::lazy_static;
use portpicker::Port;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref USED_PORTS: Arc<Mutex<HashSet<Port>>> = Arc::new(Mutex::new(HashSet::new()));
}

/// This function is an extension to the crate `pick_unused_port`
/// Given that we multithread e2e tests, and that each test spawn his own server
/// We have to make sure the port is not used by another test
/// That's how the pick_unused_port is supposed to work
/// However, it is not thread safe
/// This function keeps a mutex guard of all used ports and will generate other ports if they are already used
pub fn pick_unused_port() -> Port {
    for _ in 0..10 {
        match portpicker::pick_unused_port() {
            None => continue,
            Some(port) => {
                let mut used_ports = USED_PORTS.lock().unwrap();
                if !used_ports.contains(&port) {
                    used_ports.insert(port);
                    return port;
                }
            }
        }
    }

    panic!("No available port on system");
}

/// Allow the server to release port at the end of a test
pub fn release_port(port: Port) {
    USED_PORTS.lock().unwrap().remove(&port);
}
