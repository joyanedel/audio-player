mod song_handler;

use song_handler::*;

use rodio::Sink;
use rodio::{Decoder, OutputStream};
use std::fs::File;
use std::io::{stdin, stdout, BufReader, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = load_song("src/bat_country.mp3");
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    show_current_song_information(&mut stdout, &sink);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,
            Key::Char(' ') => toggle_play_pause(&sink),
            Key::Right => increase_song_speed(&sink, 0.02),
            Key::Left => decrease_song_speed(&sink, 0.02).unwrap_or_else(|_| {
                sink.set_speed(0.01);
            }),
            Key::Up => increase_song_volume(&sink, 0.02).unwrap_or_else(|_| {
                sink.set_volume(1.0);
            }),
            Key::Down => decrease_song_volume(&sink, 0.02).unwrap_or_else(|_| {
                sink.set_volume(0.00);
            }),
            _ => (),
        }

        show_current_song_information(&mut stdout, &sink);
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn load_song(filename: &str) -> BufReader<File> {
    BufReader::new(File::open(filename).unwrap())
}

fn show_current_song_information(stdout: &mut RawTerminal<Stdout>, sink: &Sink) {
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();

    // write current song
    let song_name = "Bat Country";
    write!(stdout, "Current song: {}", song_name).unwrap();
    if sink.is_paused() {
        write!(stdout, " [PAUSED]").unwrap();
    }
    writeln!(stdout).unwrap();

    // write volume
    write!(
        stdout,
        "{}{:<10} {:>3.0}%",
        termion::cursor::Goto(1, 2),
        "Volume:",
        sink.volume() * 100.0
    )
    .unwrap();
    let current_volume_blocks = (sink.volume() / 0.05).round() as usize;
    writeln!(stdout, "  {}", "#".repeat(current_volume_blocks)).unwrap();

    // write speed
    writeln!(
        stdout,
        "{}{:<10} {:>3.0}%",
        termion::cursor::Goto(1, 3),
        "Speed:",
        sink.speed() * 100.0
    )
    .unwrap();
    stdout.flush().unwrap();
}
