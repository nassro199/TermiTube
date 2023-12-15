mod video;
use video::*;

use std::env;

struct TermiTube {
    framerate: i32,
    video_path: String,
}

impl TermiTube {
    fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!(
                "No arguments found!
Example usage:
    TermiTube video.mp4 30
    TermiTube --play"
            );
            return Err("Insufficient arguments.");
        }

        let video_path = args[1].clone();
        let framerate = args.get(2).map_or(Ok(15), |s| s.parse::<i32>());

        let framerate = match framerate {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Invalid framerate. Using default: 15");
                15
            }
        };

        Ok(TermiTube { framerate, video_path })
    }

    fn run(&self) -> std::io::Result<()> {
        if self.video_path == "--play" {
            video::print_frames("/tmp/TermiTube.cache", self.framerate)?;
        } else {
            video::process_video(&self.video_path, self.framerate)?;
        }

        Ok(())
    }
}

fn main() {
    match TermiTube::new() {
        Ok(app) => {
            if let Err(err) = app.run() {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
