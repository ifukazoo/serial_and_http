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

pub fn write_serial(port: &mut Box<dyn serialport::SerialPort>, s: &str) -> Result<(), io::Error> {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
