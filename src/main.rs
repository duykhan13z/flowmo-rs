extern crate termion;

use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;
use std::time::{Duration, Instant};

use exitfailure::ExitFailure;
use termion::raw::{IntoRawMode, RawTerminal};

mod key_handler;
mod notification;
mod view;

fn main() -> Result<(), ExitFailure> {
    // start key handler on another thread
    let receiver = key_handler::run();
    // start timer
    let mut stdout = stdout().into_raw_mode()?;
    let mut round: u64 = 1;
    let mut start_time = Instant::now();
    loop {
        view::release_raw_mode(&mut stdout)?;
        // work timer
        if start_timer(
            start_time.elapsed().as_secs(),
            round,
            &receiver,
            &mut stdout,
            view::flush_work_timer,
            true,
        )? {
            return Ok(());
        }

        notification::send("Taking a break \u{2615}")?;

        // Break Timer
        if start_timer(
            start_time.elapsed().as_secs() / 5,
            round,
            &receiver,
            &mut stdout,
            view::flush_break_timer,
            false,
        )? {
            return Ok(());
        }

        notification::send("It's time to start working \u{1F4AA}")?;
        
        // work interval
        view::flush_work_interval(&mut stdout)?;
        if handle_input_on_interval(&mut stdout, &receiver)? {
            return Ok(());
        }

        round += 1;
        start_time = Instant::now();
    }
}

fn start_timer(
    current_sec: u64,
    current_round: u64,
    receiver: &Receiver<key_handler::KeyAction>,
    stdout: &mut RawTerminal<Stdout>,
    flush_fn: fn(s: &mut RawTerminal<Stdout>, t: u64, c: u64) -> Result<(), failure::Error>,
    counting: bool,
) -> Result<bool, failure::Error> {
    let mut quited = false;
    let mut paused = false;
    let mut current_sec = current_sec;
    while !quited {
        match handle_input_on_timer(receiver) {
            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            key_handler::KeyAction::Pause => if counting { paused = !paused } else { },
            key_handler::KeyAction::Ok => {
                view::release_raw_mode(stdout)?;
                break;
            }
            _ => (),
        }

        if !paused {
            if counting {
                flush_fn(stdout, current_sec, current_round)?;
                current_sec += 1;
            } else {
                if current_sec == 0 {
                    break; // Tránh giảm dưới 0
                }
                flush_fn(stdout, current_sec, current_round)?;
                current_sec -= 1;
            }
        }

        spin_sleep::sleep(Duration::from_secs(1));
    }
    Ok(quited)
}

fn handle_input_on_timer(receiver: &Receiver<key_handler::KeyAction>) -> key_handler::KeyAction {
    match receiver.try_recv() {
        Ok(key_handler::KeyAction::Quit) => key_handler::KeyAction::Quit,
        Ok(key_handler::KeyAction::Pause) => key_handler::KeyAction::Pause,
        Ok(key_handler::KeyAction::Ok) => key_handler::KeyAction::Ok,
        _ => key_handler::KeyAction::None,
    }
}

fn handle_input_on_interval(
    stdout: &mut RawTerminal<Stdout>,
    receiver: &Receiver<key_handler::KeyAction>,
) -> Result<bool, failure::Error> {
    let mut quited = false;
    for received in receiver.iter() {
        match received {
            key_handler::KeyAction::Ok => {
                view::release_raw_mode(stdout)?;
                return Ok(false); 
            },

            key_handler::KeyAction::Pause => {
                view::release_raw_mode(stdout)?;
                // return Ok(false); 
            },

            key_handler::KeyAction::Quit => {
                view::release_raw_mode(stdout)?;
                quited = true;
                break;
            }
            _ => (),
        }
    }
    Ok(quited)
}
