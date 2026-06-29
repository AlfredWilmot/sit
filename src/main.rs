use clap::Parser;
use rodio::Source;
use rodio::source::SineWave;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

// TODO
// - [ ] cli flags: --interval 1m,30s
// - [ ] print single formatted output string
// - [ ] and...

const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long, required=true, num_args=1..)]
    rounds: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let rounds: Vec<u64> = args
        .rounds
        .iter()
        .filter_map(|s| Some(s.parse::<u64>().ok())?)
        .rev()
        .collect();

    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mixer = stream_handle.mixer();

    let start_time = Instant::now();
    let mut last_time = Instant::now();
    let mut current_time = Instant::now();
    let mut total_time: u64 = 0;
    let mut round_time: u64 = rounds[0];

    let mut idx: usize = 0;

    loop {
        // keep track of which round we're on
        let interval = rounds[idx];
        if round_time >= interval {
            idx += 1;
            if idx >= rounds.len() {
                idx = 0;
            }
            // Generate bell
            println!("{ANSI_CLEAR}Round start!");
            last_time = current_time; // reset the interval tracker
            let wave = SineWave::new(960.0)
                .amplify(1.5)
                .take_duration(Duration::from_secs_f32(1.5));
            mixer.add(wave);
            thread::sleep(Duration::from_secs_f32(1.5));
        } else {
            // output some words of encouragement
            println!("{ANSI_CLEAR}Keep Going! (interval: {interval}s)");
            println!("Total time: {total_time}s");
            println!("Round time: {round_time}s");
            thread::sleep(Duration::from_millis(100));
        }
        current_time = Instant::now();
        total_time = current_time.duration_since(start_time).as_secs();
        round_time = current_time.duration_since(last_time).as_secs();
    }
}
