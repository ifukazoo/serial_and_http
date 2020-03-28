extern crate mylib;

use std::env;
use std::ffi::OsString;
use std::io;
use std::process;

enum Cmd {
    Quit,
    Astro,
    Coin,
    None,
}

static ASTRO_URL: &str = "http://api.open-notify.org/astros.json";
static COIN_URL: &str = "https://api.coindesk.com/v1/bpi/currentprice.json";

fn main() {
    let args: Vec<OsString> = env::args_os().collect();

    if args.len() < 2 {
        eprintln!("client> <cmd> <serial port name>");
        process::exit(1);
    }

    let port_name = &args[1];
    let mut port = mylib::open_serialport(port_name).unwrap_or_else(|e| {
        let err = format!("could not open port {:?}. reason [{}]", port_name, e);
        eprintln!("client> {}", err);
        process::exit(1);
    });

    loop {
        // 端末入力を待つ
        println!("client> waiting cmd ...");
        let cmd = get_cmd();

        match cmd {
            Cmd::Quit => break,

            Cmd::Astro | Cmd::Coin => {
                let url = match cmd {
                    Cmd::Astro => String::from(ASTRO_URL),
                    Cmd::Coin => String::from(COIN_URL),
                    _ => panic!("must not reach"),
                };

                // サーバー側にURLを送信
                println!("client> request to server.");
                mylib::write_to(&mut port, &url).unwrap();

                // レスポンス待ち
                let s = mylib::read_from(&mut port).unwrap();
                println!("client> response from server. [{}]", s);
            }
            Cmd::None => usage(),
        }
    }
    println!("client> exit process.");
    process::exit(0);
}

fn usage() {
    println!("client>");
    println!("command not found.");
    println!(r#"  "astro""#);
    println!(r#"  "coin""#);
    println!("are available.");
}

fn get_cmd() -> Cmd {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cmd = input.to_uppercase();
    if cmd.starts_with("Q") {
        Cmd::Quit
    } else if cmd.starts_with("ASTRO") {
        Cmd::Astro
    } else if cmd.starts_with("COIN") {
        Cmd::Coin
    } else {
        Cmd::None
    }
}
