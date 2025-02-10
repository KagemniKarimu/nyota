use rodio::source::chirp;
use rodio::{
    source::{PinkNoise, SineWave, Source},
    OutputStream, Sink,
};
use std::time::Duration;

pub fn play_welcome_chirp() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = chirp(
            rodio::cpal::SampleRate(44100),
            600.0,
            900.0,
            Duration::from_millis(100), // very short
        )
        .take_duration(Duration::from_secs_f32(0.3))
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
            Duration::from_millis(100), // very short
        )
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.12);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_keystroke() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Very short, high-frequency click
        let source = SineWave::new(2000.0) // Higher frequency for sharp click
            .take_duration(Duration::from_millis(10)) // Very short duration
            .amplify(0.1); // Lower volume for subtlety

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_keystroke2() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Short white noise burst for a "click" effect
        let source = PinkNoise::new(rodio::cpal::SampleRate(3000))
            .take_duration(Duration::from_millis(10))
            .amplify(0.1);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_enter() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Rising tone for confirmation
        let source = SineWave::new(1000.0)
            .take_duration(Duration::from_millis(100))
            .amplify(0.2);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn play_backspace() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Using noise for a sharp, short sound
        let source = PinkNoise::new(rodio::cpal::SampleRate(10))
            .take_duration(Duration::from_millis(20))
            .amplify(0.1);

        sink.append(source);
        sink.sleep_until_end();
    });
}

// Previously using SquareWave for mode changes
pub fn _play_mode_normal() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Using two sine waves at different frequencies instead
        let source = SineWave::new(800.0)
            .take_duration(Duration::from_millis(50))
            .amplify(0.15);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_mode_insert() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Rising indicator
        let source = SineWave::new(600.0)
            .take_duration(Duration::from_millis(80))
            .amplify(0.15);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn play_message_sent() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Short ascending chirp
        let source = SineWave::new(1200.0)
            .take_duration(Duration::from_millis(150))
            .amplify(0.2);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_message_received() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Gentle notification
        let source = SineWave::new(800.0)
            .take_duration(Duration::from_millis(100))
            .fade_in(Duration::from_millis(20))
            .amplify(0.15);

        sink.append(source);
        sink.sleep_until_end();
    });
}

pub fn _play_error() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Descending warning tone
        let source = SineWave::new(1000.0)
            .take_duration(Duration::from_millis(200))
            .amplify(0.25);

        sink.append(source);
        sink.sleep_until_end();
    });
}

// Previously using SquareWave in mix
pub fn _play_connection_change() {
    tokio::spawn(async move {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Using combination of sine wave and brief noise
        let source = SineWave::new(600.0)
            .take_duration(Duration::from_millis(30))
            .amplify(0.15);

        sink.append(source);
        sink.sleep_until_end();
    });
}
