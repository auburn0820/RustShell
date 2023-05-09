use std::env::current_dir;
use std::fs::read_dir;

fn main() -> std::io::Result<()> {
    let current_dir = current_dir()?;
    let entries = read_dir(current_dir)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        print!("{}\t\t", file_name.to_string_lossy());
    }
    println!();

    Ok(())
}