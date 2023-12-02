use std::io::Read;
use relative_path::RelativePath;

pub mod day1;
pub mod day2;

pub fn read_to_string(file_path: &str) -> Result<String, std::io::Error>
{
    let mut data = String::new();
    let fd = open_file(file_path)?;
    let mut br = std::io::BufReader::new(fd);
    br.read_to_string(&mut data)?;
    Ok(data)
}

pub fn open_file(file_path: &str)-> Result<std::fs::File, std::io::Error>
{
    let relative_path = RelativePath::new(file_path);
    let root = std::env::current_dir()?;
    let full_path = relative_path.to_logical_path(&root);

    std::fs::File::open(full_path)
}
