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
        let mut ch = vec![0];
        let size = port.read(&mut ch).unwrap_or(0);
        if size == 0 {
            continue;
        }
        if ch[0] == 0x2 {
            // 電文開始
            let mut buf: Vec<u8> = vec![];
            loop {
                let size = port.read(&mut ch).unwrap_or(0);
                if size == 0 {
                    continue;
                }
                // println!("{:?}", ch);
                if ch[0] == 0x3 {
                    break;
                }
                buf.push(ch[0]);
            }
            let url = String::from_utf8(buf).unwrap();
            mylib::write_serial(&mut port, &url).unwrap();
            println!("{}", url);
        }
    }
}
