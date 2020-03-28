extern crate mylib;
use std::env;
use std::ffi::OsString;
use std::io;
use std::process;

enum Cmd {
    Quit,
    Astro,
}

static ASTRO_URL: &str = "http://api.open-notify.org/astros.json";

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
        match get_cmd() {
            Cmd::Quit => break,
            Cmd::Astro => {
                let url = String::from(ASTRO_URL);
                mylib::write_to(&mut port, &url).unwrap();
                let s = mylib::read_from(&mut port).unwrap();
                println!("{}", s);
            }
        }
    }
    println!("exit process.");
    process::exit(0);
}

fn get_cmd() -> Cmd {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = input.to_uppercase();
    if cmd.starts_with("Q") {
        Cmd::Quit
    } else {
        Cmd::Astro
    }
}
