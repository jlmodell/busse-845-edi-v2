use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(data: &str) -> Result<(), Box<dyn std::error::Error>> {    
    let mut file = File::create("output.edi")?;

    let lines: Vec<&str> = data.trim_start_matches("\"").trim_end_matches("\"").split("\\n").collect();
    let mut count: i32 = 0;

    // dbg!(&lines);

    for line in lines {        
        if regex::Regex::new(r"^N(3|4)$").unwrap().is_match(line) {
            count += 1;
            continue;
        }        
        if regex::Regex::new(r"^SE\*").unwrap().is_match(line) {
            let parts: Vec<&str> = line.split("*").collect();
            let new_count = parts[1].parse::<i32>().unwrap() - count;

            // dbg!(&parts);
            // dbg!(&new_count);

            writeln!(file, "{}*{}*{}", parts[0], new_count, parts[2])?;
            continue;
        }
        writeln!(file, "{}", line)?;
    }

    Ok(())
}