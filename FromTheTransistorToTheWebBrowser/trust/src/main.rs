use std::{io, thread};

fn main() -> io::Result<()> {
    let mut i = trust::Interface::new()?;
    let mut l1 = i.bind(9000)?;
    let mut l2 = i.bind(9001)?;
    let jh1 = thread::spawn(move || {
        while let Ok(_stream) = l1.accept()  {
            eprintln!("got connection on 9000");
        }
    });

    let jh2 = thread::spawn(move || {
        while let Ok(_stream) = l2.accept()  {
            eprintln!("got connection on 9001");
        }
    });

    jh1.join().unwrap();
    jh2.join().unwrap();
    Ok(())
}
