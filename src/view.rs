use std::io::Write;

use failure::ResultExt;
use termion::{clear, color};

pub fn flush_work_timer(
    stdout: &mut impl Write,
    current_sec: u64,
    current_round: u64,
) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{1F345} {timer} (Round {current_round}){desc_cursor}",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Red),
        clear = clear::AfterCursor,
        timer = convert_to_min(current_sec),
        current_round = current_round,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show work timer")?;
    stdout.flush().context("failed to flush work timer")?;
    Ok(())
}

pub fn flush_break_timer(
    stdout: &mut impl Write,
    current_sec: u64,
    current_round: u64,
) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{timer_cursor}{color}{clear}\u{2615} {timer} (Round {current_round}){desc_cursor}",
        timer_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Green),
        clear = clear::AfterCursor,
        timer = convert_to_min(current_sec),
        current_round = current_round,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show break timer")?;
    stdout.flush().context("failed to flush break timer")?;
    Ok(())
}


pub fn flush_work_interval(stdout: &mut impl Write) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{msg_cursor}{color}{clear}\u{1F514} Press [Enter] to start working{desc_cursor}",
        msg_cursor = termion::cursor::Goto(2, 1),
        color = color::Fg(color::Red),
        clear = clear::AfterCursor,
        desc_cursor = termion::cursor::Goto(2, 2)
    ).context("failed to show work interval")?;
    stdout.flush().context("failed to flush work interval")?;
    Ok(())
}

pub fn release_raw_mode(stdout: &mut impl Write) -> Result<(), failure::Error> {
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(1, 1),
        termion::cursor::Show,
        clear::AfterCursor
    )
    .context("failed to release raw mode")?;
    Ok(())
}

fn convert_to_min(duration: u64) -> String {
    let min = duration / 60;
    let sec = duration % 60;
    format!("{:02}:{:02}", min, sec)
}

#[cfg(test)]
mod tests {
    use crate::view::*;

    #[test]
    fn flush_work_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_timer(&mut buf, 4, 1);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("00:04 (Round 1)"));
    }

    #[test]
    fn flush_break_timer_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_break_timer(&mut buf, 604, 2);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("10:04 (Round 2)"));
    }


    #[test]
    fn flush_work_interval_works_fine() {
        let mut buf = Vec::<u8>::new();
        let actual_resp = flush_work_interval(&mut buf);
        let actual_view = String::from_utf8(buf.to_vec()).unwrap();

        assert!(actual_resp.is_ok());
        assert!(actual_view.contains("press Enter to start working"));
    }
}
