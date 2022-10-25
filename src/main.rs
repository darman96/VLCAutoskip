use std::path::Path;
use tokio::sync::mpsc::channel;
use serde::{Serialize, Deserialize};

use crate::audio::AudioListener;
use crate::media_player::MediaPlayer;

pub(crate) mod audio;
pub(crate) mod media_player;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub threshold: f32,
    pub event_delay: u64,
    pub media_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // print current directory
    println!("Current directory: {}", std::env::current_dir()?.display());
    
    let home_dir = std::env::var("HOME").unwrap();
    let settings_file = ".config/vlc-autoskip/settings.json";
    let settings = serde_json::from_str::<Settings>(
        std::fs::read_to_string(Path::new(&home_dir).join(settings_file))
            .unwrap_or_else(|_| panic!("Failed to read settings file: {}", settings_file))
            .as_str()
    ).expect("Failed to parse settings file");
    
    let (media_tx, media_rx) = channel(10);
    let (audio_tx, audio_rx) = channel(10);
    let mut player = MediaPlayer::new(media_tx.clone(), media_rx, &settings);
    let mut audio_listener = AudioListener::new(audio_tx, audio_rx, media_tx.clone(), &settings);
    
    tokio::spawn(async move {
        audio_listener.listen().await;
    });
    player.listen().await;
    
    Ok(())
}

