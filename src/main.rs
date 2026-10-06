use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::{self};

fn readfile(path: &std::path::Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    // we read in chunks of 4096 bytes
    let mut buf = [0u8; 4096];
    loop {
       let n = file.read(&mut buf)?;
       if n == 0 {
           break;
       }
       // get the first n elements of our buffer
       hash.update(&buf[..n]);
    }
    Ok(hex::encode(hash.finalize()))
}

fn main() -> io::Result<()> {
    // lets us obtain the cwd
    // TODO: generalize to other paths
    // TODO: apply n-uplets to n files
    let cwd = env::current_dir().unwrap();
    for entry in WalkDir::new(cwd) {
        // getting an entry may cause errors
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let value = readfile(path)?;
            println!("{value}");
        }
    }
    Ok(())
}
