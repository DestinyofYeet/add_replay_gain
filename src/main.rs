use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::Duration;
use std::process::Command;

use futures::stream::{self, StreamExt};

use notify::{RecursiveMode, Watcher};
use notify::event::EventKind;
use notify_debouncer_full::new_debouncer;

use clap::Parser;

mod config;

enum FileType {
    Empty,
    FLAC,
    MP3
}

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Add replay gain to files", long_about = None)]
struct Args {
    #[arg(short = 'c', long = "config")]
    config_path: String,
}

#[tokio::main]
async fn main() {
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
      let app_config_clone = app_config.clone();
      match result {
        Ok(events) => {
        
          let _ = stream::iter(events).for_each_concurrent(None, | event | {
            let app_config_clone_clone = app_config_clone.clone();
            async move {
                match event.kind {
                    EventKind::Create(_) => {
                        println!("{:?}", event.paths);
                        let path = &event.paths[0];


                        if path.is_dir(){
                          println!("Found directory {:?}", path.to_str());

                          let files = std::fs::read_dir(path).unwrap();

                          for file in files {
                            match file {
                              Ok(file) => {
                                println!("Processing: {:?}", file.path().to_str());
                                handle_file(&file.path(), &app_config_clone_clone).await;
                              }
                              Err(e) => {
                                eprintln!("Error: {}", e);
                              }
                            };
                          }
                        } else {
                            handle_file(&path, &app_config_clone_clone).await;
                        }
                    }

                    _ => {}
                }
            }
          }).await;
        }
        Err(e) => eprintln!("Event error: {e:?}"),
    }
  }
}

async fn handle_file(path: &PathBuf, app_config: &config::Config){

    let path_str = path.to_str().unwrap();

     println!("A new file was created! Path: {:?}", &path_str);

    let mut successful: bool = run_audio_normalizer(path, &app_config); 

    if successful {
        return;
    }

    let interval_steps = vec![5, 10, 30, 60, 120, 300];


    for interval in interval_steps.iter() {
      println!("Trying to run the audio normalizer again in {} seconds!", interval);
      tokio::time::sleep(Duration::from_secs(*interval)).await;
      successful = run_audio_normalizer(path, &app_config);

      if successful {
        return;
      }
    }
}

fn run_audio_normalizer(path: &PathBuf, config: &config::Config) -> bool {
    // https://bbs.archlinux.org/viewtopic.php?id=125734
    
    let binary: &String;
    let flags: &String;

    let file_type: FileType;

    let path_str = path.to_str().unwrap();

    if path_str.ends_with(".flac") {
      file_type = FileType::FLAC;
    } else if path_str.ends_with(".mp3") {
      file_type = FileType::MP3;
    } else {
      file_type = FileType::Empty;
    }

    match file_type {
        FileType::FLAC => {
            binary = &config.metaflac_path;
            flags = &config.metaflac_flags;
        }
        
        FileType::MP3 => {
            binary = &config.mp3gain_path;
            flags = &config.mp3gain_flags;
        }
         
        _ => {
          println!("File is not a flac or a mp3 file, skipping!");
          return true;
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
        return false;
    }

    let output_unwrapped = output.unwrap();

    if !output_unwrapped.status.success() {
        eprintln!("Could not add replay-gain to {} because: {:?}", &path.to_str().unwrap(), String::from_utf8(output_unwrapped.stderr));
        return false;
    }

    // println!("Command stdout: {:?}\nCommand stderr: {:?}\nCommand exit code: {:?}", String::from_utf8(output_unwrapped.stdout), String::from_utf8(output_unwrapped.stderr), output_unwrapped.status.code());

    println!("Added replay-gain to {}", &path.to_str().unwrap());
    return true;
}
