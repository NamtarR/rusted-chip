use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::SineWave};

pub struct Beeper {
    player: Player,
    _sink: MixerDeviceSink,
}

impl Beeper {
    pub fn new() -> Beeper {
        let source = SineWave::new(440.0).amplify(0.20).repeat_infinite();

        let sink = DeviceSinkBuilder::open_default_sink().expect("default audio stream");
        let player = Player::connect_new(&sink.mixer());

        player.pause();
        player.append(source);

        Beeper {
            player: player,
            _sink: sink,
        }
    }

    pub fn play(&mut self) {
        if self.player.is_paused() {
            self.player.play();
        }
    }

    pub fn pause(&mut self) {
        if !self.player.is_paused() {
            self.player.pause();
        }
    }
}
