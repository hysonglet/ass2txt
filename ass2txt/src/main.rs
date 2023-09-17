use std::ffi::OsString;
use std::fs::{self};
use std::io::{BufReader, BufWriter, Write};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 1 {
        return;
    }
    let dir_name = args.get(1).unwrap();

    let dir = fs::read_dir(dir_name).unwrap_or_else(|e| panic!("{}", e.to_string()));

    let newfile:String = args.get(2).unwrap().to_owned() + ".txt";
    let newfile = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(true)
        .create(true)
        .open(newfile)
        .unwrap();
    let mut writer = BufWriter::new(newfile);

    let mut sort: Vec<OsString> = Vec::new();
    for file in dir {
        let file = file.unwrap();
        let file_type = file.file_type().unwrap();
        if !file_type.is_file() {
            continue;
        }

        let file = file.file_name();
        if !file.clone().to_str().unwrap().ends_with(".ass") {
            continue;
        }

        sort.push(file);
    }

    sort.sort();

    for file in sort {
        let title = file.to_str().unwrap();
        let _ = writer.write(format!("\r\n{}\r\n", title).as_bytes());

        // println!("{:?}", &file);
        let mut full_path: String;
        full_path = dir_name.clone().to_string();
        full_path.push_str("/");
        full_path.push_str(file.clone().to_str().unwrap());

        let f = fs::File::open(full_path).unwrap_or_else(|e| panic!("{}", e.to_string()));

        let reader = BufReader::new(f);
        let reader = utf16_reader::read_to_string(reader);

        for line in reader.lines() {
            if !line.contains("Dialogue") {
                continue;
            }

            // println!("{}", &line);
            let split: Vec<&str> = line.split(",").collect();

            // let sub = split.get(9).clone().unwrap();
            let mut sub: String = String::new();
            for i in 9..split.len() {
                sub.push_str(split[i]);
            }

            let mut sub = sub.to_string();

            if sub.starts_with(r"{") || sub.starts_with(r"[") {
                continue;
            }

            sub = sub.replace("\\N", "\r\n");

            if let Some(x) = sub.find('{') {
                while sub.contains("}") {
                    sub.remove(x);
                }
            }

            sub.push_str("\r\n");
            // sub = sub.replace(r"{*}", "");
            let _ = writer.write_all(sub.as_bytes());

            println!("{}", &sub);
        }
    }
}
