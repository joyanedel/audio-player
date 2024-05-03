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
            Key::Char(' ') => {
                if sink.is_paused() {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
            Key::Right => increase_song_speed(&sink, 0.02),
            Key::Left => decrease_song_speed(&sink, 0.02),
            Key::Up => increase_song_volume(&sink, 0.02),
            Key::Down => decrease_song_volume(&sink, 0.02),
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
    write!(stdout, "Current song: {}", "Bat Country").unwrap();
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

fn increase_song_volume(sink: &Sink, step: f32) {
    let current_volume = sink.volume();

    if current_volume < 1.0 {
        println!("{}", current_volume);
        sink.set_volume(f32::min(current_volume + step, 1.0));
    }
}

fn decrease_song_volume(sink: &Sink, step: f32) {
    let current_volume = sink.volume();

    if current_volume > 0.0 {
        sink.set_volume(f32::max(current_volume - step, 0.0));
    }
}

fn increase_song_speed(sink: &Sink, step: f32) {
    let current_speed = sink.speed();
    sink.set_speed(current_speed + step);
}

fn decrease_song_speed(sink: &Sink, step: f32) {
    let current_speed = sink.speed();

    if current_speed > 0.0 {
        sink.set_speed(f32::max(current_speed - step, 0.001));
    }
}
