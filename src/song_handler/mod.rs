use rodio::Sink;
mod test;

pub struct IncreaseVolumeErr;
pub struct DecreaseVolumeErr;
pub struct DecreaseSpeedErr;

pub fn increase_song_volume(sink: &Sink, step: f32) -> Result<(), IncreaseVolumeErr> {
    let current_volume = sink.volume();

    if current_volume < 1.0 {
        println!("{}", current_volume);
        sink.set_volume(f32::min(current_volume + step, 1.0));
        Ok(())
    } else {
        Err(IncreaseVolumeErr)
    }
}

pub fn decrease_song_volume(sink: &Sink, step: f32) -> Result<(), DecreaseVolumeErr> {
    let current_volume = sink.volume();

    if current_volume > step {
        sink.set_volume(current_volume - step);
        Ok(())
    } else {
        Err(DecreaseVolumeErr)
    }
}

pub fn increase_song_speed(sink: &Sink, step: f32) {
    let current_speed = sink.speed();
    sink.set_speed(current_speed + step);
}

pub fn decrease_song_speed(sink: &Sink, step: f32) -> Result<(), DecreaseSpeedErr> {
    let current_speed = sink.speed();

    if current_speed > step {
        sink.set_speed(current_speed - step);
        Ok(())
    } else {
        Err(DecreaseSpeedErr)
    }
}

pub fn toggle_play_pause(sink: &Sink) {
    if sink.is_paused() {
        sink.play();
    } else {
        sink.pause();
    }
}
