use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound, play_sound_once, stop_sound};

pub struct Audio {
    ball_brick: Sound,
    ball_paddle: Sound,
    ball_wall: Sound,
    click_button: Sound,
    start_game: Sound,
    title_music: Sound,
    gameplay_music: Sound,
    game_over: Sound,
    you_win: Sound,
}

impl Audio {
    pub async fn load() -> Self {
        Audio {
            ball_brick: load("audio/ball_brick.wav").await,
            ball_paddle: load("audio/ball_paddle.wav").await,
            ball_wall: load("audio/ball_wall.wav").await,
            click_button: load("audio/click-button2.wav").await,
            start_game: load("audio/start_game.wav").await,
            title_music: load("audio/title_music.wav").await,
            gameplay_music: load("audio/gameplay_music.wav").await,
            game_over: load("audio/game_over.wav").await,
            you_win: load("audio/you_win.wav").await,
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

    pub fn click_button(&self) {
        play_sound_once(&self.click_button);
    }

    pub fn start_game(&self) {
        play_sound_once(&self.start_game);
    }

    pub fn play_title_music(&self) {
        stop_sound(&self.gameplay_music);
        play_looped(&self.title_music);
    }

    pub fn play_gameplay_music(&self) {
        stop_sound(&self.title_music);
        play_looped(&self.gameplay_music);
    }

    pub fn play_game_over_music(&self) {
        stop_sound(&self.gameplay_music);
        play_looped(&self.game_over);
    }

    pub fn play_victory_music(&self) {
        stop_sound(&self.gameplay_music);
        play_looped(&self.you_win);
    }

    pub fn stop_music(&self) {
        stop_sound(&self.title_music);
        stop_sound(&self.gameplay_music);
    }
}

async fn load(path: &str) -> Sound {
    load_sound(path)
        .await
        .unwrap_or_else(|e| panic!("failed to load {path}: {e}"))
}

fn play_looped(sound: &Sound) {
    play_sound(
        sound,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );
}
