// Rust-101, Part 13: Concurrency, Arc, Send
// =========================================

use std::io::prelude::*;
use std::ops::Deref;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::{fs, io, thread};

// Before we come to the actual code, we define a data-structure `Options` to store all the
// information we need to complete the job: Which files to work on, which pattern to look for, and
// how to output.
#[derive(Clone, Copy)]
pub enum OutputMode {
    Print,
    SortAndPrint,
    Count,
}
use self::OutputMode::*;

pub struct Options {
    pub files: Vec<Arc<String>>,
    pub pattern: String,
    pub output_mode: OutputMode,
}

struct MatchedLine {
    file_name: Arc<String>,
    number: usize,
    line: String,
}

// The first function reads the files, and sends every line over the `out_channel`.
fn read_files(options: Arc<Options>, out_channel: SyncSender<MatchedLine>) {
    for file_name in options.files.iter() {
        // First, we open the file, ignoring any errors.
        let file = fs::File::open(file_name.deref()).unwrap();
        // Then we obtain a `BufReader` for it, which provides the `lines` function.
        let file = io::BufReader::new(file);

        for (line_number, line) in file.lines().enumerate() {
            let line = line.unwrap();
            let matched_line = MatchedLine {
                file_name: file_name.clone(),
                number: line_number,
                line,
            };

            // Now we send the line over the channel, ignoring the possibility of `send` failing.
            out_channel.send(matched_line).unwrap();
        }
    }
    // When we drop the `out_channel`, it will be closed, which the other end can notice.
}

// The second function filters the lines it receives through `in_channel` with the pattern, and sends
// matches via `out_channel`.
fn filter_lines<'a>(
    options: Arc<Options>,
    in_channel: Receiver<MatchedLine>,
    out_channel: SyncSender<MatchedLine>,
) {
    // We can simply iterate over the channel, which will stop when the channel is closed.
    for matched_line in in_channel.iter() {
        // `contains` works on lots of types of patterns, but in particular, we can use it to test
        // whether one string is contained in another. This is another example of Rust using traits
        // as substitute for overloading.
        if matched_line.line.contains(&options.pattern) {
            out_channel.send(matched_line).unwrap();
        }
    }
}

// The third function performs the output operations, receiving the relevant lines on its
// `in_channel`.
fn output_lines(options: Arc<Options>, in_channel: Receiver<MatchedLine>) {
    match options.output_mode {
        Print => {
            // Here, we just print every line we see.
            for matched_line in in_channel.iter() {
                let MatchedLine {
                    file_name,
                    number,
                    line,
                } = matched_line;

                println!("{file_name}:{number}: {line}");
            }
        }
        Count => {
            // We are supposed to count the number of matching lines. There's a convenient iterator
            // adapter that we can use for this job.
            let count = in_channel.iter().count();

            println!("{} hits for {}.", count, options.pattern);
        }
        SortAndPrint => {
            // We are asked to sort the matching lines before printing. So let's collect them all
            // in a local vector...
            let mut data: Vec<_> = in_channel.iter().collect();
            // ...and implement the actual sorting later.
            unimplemented!()
        }
    }
}

// With the operations of the three threads defined, we can now implement a function that performs
// grepping according to some given options.
pub fn run(options: Options) {
    // We move the `options` into an `Arc`, as that's what the thread workers expect.
    let options = Arc::new(options);

    // This sets up the channels. We use a `sync_channel` with buffer-size of 16 to avoid needlessly
    // filling RAM.
    let (line_sender, line_receiver) = sync_channel(16);
    let (filtered_sender, filtered_receiver) = sync_channel(16);

    // Spawn the read thread: `thread::spawn` takes a closure that is run in a new thread.
    let options1 = options.clone();
    let handle1 = thread::spawn(move || read_files(options1, line_sender));

    // Same with the filter thread.
    let options2 = options.clone();
    let handle2 = thread::spawn(move || filter_lines(options2, line_receiver, filtered_sender));

    // And the output thread.
    let options3 = options.clone();
    let handle3 = thread::spawn(move || output_lines(options3, filtered_receiver));

    // Finally, wait until all three threads did their job.
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

// Now we have all the pieces together for testing our rgrep with some hard-coded options.
pub fn main() {
    let options = Options {
        files: vec![
            "src/part10.rs".to_string().into(),
            "src/part11.rs".to_string().into(),
            "src/part12.rs".to_string().into(),
        ],
        pattern: "let".to_string(),
        output_mode: Print,
    };
    run(options);
}

// **Exercise 13.1**: Change rgrep such that it prints not only the matching lines, but also the
// name of the file and the number of the line in the file. You will have to change the type of the
// channels from `String` to something that records this extra information.
// NOTE(DP): Done
