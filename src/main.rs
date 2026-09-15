use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::stdout;

fn main() -> Result<(), std::io::Error> {

    let path = "/proc/kpageflags";      // set the path
    let f = File::open(path)?;          // open file, set it to f
    let mut reader = BufReader::new(f); // create a reader

    let mut buf = [0u8; 8];             // create a mutable 8byte array, [value;length]
    let mut num : u64;
    let mut pfn : usize = 0;

    let mut pageflags_vec : Vec<u64> = Vec::new(); // create a u64 vector for the page flag bitmasks
    loop {
        match reader.read_exact(&mut buf) {
            Ok(_) => {
                num = u64::from_le_bytes(buf);      // converts the little endian bytes 
                                                    // to hexadecimal
                pageflags_vec.push(num);            // push to the vector
                pfn += 1;
                println!("{} : {}", pfn, pageflags_vec[pfn-1]); // print the 64bit bitmask
            },
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
    }
    for num in pageflags_vec {
    stdout().write_all(&num.to_le_bytes())?;
}
    Ok(())
}
