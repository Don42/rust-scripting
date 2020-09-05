use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use std::process::Command;

fn main() -> io::Result<()> {
    let zip_files: Vec<DirEntry> = fs::read_dir(".")?
        .map(|x| x.unwrap())
        .filter(|x| x.file_name().to_string_lossy().ends_with(".zip"))
        .collect();
    for item in zip_files {
        let file_name = item.file_name();
        let parts: Vec<&str> = file_name
            .to_str()
            .ok_or(io::Error::new(io::ErrorKind::Other, "missing"))?
            .strip_suffix(".zip")
            .ok_or(io::Error::new(io::ErrorKind::Other, "missing"))?
            .splitn(2, "-")
            .collect();
        //assert_eq!(parts.len(), 2);

        let band_name = parts[0].trim();
        let album_name = parts[1].trim();

        let extract_path = format!("{}/{}", band_name, album_name);
        println!("{}", &extract_path);
        fs::create_dir_all(&extract_path)?;

        let status = Command::new("unzip")
            .arg("-d")
            .arg(&extract_path)
            .arg(file_name)
            .status()?;
        if !status.success() {
            return Err(match status.code() {
                Some(code) => io::Error::from_raw_os_error(code),
                None => io::Error::new(io::ErrorKind::Other, "killed by signal"),
            });
        }
    }
    Ok(())
}
