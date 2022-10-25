use std::collections::VecDeque;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use futures::executor::block_on;
use tokio::sync::mpsc::{Receiver, Sender};
use vlc::{Instance};
use crate::Settings;

pub struct MediaTrack {
    pub path: String,
}

impl Clone for MediaTrack {
    fn clone(&self) -> Self {
        MediaTrack {
            path: self.path.clone(),
        }
    }
}

pub struct MediaPlayerState {
    playlist: VecDeque<MediaTrack>,
}

impl MediaPlayerState {
    pub fn new(media_dir: &str) -> Self {
        let tracks = std::fs::read_dir(media_dir)
            .unwrap()
            .map(|entry| MediaTrack {
                path: entry.unwrap().path().to_str().unwrap().to_string(),
            })
            .collect::<VecDeque<MediaTrack>>();
        
        Self {
            playlist: tracks,
        }
    }
    
    pub fn add_track(&mut self, media: MediaTrack) {
        self.playlist.push_back(media);
    }
    
    pub fn next_track(&mut self) -> Option<MediaTrack> {
        if let Some(next) = self.playlist.pop_front() {
            self.add_track(next.clone());
            Some(next)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum MediaEvent {
    PlayNext,
}

pub struct MediaPlayer {
    instance: Instance,
    player: vlc::MediaPlayer,
    state: Arc<Mutex<MediaPlayerState>>,
    media_rx: Receiver<MediaEvent>,
}

impl MediaPlayer {
    pub fn new(media_tx: Sender<MediaEvent>, media_rx: Receiver<MediaEvent>, settings: &Settings) -> Self {
        let instance = Instance::new().unwrap();
        let player = vlc::MediaPlayer::new(&instance).unwrap();
        instance.add_intf("qt").unwrap();
        
        Self::subscribe_to_vlc_events(media_tx, &mut player.event_manager());
        Self {
            instance,
            player,
            state: Arc::new(Mutex::new(MediaPlayerState::new(settings.media_dir.as_str()))),
            media_rx,
        }
    }
    
    pub async fn listen(&mut self) {
        println!("Listening for media events...");
        self.play_next().await;
        while let Some(event) = self.media_rx.recv().await {
            match event {
                MediaEvent::PlayNext => self.play_next().await,
            }
        }
    }
    
    async fn play_next(&mut self) {
        let mut state = self.state.lock().unwrap();
        if let Some(next) = state.next_track() {
            println!("Playing next track: {}", next.path);
            let media = vlc::Media::new_path(&self.instance, next.path)
                .unwrap();
            self.player.set_media(&media);
            self.player.play().unwrap();
        }
    }
    
    fn subscribe_to_vlc_events(tx: Sender<MediaEvent>, event_manager: &mut vlc::EventManager)
    {
        event_manager.attach(vlc::EventType::MediaPlayerEndReached, move |_, _| {
            block_on(tx.clone().send(MediaEvent::PlayNext)).unwrap();
        }).unwrap();
    }
}