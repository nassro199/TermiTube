use image::imageops::{grayscale, resize};
use image::open;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::cmp::Ordering;
use std::fs;
use std::path::PathBuf;
use std::{thread, time};
use terminal_size::{Height, Width};

pub fn process_video(file: &str, framerate: i32) -> std::io::Result<()> {
    let cache_path = "/tmp/TermiTube.cache";

    cleanup_cache_directory(cache_path)?;
    create_cache_directory(cache_path)?;

    let frames = format!("fps={}", framerate);
    let ffmpeg = Command::new("ffmpeg")
        .args(["-i", file, "-vf", &frames, &format!("{}/%d.jpg", cache_path)])
        .status()
        .expect("failed to run ffmpeg");

    if !ffmpeg.success() {
        panic!("ffmpeg failed to install");
    }

    resize_frames(cache_path)?;
    print_frames(cache_path, framerate)?;
    println!("\x1Bc");
    println!("\x1B[?251]");

    Ok(())
}

fn cleanup_cache_directory(cache_path: &str) -> std::io::Result<()> {
    fs::remove_dir_all(cache_path).map_err(|_| "Failed to remove cache directory")
}

fn create_cache_directory(cache_path: &str) -> std::io::Result<()> {
    fs::create_dir(cache_path).map_err(|_| "Failed to create cache directory")
}

fn resize_frames(cache_path: &str) -> std::io::Result<()> {
    let (Width(w), Height(h)) = match terminal_size() {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut files = Vec::new();
    fs::read_dir(cache_path)?.for_each(|x| files.push(x.unwrap().path()));

    files.par_iter().for_each(|i| {
        let img = open(i).expect("image failed to open");
        let img = resize(
            &grayscale(&img),
            w as u32,
            h as u32,
            image::imageops::FilterType::Nearest,
        );
        img.save(i).expect("failed to resize frame");
        println!("resized: {:#?}", i);
    });

    Ok(())
}

#[allow(clippy::ptr_arg)]
#[inline(always)]
fn format(l: &PathBuf, r: &PathBuf) -> Ordering {
    let left = l.file_stem().and_then(|s| s.to_str()).unwrap_or("0").parse::<usize>().unwrap_or(0);
    let right = r.file_stem().and_then(|s| s.to_str()).unwrap_or("0").parse::<usize>().unwrap_or(0);

    left.cmp(&right)
}

#[inline(always)]
pub fn print_frames(cache_path: &str, framerate: i32) -> std::io::Result<()> {
    let chars = vec!["#", "&", "@", "$", "%", "*", ".", " "];
    let mut frames = fs::read_dir(cache_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    frames.sort_by(format);

    for i in frames {
        match open(i) {
            Ok(v) => {
                let img = v.into_bytes();
                let mut frame: String = String::with_capacity(img.len());
                for i in img {
                    frame.push_str(chars[(i / 36) as usize]);
                }

                print!("{frame}");
                let delay = time::Duration::from_millis((1000 / framerate) as u64);
                thread::sleep(delay);
            }
            Err(e) => println!("{e}"),
        }
    }

    Ok(())
}
