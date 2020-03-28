extern crate mylib;
extern crate reqwest;

use std::env;
use std::ffi::OsString;
use std::process;

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() < 2 {
        eprintln!("server> <cmd> <serial port name>");
        process::exit(1);
    }

    let port_name = &args[1];
    let mut port = mylib::open_serialport(port_name).unwrap_or_else(|e| {
        let err = format!("could not open port {:?}. reason [{}]", port_name, e);
        eprintln!("server> {}", err);
        process::exit(1);
    });
    loop {
        println!("server> waiting from client ...");

        // クライアントからのURLを待つ
        let url = mylib::read_from(&mut port).unwrap();
        println!("server> data has come. [{}]", url);

        // HTTP Request
        let res = match request(&url) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("server> error happened.");
                e
            }
        };
        println!("server> response to.");
        mylib::write_to(&mut port, &res).unwrap();
    }
}

fn request(url: &str) -> Result<String, String> {
    let result = reqwest::blocking::get(url);
    if let Err(result) = result {
        Err(result.to_string())
    } else {
        let text = result.unwrap().text();
        match text {
            Ok(text) => Ok(text),
            Err(e) => Err(e.to_string()),
        }
    }
}
