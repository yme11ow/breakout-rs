use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound, play_sound_once, set_sound_volume, stop_sound};

pub struct Audio {
    ball_brick: Sound,
    ball_paddle: Sound,
    ball_wall: Sound,
    ball_falls_off: Sound,
    click_button: Sound,
    start_game: Sound,
    title_music: Sound,
    gameplay_music: Sound,
    game_over: Sound,
    you_win: Sound,
    pub muted: bool,
}

impl Audio {
    pub async fn load() -> Self {
        Audio {
            ball_brick: load("audio/ball_brick.wav").await,
            ball_paddle: load("audio/ball_paddle.wav").await,
            ball_wall: load("audio/ball_wall.wav").await,
            ball_falls_off: load("audio/ball_falls_off.wav").await,
            click_button: load("audio/click-button2.wav").await,
            start_game: load("audio/start_game.wav").await,
            title_music: load("audio/title_music.wav").await,
            gameplay_music: load("audio/gameplay_music.wav").await,
            game_over: load("audio/game_over.wav").await,
            you_win: load("audio/you_win.wav").await,
            muted: false,
        }
    }

    pub fn ball_brick(&self) {
        play_sound_once(&self.ball_brick);
    }

    pub fn ball_paddle(&self) {
        play_sound_once(&self.ball_paddle);
    }

    pub fn ball_wall(&self) {
        play_sound_once(&self.ball_wall);
    }

    pub fn ball_falls_off(&self) {
        play_sound_once(&self.ball_falls_off);
    }

    pub fn click_button(&self) {
        play_sound_once(&self.click_button);
    }

    pub fn start_game(&self) {
        play_sound_once(&self.start_game);
    }

    pub fn play_title_music(&self) {
        stop_sound(&self.gameplay_music);
        self.play_looped(&self.title_music);
    }

    pub fn play_gameplay_music(&self) {
        stop_sound(&self.title_music);
        stop_sound(&self.game_over);
        stop_sound(&self.you_win);
        self.play_looped(&self.gameplay_music);
    }

    pub fn play_game_over_music(&self) {
        stop_sound(&self.gameplay_music);
        self.play_looped(&self.game_over);
    }

    pub fn play_victory_music(&self) {
        stop_sound(&self.gameplay_music);
        self.play_looped(&self.you_win);
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
        let volume = self.music_volume();
        for music in [&self.title_music, &self.gameplay_music, &self.game_over, &self.you_win] {
            set_sound_volume(music, volume);
        }
    }

    fn music_volume(&self) -> f32 {
        if self.muted { 0.0 } else { 1.0 }
    }

    fn play_looped(&self, sound: &Sound) {
        play_sound(
            sound,
            PlaySoundParams {
                looped: true,
                volume: self.music_volume(),
            },
        );
    }
}

async fn load(path: &str) -> Sound {
    load_sound(path)
        .await
        .unwrap_or_else(|e| panic!("failed to load {path}: {e}"))
}
