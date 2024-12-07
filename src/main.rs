use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{path::Path, sync::mpsc};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("/home/syscall/logbox"), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                // println!("{:?}", event);
                logParser(event.paths, event.kind)?;
            }
            Err(e) => {
                println!("watch err: {:?}", e);
            }
        }
    }

    Ok(())
}

fn logParser(filePath: Vec<PathBuf>, eventKind: EventKind) -> Result<()> {
    if eventKind.is_create() || eventKind.is_modify() || eventKind.is_access() {
        for f in filePath {
            let f_str = f.to_str();
            let mut file = File::open(f)?;
            let mut contents = String::new();

            file.read_to_string(&mut contents)?;

            println!("File contents: {}", contents);
        }
    } else {
        // Handle other event kinds if necessary
        println!("Unhandled event kind: {:?}", eventKind);
    }

    Ok(())
}
