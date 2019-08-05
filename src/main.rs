extern crate y4m;
extern crate clap;

use std::fs::File;
use std::io;
use std::path::Path;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("y4m Extract")
        .version("0.0.1")
        .author("Fredrik Pihl <pi.arctan@gmail.com>")
        .subcommand(SubCommand::with_name("n")
                    .about("Extract single frame")
                    .arg(Arg::with_name("INPUT")
                         .short("i")
                         .help("y4m file")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("start")
                         .short("s")
                         .help("Frame to extract")
                         .takes_value(true)
                         .required(true)))
        .subcommand(SubCommand::with_name("nn")
                    .about("Extract range of frames")
                    .arg(Arg::with_name("INPUT")
                         .short("i")
                         .help("y4m file")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("start")
                         .short("s")
                         .help("Start frame to extract")
                         .takes_value(true)
                         .required(true))
                    .arg(Arg::with_name("end")
                         .short("e")
                         .help("End frame to extract")
                         .takes_value(true)
                         .required(true)))
        .get_matches();

    let mut start:u32 = 0;
    let mut end:u32 = 0;
    let mut input = String::new();

    if let Some(matches) = matches.subcommand_matches("n") {
        if let Some(string) = matches.value_of("INPUT") {
            input = string.parse().unwrap();
        }
        if let Some(string) = matches.value_of("start") {
            start = string.parse().unwrap();
        }
        end = start;
    }
    if let Some(matches) = matches.subcommand_matches("nn") {
        if let Some(string) = matches.value_of("INPUT") {
            input = string.parse().unwrap();
        }
        if let Some(string) = matches.value_of("start") {
           start = string.parse().unwrap();
        }
        if let Some(string) = matches.value_of("end") {
            end = string.parse().unwrap();
        }
    }

    if end < start {
        eprintln!( "Start must be smaller than or equal to end.");
        return
    }

    // Create output filename
    // /path/FILE.y4m -> FILE_start.y4m or
    // /path/FILE.y4m -> FILE_start-end.y4m
    let path = Path::new(&input);
    let output = path.file_stem().unwrap();
    let tmp = match output.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => s,
    };
    let outputstr;
    if start == end {
        outputstr = format!("{}_{}.y4m", tmp, start);
    } else {
        outputstr = format!("{}_{}-{}.y4m", tmp, start, end);
    }


    let mut fdin: Box<io::Read> = Box::new(File::open(input).unwrap());
    let mut decoder = y4m::decode(&mut fdin).unwrap();

    let mut fdout: Box<io::Write> = Box::new(File::create(outputstr).unwrap());
    let mut encoder = y4m::encode(decoder.get_width(), decoder.get_height(), decoder.get_framerate())
        .with_colorspace(decoder.get_colorspace())
        .write_header(&mut fdout)
        .unwrap();

    let mut i = 0;
    loop {
        match decoder.read_frame() {
            Ok(frame) => {
                if i >= start && i <= end && encoder.write_frame(&frame).is_err() { break }
                if i > end {
                    break
                }
           },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            },
        }
        i += 1;
    }
}
