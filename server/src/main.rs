extern crate mylib;
use std::env;
use std::ffi::OsString;
use std::process;

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() < 2 {
        eprintln!("<cmd> <serial port name>");
        process::exit(1);
    }

    let port_name = &args[1];
    let mut port = mylib::open_serialport(port_name).unwrap_or_else(|e| {
        let err = format!("could not open port {:?}. reason [{}]", port_name, e);
        eprintln!("{}", err);
        process::exit(1);
    });
    loop {
        let url = mylib::read_from(&mut port).unwrap();
        println!("data has come. [{}]", url);
        mylib::write_to(&mut port, &url).unwrap();
    }
}
