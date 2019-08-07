use clap::{App, Arg, SubCommand};
use std::fs::File;

fn main() {
    let matches = App::new("y4m Extract")
        .version("0.0.1")
        .author("Fredrik Pihl <pi.arctan@gmail.com>")
        .subcommand(
            SubCommand::with_name("n")
                .about("Extract single frame")
                .arg(
                    Arg::with_name("INPUT")
                        .short("i")
                        .help("y4m file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("start")
                        .short("s")
                        .help("Frame to extract")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("nn")
                .about("Extract range of frames")
                .arg(
                    Arg::with_name("INPUT")
                        .short("i")
                        .help("y4m file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("start")
                        .short("s")
                        .help("Start frame to extract")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("end")
                        .short("e")
                        .help("End frame to extract")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    // input
    let input = if let Some(matches) = matches.subcommand_matches("n") {
        matches.value_of("INPUT").unwrap()
    } else if let Some(matches) = matches.subcommand_matches("nn") {
        matches.value_of("INPUT").unwrap()
    } else {
        panic!("no subcommand");
    };

    // start
    let start: u32 = if let Some(matches) = matches.subcommand_matches("n") {
        matches.value_of("start").unwrap().parse().unwrap()
    } else if let Some(matches) = matches.subcommand_matches("nn") {
        matches.value_of("start").unwrap().parse().unwrap()
    } else {
        panic!("no subcommand");
    };

    // now we need to determine what end should be.
    let end: u32 = if let Some(_) = matches.subcommand_matches("n") {
        start
    } else if let Some(matches) = matches.subcommand_matches("nn") {
        matches.value_of("end").unwrap().parse().unwrap()
    } else {
        panic!("no subcommand");
    };

    if end < start {
        panic!("Start must be smaller than or equal to end.");
    }

    // Create output filename
    // /path/FILE.y4m -> FILE_start.y4m or
    // /path/FILE.y4m -> FILE_start-end.y4m
    // find last "." in filename, if it doesnt exist, take input.len() instead.
    let base_name = {
        // discard all input except after last slash, or start from beginning if no slash.
        let start_index = input.rfind('/').map(|i| i + 1).unwrap_or(0);
        let file_name = &input[start_index..];
        // in whats left, discard period and what's after.
        let end_index = file_name.rfind('.').unwrap_or(file_name.len());
        &file_name[0..end_index]
    };

    let outputstr = if start == end {
        format!("{}_{}.y4m", base_name, start)
    } else {
        format!("{}_{}-{}.y4m", base_name, start, end)
    };

    let mut fdin = File::open(input).unwrap();
    let mut decoder = y4m::decode(&mut fdin).unwrap();

    let mut fdout = File::create(outputstr).unwrap();
    let mut encoder = y4m::encode(
        decoder.get_width(),
        decoder.get_height(),
        decoder.get_framerate(),
    )
    .with_colorspace(decoder.get_colorspace())
    .write_header(&mut fdout)
    .unwrap();

    // loop until end inclusive
    for i in 0..=end {
        let result = decoder.read_frame().and_then(|frame| {
            if i >= start {
                encoder.write_frame(&frame)
            } else {
                Ok(())
            }
        });

        if let Err(e) = result {
            panic!(
                "Incomplete output or out of range interval, assume unrecoverable {:?}",
                e
            );
        }
    }
}
