use walkdir::WalkDir;
use std::process::Command;
fn main() {
    for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
        let rap = Command::new("/home/paul/rapproximate/target/release/rap").args(["-new", "-a", "1" ,"-p", "DC-CO", "-f", &entry.path().display().to_string()]).output().unwrap();
        let harper = Command::new("/home/paul/rapproximate/target/release/rap").args(["-a", "1" ,"-p", "DC-CO", "-f", &entry.path().display().to_string()]).output().unwrap();
        
        println!("{} vs {}", String::from_utf8(rap.stdout).unwrap(), String::from_utf8(harper.stdout).unwrap());
    }
}
