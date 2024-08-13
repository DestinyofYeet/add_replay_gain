use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;
use std::process::Command;

use notify::{RecursiveMode, Watcher};
use notify::event::EventKind;
use notify_debouncer_full::new_debouncer;

use clap::Parser;

mod config;

enum FileType {
    FLAC,
    MP3
}

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Add replay gain to files", long_about = None)]
struct Args {
    #[arg(short = 'c', long = "config")]
    config_path: String,
}

fn main() {
    let args = Args::parse();
    let app_config = config::Config::parse(&args.config_path);

    if app_config.is_none() {
        eprintln!("Failed to parse config! Exiting!");
        exit(1);
    }

    let app_config = app_config.unwrap();

    println!("Watch path is: {}", app_config.watch_path);

    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&app_config.watch_path), RecursiveMode::Recursive)
        .unwrap();

    for result in rx {
        match result {
            Ok(events) => {
                events
                    .iter()
                    .for_each(|event| match event.kind {
                        EventKind::Create(_) => {
                            let path = &event.paths[0];
                            
                            let path_str = path.to_str().unwrap();
                            
                             println!("A new file was created! Path: {:?}", &path_str);
                            
                            if path_str.ends_with(".mp3"){
                                run_audio_normalizer(path, &app_config, FileType::MP3);
                                
                            } else if path_str.ends_with(".flac"){
                                run_audio_normalizer(path, &app_config, FileType::FLAC);
                                
                            } else {
                                println!("Skipping file '{}'! Not a .flac or .mp3!", &path_str);
                            }
                        },
                        
                        _ => {
                            
                        }
                    })
            }

            Err(e) => eprintln!("Event error: {e:?}"),
        }
    }
}

fn run_audio_normalizer(path: &PathBuf, config: &config::Config, file_type: FileType){
    // https://bbs.archlinux.org/viewtopic.php?id=125734
    
    let binary: &String;
    let flags: &String;
    
    match file_type {
        FileType::FLAC => {
            binary = &config.metaflac_path;
            flags = &config.metaflac_flags;
        }
        
        FileType::MP3 => {
            binary = &config.mp3gain_path;
            flags = &config.mp3gain_flags;
        }
    }
    
    println!("Running: {} {} {:?}", binary, flags, &path);
    let mut command = Command::new(binary);
    
    let split = flags.split(" ");
            
    for value in split.into_iter(){
        command.arg(value);
    }
    
    command.arg(path);
    
    let output = command.output();

    if output.is_err() {
        eprintln!("Failed to run {:?}!", command.get_program());
        return;
    }

    let output_unwrapped = output.unwrap();

    if !output_unwrapped.status.success() {
        eprintln!("Could not add replay-gain to {} because: {:?}", &path.to_str().unwrap(), String::from_utf8(output_unwrapped.stderr));
        return;
    }

    // println!("Command stdout: {:?}\nCommand stderr: {:?}\nCommand exit code: {:?}", String::from_utf8(output_unwrapped.stdout), String::from_utf8(output_unwrapped.stderr), output_unwrapped.status.code());

    println!("Added replay-gain to {}", &path.to_str().unwrap());
}
