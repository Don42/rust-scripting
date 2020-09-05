use std::fs::{self, DirEntry};
use std::io;
use std::path::{PathBuf, Path};
use std::process::Command;

fn convert_file(input: &Path, output: &Path) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&input)
        .args(&["-ab", "320k"])
        //.args(&["-map_metadata", "0", "-id3v2_version", "3"])
        // Not necessary since ffmpeg 3.2
        .arg(&output)
        .status()?;
    if !status.success() {
        return Err(match status.code() {
            Some(code) => io::Error::from_raw_os_error(code),
            None => io::Error::new(io::ErrorKind::Other, "killed by signal"),
        });
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let input_files: Vec<PathBuf> = std::env::args()
        .skip(1)  // Skip the command name
        .map(|x| PathBuf::from(&x).canonicalize().unwrap())
        .filter(|p| p.exists() && p.is_file())
        .collect();

    for input_path in input_files {
        let current_dir = std::env::current_dir()?;
        println!("'{:?}'", input_path);

        // Find the music folder
        let music_dir_name= std::ffi::OsStr::new("music");
        let music_dir = input_path
            .ancestors()
            .find(|p| p.file_name() == Some(&music_dir_name));

        if music_dir.is_none() {
            return Ok(());
        }

        // Build the output path
        // The suffix contains the artist and album names
        let input_path_suffix = input_path.strip_prefix(music_dir.unwrap()).unwrap();
        println!("'{:?}'", input_path_suffix);

        let mut output_path = current_dir.to_path_buf();
        output_path.push("mp3_export");  // Append export folder
        output_path.push(input_path_suffix);  // Append artist/album folder
        output_path.set_extension("mp3");  // Append file extension

        // Create folder if it doesn't exist
        fs::create_dir_all(output_path.parent().unwrap())?;

        convert_file(&input_path, &output_path)?;
    }
    Ok(())
}
