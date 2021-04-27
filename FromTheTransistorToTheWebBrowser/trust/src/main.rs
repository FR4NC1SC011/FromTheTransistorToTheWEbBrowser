use std::io::prelude::*;
use std::{io, thread};

fn main() -> io::Result<()> {
    let mut i = trust::Interface::new()?;
    eprintln!("created interface");
    let mut l1 = i.bind(9000)?;
    let jh1 = thread::spawn(move || {
        while let Ok(mut stream) = l1.accept() {
            eprintln!("got connection");
            loop {
                let mut buf = [0; 512];
                let n = stream.read(&mut buf[..]).unwrap();
                eprintln!("read {}b of data", n);
                if n == 0 {
                    eprintln!("no more data!");
                } else {
                    println!("got {:?}", &buf[..n]);
                }
            }
        }
    });

    jh1.join().unwrap();

    Ok(())
}
