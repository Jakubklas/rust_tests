use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn read_file() -> io::Result<()> {
    // OPENING & READING FROM A FILE
    let path = "/Users/jakubklas/Documents/Code/test_project/dockerfile";
    let contents = fs::read_to_string(path);
    
    println!("{}", contents?);
    Ok(())
}

fn read_whole_file(path:&str) -> io::Result<String> {
    fs::read_to_string(path)
}

fn read_by_line() -> io::Result<()> {
    let path = "/Users/jakubklas/Documents/Code/test_project/dockerfile";
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if index >= 5 { break; }
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn write_to_file() -> io::Result<()> {
    let path = "/Users/jakubklas/Documents/Code/test_project/rust_text.txt";
    fs::write(path, "Hello from Rust!!!")?;
    Ok(())
}

fn write_with_control() -> io::Result<()> {
    let path = "/Users/jakubklas/Documents/Code/test_project/hello_rust.txt";
    let mut file = File::create(path)?;
    writeln!(file, "This is line 1")?;
    writeln!(file, "This is line 2")?;
    writeln!(file, "This is line 3")?;
    Ok(())
}

fn dir_preview(path: &str) -> io::Result<(i32, i32)> {
    let mut files = 0;
    let mut folders = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{:?}", path);

        if path.is_dir() {
            // continue;
            folders += 1;
            let folder = path.file_name().and_then(|n| n.to_str());
            let folder_name = folder.unwrap_or("unknown");
            println!("==> Diving into {}", folder_name);
            let _ = dir_preview(path.to_str().unwrap_or(""));
        }

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        println!("===== {} =====", filename);

        if path.is_file() {
            files += 1;
            if let Ok(file) = File::open(&path) {
                let reader = BufReader::new(file);
                for (i, line) in reader.lines().enumerate() {
                    if i >= 5 {
                        break;
                    }
                    if let Ok(content) = line {
                        println!("{}", content);
                    }
                }
            }
        }
    }

    Ok((files, folders))
}

fn read_file_by_line(path:&str) -> io::Result<String> {
    let file = fs::File::open(path.to_string())?; 
    let reader = BufReader::new(file);
    let mut result = String::new();

    for (idx, line) in reader.lines().enumerate() {
        result.push_str("\n");
        result.push_str(&line?);
        if idx >= 10 { break; }
    }
    Ok(result.to_string())
}

fn open_and_read_n(path: &str, n: usize) -> io::Result<String> {
    // Opens a FOLDER & and reads N lines from each UTF-8 file
    let mut result = String::new();
    for entry in fs::read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();

        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("unknown");
        
        if path.is_dir() { 
            println!("========> {} is a folder. Diving deeper", &name.to_string());
            if let Some(path_str) = path.to_str() {
                if let Ok(subdir_content) = open_and_read_n(path_str, n) {
                    result.push_str(&subdir_content);
                }
            }
            continue;
        }

        // If the file fails to read (e.g. it's binary), move on to the next
        let Ok(file) = fs::File::open(&path) else { continue; };
        result.push_str("\n====");
        result.push_str(name);
        result.push_str("====\n");
        let reader = BufReader::new(file);
        for (idx, line) in reader.lines().enumerate() {
            let Ok(line) = line else { continue; };
            result.push_str("\n");
            result.push_str(&line.to_string());
            if idx >= n { break; }
        }
    }
    Ok(result)
}

fn main() {
    // let path = "/Users/jakubklas/Documents/Code/test_project/dockerfile";
    let path = "/Users/jakubklas/Documents/Code/test_project/";
    
    // let result = read_whole_file(&path);
    // let result = read_file_by_line(&path);
    let result = open_and_read_n(&path, 5);

    match result {
        Ok(r) => println!("{}",r),
        Err(_) => println!("Nothing to read."),
    }
}