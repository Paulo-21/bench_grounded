use walkdir::WalkDir;
use std::{process::Command, path::Path, fs::File};
use std::io::{BufRead, self};
fn main() {
    let mut good = 0;
    let mut wrong = 0;
    for entry in WalkDir::new("C:/Users/paulc/Downloads/iccma2023_benchmarks/benchmarks_clean/").into_iter().filter_map(|e| e.ok()) {
        let r = entry.path().display().to_string();
        if r.ends_with(".af") {
            //for i in 1..get_nb_arg(entry.path().display().to_string()) {
                //println!(" {i} ; {}", entry.path().display());
                let rap = Command::new("C:/Users/paulc/Desktop/code/rapproximate_hcat/target/release/rapproximate.exe").args(["--heuristic", "harper", "-n", "-a", "1" ,"-p", "DC-CO", "-f", &entry.path().display().to_string()]).output().unwrap();
                let harper = Command::new("C:/Users/paulc/Desktop/code/rapproximate_hcat/target/release/rapproximate.exe").args(["--heuristic", "harper","-a", "1" ,"-p", "DC-CO", "-f", &entry.path().display().to_string()]).output().unwrap();
                let rap_res = String::from_utf8(rap.stdout).unwrap();
                let harper_res = String::from_utf8(harper.stdout).unwrap();
                //println!("{} vs {}", rap_res.trim(), harper_res.trim());
                if rap_res.trim() == harper_res.trim() {
                    good += 1;
                }
                else {
                    wrong += 1;
                }
                println!(" good : {good}, wrong : {wrong} ");
            //}
        }
        
    }
}


fn get_nb_arg(path : String) -> i32 {
    let mut a = read_lines(path).unwrap();
    let lin =  a.next().unwrap().unwrap();
    let iter: Vec<&str> = lin.split_ascii_whitespace().collect();
    let nb_arg = iter[2].parse::<i32>().unwrap();
    nb_arg
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}