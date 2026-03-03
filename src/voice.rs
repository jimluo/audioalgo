use sherpa_rs::vad::{Vad, VadConfig};
use std::collections::VecDeque;

pub const WINDOW_SIZE: usize = 512;
const SAMPLE_RATE: u32 = 16000;

pub struct Voice {
    vad: Vad,
    queue: VecDeque<f32>,
}

impl Voice {
    pub fn new() -> Self {
        let config = VadConfig {
            model: "./vad.onnx".into(),
            window_size: WINDOW_SIZE as i32,
            min_silence_duration: 0.1,
            min_speech_duration: 0.25,
            max_speech_duration: 8.0,
            num_threads: Some(4),
            ..Default::default()
        };

        Voice {
            vad: Vad::new(config, 20.0).unwrap(),
            queue: VecDeque::new(),
        }
    }

    pub fn detect(&mut self, pcm_frame: &[f32]) -> Option<i32> {
        self.queue.extend(pcm_frame);
        if self.queue.len() >= WINDOW_SIZE {
            let buf = self.queue.drain(0..WINDOW_SIZE).collect::<Vec<f32>>();

            self.vad.accept_waveform(buf);

            if self.vad.is_speech() {
                while !self.vad.is_empty() {
                    let segment = self.vad.front();
                    let start_sec = (segment.start as f32) / SAMPLE_RATE as f32;
                    let duration_sec = (segment.samples.len() as f32) / SAMPLE_RATE as f32;
                    println!("start={}s duration={}s", start_sec, duration_sec);
                    self.vad.pop();

                    return Some(segment.start);
                }
            }
        }

        return None;
    }
}

#[test]
fn test_vad() {
    let mut vad = Voice::new();
    let voice_start = vad.detect(&[0f32; 513]);
    assert!(voice_start.is_none());
}
