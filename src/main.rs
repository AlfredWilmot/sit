use clap::Parser;
use rodio::source::SineWave;
use rodio::{MixerDeviceSink, Source};
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

// TODO
// - [ ] cli flags and parse human-readable duration format:
//          e.g. --intervals 1m 30s --lim 1h
//          or --spar 5m --rest 2m --rounds 5
// - [ ] print single formatted output string
// - [ ] TUI showing grid of digital timers

const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";

/// A simple time-keeping CLI app
#[derive(Parser)]
#[command(
    version,
    after_help = "Examples:
    cli-timer --intervals 10 20 30 --rounds 3
    cli-timer --intervals 60 90
    "
)]
struct Args {
    /// List of intervals for every round
    #[arg(long, required=true, num_args=1..)]
    intervals: Vec<String>,
    /// Total number of rounds (default=1)
    #[arg(long, required = false, num_args = 1)]
    rounds: Option<usize>,
}

fn bell(device_sink: &MixerDeviceSink, tone: f32) {
    let mixer = device_sink.mixer();
    let wave = SineWave::new(tone)
        .amplify(1.5)
        .take_duration(Duration::from_secs_f32(1.5));
    mixer.add(wave);
    thread::sleep(Duration::from_secs_f32(1.5));
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut device_sink = rodio::DeviceSinkBuilder::open_default_sink()?;

    let intervals: Vec<u64> = args
        .intervals
        .iter()
        .filter_map(|s| Some(s.parse::<u64>().ok())?)
        .rev()
        .collect();

    // ensure rounds is greater than or equal to 1
    let rounds = args.rounds.filter(|&x| x >= 1).unwrap_or(1);

    let start_time = Instant::now();
    let mut last_time = start_time;

    let mut current_time: Instant;
    let mut total_time: u64 = 0;
    let mut round_time: u64;

    for round in 1..=rounds {
        for (idx, current_interval) in intervals.iter().enumerate() {
            let _idx = idx + 1;
            println!("{ANSI_CLEAR} Next Interval: #{_idx} in {intervals:?}");
            bell(&device_sink, 960.0);

            loop {
                current_time = Instant::now();
                total_time = current_time.duration_since(start_time).as_secs();
                round_time = current_time.duration_since(last_time).as_secs();

                // keep track of which round we're on
                if round_time >= *current_interval {
                    last_time = current_time;
                    break; // exit current interval

                // output some words of encouragement
                } else {
                    println!("{ANSI_CLEAR}Keep Going!");
                    println!("Round {round} out of {rounds}");
                    println!("Interval #{_idx} in {intervals:?}");
                    println!("Interval Time: {round_time}s out of {current_interval}s");
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    println!("Well Done! (Total Time: {total_time}s)");
    bell(&device_sink, 480.0);

    device_sink.log_on_drop(false);

    Ok(())
}
