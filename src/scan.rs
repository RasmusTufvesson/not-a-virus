use std::{fs::{self, read_to_string}, path::PathBuf};

pub struct Line {
    pub file: String,
    pub contents: String,
    pub keyword: String,
    pub filetype: String,
}

pub struct Scan {
    pub lines: Vec<Line>,
}

pub fn scan_drive(drive: &str, extensions: Vec<&str>, keywords: Vec<&str>) -> Scan {
    let mut scan = Scan { lines: vec![] };
    scan_dir(&mut scan, drive, &extensions, &keywords).unwrap();
    scan
}

fn scan_dir(scan: &mut Scan, dir: &str, extensions: &Vec<&str>, keywords: &Vec<&str>) -> std::io::Result<()> {
    let paths = fs::read_dir(dir)?;
    for path in paths {
        match path {
            Ok(path) => {
                match path.file_type() {
                    Ok(kind) => {
                        if kind.is_dir() {
                            let _ = scan_dir(scan, path.path().to_str().unwrap(), extensions, keywords);
                        } else if kind.is_file() {
                            let path = path.path();
                            match path.extension() {
                                None => {}
                                Some(name) => {
                                    let name = name.to_str().unwrap();
                                    if extensions.contains(&name) {
                                        let name = name.to_string();
                                        scan_file(scan, path, keywords, name);
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
    Ok(())
}

fn scan_file(scan: &mut Scan, path: PathBuf, keywords: &Vec<&str>, filetype: String) {
    let metadata = path.metadata().unwrap();
    if metadata.len() < 50_000 {
        let filename = path.to_str().unwrap().to_string();
        match read_to_string(path) {
            Ok(file) => {
                for line in file.split("\n") {
                    for keyword in keywords {
                        if line.contains(keyword) {
                            scan.lines.push(Line { file: filename.clone(), contents: line.to_string(), keyword: keyword.to_string(), filetype: filetype.clone() });
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
}