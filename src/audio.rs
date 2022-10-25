use cpal::traits::{DeviceTrait, HostTrait};
use futures::executor::block_on;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::media_player::MediaEvent;
use crate::Settings;

#[derive(Debug)]
pub enum AudioEvent {
    ThresholdCrossed,
}

pub struct AudioListener {
    _stream: cpal::Stream,
    audio_rx: Receiver<AudioEvent>,
    media_tx: Sender<MediaEvent>,
    event_delay: u64,
}

unsafe impl Send for AudioListener {}

impl AudioListener {
    pub fn new(audio_tx: Sender<AudioEvent>, audio_rx: Receiver<AudioEvent>, media_tx: Sender<MediaEvent>, settings: &Settings) -> Self {
        let _stream = Self::subscribe_to_audio(settings.threshold, audio_tx);
        Self { 
            _stream,
            audio_rx,
            media_tx,
            event_delay: settings.event_delay 
        }
    }
    
    fn subscribe_to_audio(threshold: f32, audio_tx: Sender<AudioEvent>) -> cpal::Stream {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .expect("Failed to get default input device");
        let config = device
            .default_input_config()
            .expect("Failed to get default input config")
            .config();
        
        device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let peak = data.iter().map(|&x| x.abs()).fold(0.0, f32::max);
                if peak > threshold {
                    println!("Threshold crossed: {}", peak);
                    block_on(audio_tx.send(AudioEvent::ThresholdCrossed)).unwrap();
                }
            },
            move |err| {
                eprintln!("an error occurred on input stream: {}", err);
            })
            .expect("error building input stream")
    }
    
    pub async fn listen(&mut self) {
        println!("Listening for audio events...");
        let mut last_event = 0u64;
        while let Some(event) = self.audio_rx.recv().await {
            match event {
                AudioEvent::ThresholdCrossed => {
                    let now = chrono::Local::now().timestamp_millis() as u64;
                    let delta = now - last_event;
                    if delta > self.event_delay {
                        last_event = now;
                        self.media_tx.send(MediaEvent::PlayNext).await.unwrap();
                    }
                },
            }
        }
    }
}

