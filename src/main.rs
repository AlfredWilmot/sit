use clap::{CommandFactory, Parser};
use rodio::source::SineWave;
use rodio::{MixerDeviceSink, Source};
use std::thread;
use std::time::{Duration, Instant};

const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";

fn main() {
    match cli() {
        Ok(()) => {}
        Err(err) => eprintln!("{err}")
    }
}

/// A simple time-keeping CLI app
#[derive(Parser)]
#[command(
    version,
    after_help = "Examples:
    sit -r 5 30s 1m ...
    sit 0.5h 15m
    "
)]
struct Args {
    /// Timer intervals
    intervals: Vec<String>,
    /// Total number of rounds (default=1)
    #[arg(short, long, required = false, num_args = 1)]
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

fn cli() -> Result<(), String> {
    let args = Args::parse();

    let mut device_sink = rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| e.to_string())?;
    device_sink.log_on_drop(false);

    let intervals: Vec<u64> = args
        .intervals
        .iter()
        .filter_map(|s| Some(s.parse::<u64>().ok())?)
        .collect();

    if intervals.is_empty() {
        return Err(format!("{}", Args::command().error(clap::error::ErrorKind::TooFewValues, "Must specify at least one interval!")));
    }

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
