extern crate serialport;
use std::env;
use std::ffi::OsString;
use std::io;
use std::io::Write;
use std::process;

enum Cmd {
    Quit,
    Astro,
}

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() < 2 {
        eprintln!("usage <cmd> <port name>");
        process::exit(1);
    }

    let port_name = &args[1];
    let mut port = serialport::open(port_name).expect(&format!("port[{:?}]", port_name));
    println!("port {:?} opened.", port_name);

    loop {
        match get_cmd() {
            Cmd::Quit => break,
            _ => {
                let url = String::from("http://api.open-notify.org/astros.json");
                write_serial(&mut port, &url).unwrap();
            }
        }
    }
    println!("client exit");
    process::exit(0);
}

fn write_serial(port: &mut Box<dyn serialport::SerialPort>, s: &str) -> Result<(), io::Error> {
    let mut data: Vec<u8> = vec![0x2];
    data.append(&mut String::from(s).into_bytes());
    data.push(0x3);
    let mut slice = &data[..];
    // 送信
    loop {
        match port.write(slice) {
            Ok(size) => {
                if size == slice.len() {
                    break;
                }
                slice = &data[size..];
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
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
