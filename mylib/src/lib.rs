extern crate serialport;

use std::ffi::OsString;
use std::io;
use std::io::Write;

pub fn open_serialport(
    port_name: &OsString,
) -> Result<Box<dyn serialport::SerialPort>, serialport::Error> {
    let port = serialport::open(port_name)?;
    Ok(port)
}

pub fn write_to(port: &mut Box<dyn serialport::SerialPort>, s: &str) -> Result<(), io::Error> {
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

pub fn read_from(port: &mut Box<dyn serialport::SerialPort>) -> Result<String, io::Error> {
    let mut buf: Vec<u8> = vec![];
    loop {
        let mut ch = vec![0];
        let size = port.read(&mut ch);
        if let Ok(_) = size {
            if ch[0] == 0x2 {
                // 電文開始
                loop {
                    let size = port.read(&mut ch);
                    if let Ok(_) = size {
                        if ch[0] == 0x3 {
                            break;
                        }
                        buf.push(ch[0]);
                    }
                }
                break;
            }
        }
    }
    Ok(String::from_utf8(buf).unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
