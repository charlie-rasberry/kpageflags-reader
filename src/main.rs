use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::ErrorKind;
use std::io::stdout;

fn main() -> Result<(), std::io::Error> {

    let f = File::open("/proc/kpageflags")?;  // open file the file as f
    let mut reader = BufReader::new(f);      // create a reader
    let mut buf = [0u8; 8];                 // create array for 64bits, [value;length]
    loop {
        match reader.read_exact(&mut buf) {
            Ok(_) => stdout().write_all(&buf)?,
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
