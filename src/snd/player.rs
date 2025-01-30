use rodio::source::chirp;
use std::time::Duration;
use rodio::{OutputStream, Sink, Source};

pub fn play_welcome_chirp() {
    tokio::spawn(async move {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = chirp(
        rodio::cpal::SampleRate(44100),
        600.0,
        900.0,
        Duration::from_millis(100)// very short
    ).take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.12);

    sink.append(source);
    sink.sleep_until_end();
     });
}


pub fn play_menu_toggle_noise() {
    tokio::spawn(async move {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = chirp(
        rodio::cpal::SampleRate(44100),
        200.0,
        240.0,
        Duration::from_millis(100)// very short
    ).take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.12);

    sink.append(source);
    sink.sleep_until_end();
     });
}
