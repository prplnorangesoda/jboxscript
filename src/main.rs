use std::{env, fs, io::Read, process::Command}; // for .read_to_end()
fn main() {
    let test_ffmpeg = Command::new("ffmpeg")
        .arg("-version")
        .status()
        .expect("should be able to run test command");

    if !test_ffmpeg.success() {
        println!("You must install ffmpeg, the CLI to convert images, to run this program.");
        println!("You can find it at https://www.ffmpeg.org/. After installation, run this program again.");
    }
    let home = std::env::var("HOME").expect("$HOME should be set");

    println!("{}", &home);
    let outdir = home.clone() + "/jboxout";
    println!("{}", &outdir);
    let dir = fs::read_dir(&outdir);

    if let Ok(_) = dir {
        fs::remove_dir_all(&outdir).ok();
    }
    fs::create_dir(&outdir).expect("should be able to create dir");

    let input = inquire::Text::new("What directory should I look in for the images?")
        .with_default(&(home + "/Downloads/jbox"))
        .prompt()
        .expect("should be able to grab a path");

    let mut _entries = std::fs::read_dir(&input).expect("path should be valid");

    let file = _entries
        .next()
        .expect("should not be empty")
        .expect("should be a valid entry");

    if !inquire::prompt_confirmation(format!(
        "Here is the full path of a file in that directory:\n{}\nDoes this look right to you?",
        file.path()
            .to_str()
            .expect("should be able to convert file path to str")
    ))
    .unwrap()
    {
        panic!("user cancelled")
    }

    let mut counter = 0;
    for file in fs::read_dir(input).unwrap() {
        counter += 1;
        let file = file.expect("file should be valid");
        let path = file.path();
        let file_name = path.to_string_lossy();
        dbg!(&file_name);
        let cmd = Command::new("ffmpeg")
            .args(["-i", &file_name])
            .args(["-vf", "scale=425:320"])
            .args(["-q:v", "15"])
            .arg(format!("{outdir}/{counter}.jpg"))
            .status()
            .expect("should be able to run command");

        if !cmd.success() {
            panic!("The FFMPEG command was unsuccessful. Try installing a newer version?")
        }
    }

    println!("\n\nDone! Your jpgs should be in {}", outdir);
}
