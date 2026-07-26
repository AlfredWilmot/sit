use clap::builder::NonEmptyStringValueParser;
use clap::{Command, arg, value_parser};
use rodio::source::SineWave;
use rodio::{MixerDeviceSink, Source};
use std::thread;
use std::time::{Duration, Instant};

const ANSI_CLEAR: &str = "\x1B[2J\x1B[H";

fn main() {
    match app() {
        Ok(()) => {}
        Err(err) => eprintln!("{err}"),
    }
}

fn bell(device_sink: &MixerDeviceSink, tone: f32) {
    let mixer = device_sink.mixer();
    let wave = SineWave::new(tone)
        .amplify(1.5)
        .take_duration(Duration::from_secs_f32(1.5));
    mixer.add(wave);
    thread::sleep(Duration::from_secs_f32(1.5));
}

fn interval_str_into_seconds(interval: String) -> Result<u64, String> {
    // scan while there are valid digits until we reach end of string or duration char [sSmMhH]

    let mut buff = Vec::<u8>::new();
    let mut mul = 1;

    for char in interval.as_bytes() {
        match char {
            b'0'..=b'9' => buff.push(*char),
            b's' => {
                mul = 1;
                break;
            }
            b'm' => {
                mul = 60;
                break;
            }
            b'h' => {
                mul = 60 * 60;
                break;
            }
            _ => return Err("detected invalid interval char".to_string()),
        }
    }
    let result: String = String::from_utf8(buff).map_err(|e| e.to_string())?;
    Ok(result.parse::<u64>().map_err(|e| e.to_string())? * mul)
}

fn app() -> Result<(), String> {
    let matches = Command::new("sit")
        .about("Simple Interval Timer")
        .arg_required_else_help(true)
        .arg(
            arg!(-r - -rounds[ROUNDS])
                .num_args(1)
                .value_parser(value_parser!(u64).range(1..))
                .default_value("1"),
        )
        .arg(
            arg!(intervals: [INTERVAL])
                .num_args(1..)
                .value_parser(NonEmptyStringValueParser::new())
                .next_line_help(true),
        )
        .get_matches();

    let mut device_sink =
        rodio::DeviceSinkBuilder::open_default_sink().map_err(|e| e.to_string())?;
    device_sink.log_on_drop(false);

    let _intervals: Result<Vec<u64>, String> = matches
        .get_many::<String>("intervals")
        .map(|vals| vals.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter_mut()
        .map(|x| interval_str_into_seconds(x.to_ascii_lowercase()))
        .collect();

    let intervals = _intervals?;

    let rounds: u64 = *matches
        .get_one("rounds")
        .expect("number of rounds should be specified");

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
