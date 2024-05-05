#[cfg(test)]
mod speed_test {
    use crate::{decrease_song_speed, increase_song_speed};
    use rodio::Sink;

    #[test]
    fn test_increase_speed() {
        let sink = Sink::new_idle().0;
        sink.set_speed(1.0);
        let step = 0.02;
        let expected_speed = sink.speed() + step;

        increase_song_speed(&sink, step);

        assert_eq!(sink.speed(), expected_speed);
    }

    #[test]
    fn test_decrease_speed_returns_ok() {
        let sink = Sink::new_idle().0;
        sink.set_speed(1.0);

        let step = 0.02;
        let expected_speed = sink.speed() - step;

        let result = decrease_song_speed(&sink, step);

        assert!(result.is_ok());
        assert_eq!(sink.speed(), expected_speed);
    }

    #[test]
    fn test_decrease_speed_returns_error_if_cannot_decrease() {
        let sink = Sink::new_idle().0;
        sink.set_speed(1.0);

        let step = 1.02;
        let expected_speed = sink.speed();

        let result = decrease_song_speed(&sink, step);

        assert!(result.is_err());
        assert_eq!(sink.speed(), expected_speed);
    }
}

#[cfg(test)]
mod volume_test {
    use rodio::Sink;

    use crate::{decrease_song_volume, increase_song_volume};

    #[test]
    fn test_increase_volume_returns_ok() {
        let sink = Sink::new_idle().0;
        sink.set_volume(0.5);

        let step = 0.02;
        let expected_volume = sink.volume() + step;

        let result = increase_song_volume(&sink, step);

        assert!(result.is_ok());
        assert_eq!(sink.volume(), expected_volume)
    }

    #[test]
    fn test_increase_volume_returns_error_if_cannot_increase() {
        let sink = Sink::new_idle().0;
        sink.set_volume(1.0);

        let step = 0.02;
        let expected_volume = sink.volume();

        let result = increase_song_volume(&sink, step);

        assert!(result.is_err());
        assert_eq!(sink.volume(), expected_volume);
    }

    #[test]
    fn test_decrease_volume_returns_ok() {
        let sink = Sink::new_idle().0;
        sink.set_volume(0.5);

        let step = 0.02;
        let expected_volume = sink.volume() - step;

        let result = decrease_song_volume(&sink, step);

        assert!(result.is_ok());
        assert_eq!(sink.volume(), expected_volume)
    }

    #[test]
    fn test_decrease_volume_returns_error_if_cannot_decrease() {
        let sink = Sink::new_idle().0;
        sink.set_volume(0.1);

        let step = 0.2;
        let expected_volume = sink.volume();

        let result = decrease_song_volume(&sink, step);

        assert!(result.is_err());
        assert_eq!(sink.volume(), expected_volume);
    }
}
