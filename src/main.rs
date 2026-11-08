use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::env;
use std::io::Read;
use std::fs::File;
use std::io::{self};
use std::collections::HashMap;
use std::path::PathBuf;

fn read_hash(path: &std::path::Path) -> io::Result<String> {
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
    let cwd = env::current_dir().unwrap();

    let mut files: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(cwd) {
        // getting an entry may cause errors
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let value = read_hash(path)?;
            if files.contains_key(&value) {
                if let Some(val) = files.get_mut(&value) { val.push(PathBuf::from(path)); };
            }
            else {
                paths.push(PathBuf::from(path));
                files.insert(value, paths);
                paths = Vec::new();
            }
        }
    }
    let mut rm = Vec::new();
    for (key, values) in &files {
        if values.len() > 1 {
            println!("{:?} => {:?}", key, values);
        }else {
            rm.push(key.clone());
        }
    }
    for key in rm {
        files.remove(&key);
    }
    println!("\nFound {} distinct files with at least one duplicate.", files.len());
    

    Ok(())
}
