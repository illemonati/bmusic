extern crate rodio;

use std::io::BufReader;
use std::env::{args, current_exe};
use std::process::Command;

fn main() {
    let mut is_sub = false;
    for arg in args(){
        if arg == "--is_sub"{
            is_sub = true;
        }
    }
    if is_sub{
        play();
    }else{
        spawn_subprocess();
    }
}


fn spawn_subprocess(){
    match Command::new(format!("{}", current_exe().unwrap().to_string_lossy()))
            .args(&["--is_sub"])
            .spawn(){
                Ok(_) => {},
                Err(e) => {eprintln!("{:?}", e); return;}
            };
}

fn play(){
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    loop{
        let file = std::fs::File::open("nyan_cat.mp3").unwrap();
        let decoded_file = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(decoded_file);
        sink.sleep_until_end();
    }
}
