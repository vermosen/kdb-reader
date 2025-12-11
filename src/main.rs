mod configuration;

extern crate clap;
extern crate chrono;
extern crate kdbplus;

use kdbplus::ipc::*;
use kdbplus::qtype;

use configuration::Configuration;

fn main() {
    
    let conf = Configuration::new();
}
