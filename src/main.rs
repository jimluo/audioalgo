mod config;
mod denoise;
mod encode;
mod voice;

use encode::Encode;
use std::io::Write;
use voice::{Voice, WINDOW_SIZE};

const MAX_I16: f32 = i16::MAX as f32;
pub const FRAME_SIZE: usize = 320;

// fn main() {
//     let fname = std::env::args().nth(1).expect("Missing wav file");

//     let mut reader = hound::WavReader::open(fname.clone()).unwrap();
//     let rate = reader.spec().sample_rate;
//     assert_eq!(rate, 8000, "The sample rate must be 8000.");
//     println!("{fname} duration: {}s", reader.len() / 8000);

//     let mut pcm_frames: Vec<i16> = Vec::new();
//     let mut samples: Vec<f32> = Vec::with_capacity(reader.len() as usize);
//     for sample in reader.samples::<i16>() {
//         let s_i16 = sample.unwrap();
//         pcm_frames.push(s_i16);

//         let s = s_i16 as f32 / MAX_I16;
//         samples.push(s);
//         samples.push(s);
//     }

//     let denoise = Denoise::new();
//     let mut enc = Encode::new(8000);
//     let mut aacfile = std::fs::File::create(format!("{fname}.aac")).unwrap();
//     for pcm_frame in pcm_frames.chunks_exact(FRAME_SIZE) {
//         let pcm = denoise.denoise(pcm_frame);
//         if let Some(aac) = enc.encode(pcm) {
//             if let Err(e) = aacfile.write(&aac) {
//                 println!("aacfile write: {e}");
//             }
//         }
//     }

//     // let mut vad = Voice::new();
//     // for pcm_frame in samples.chunks_exact(WINDOW_SIZE) {
//     //     vad.detect(pcm_frame);
//     // }
// }

use webrtc_audio_processing::*;

fn main() {
    let fname = std::env::args().nth(1).expect("Missing wav file");

    let mut reader = hound::WavReader::open(fname.clone()).unwrap();
    let rate = reader.spec().sample_rate;
    assert_eq!(rate, 8000, "The sample rate must be 8000.");
    println!("{fname} duration: {}s", reader.len() / 8000);

    let mut samples: Vec<f32> = Vec::with_capacity(reader.len() as usize);
    for sample in reader.samples::<i16>() {
        let s_i16 = sample.unwrap();
        let s = s_i16 as f32 / MAX_I16;
        samples.push(s);
        samples.push(s);
    }

    let config = InitializationConfig {
        num_capture_channels: 1, // Stereo mic input
        num_render_channels: 1,  // Stereo speaker output
        ..InitializationConfig::default()
    };

    let mut ap = Processor::new(&config).unwrap();

    let config = Config {
        // echo_cancellation: Some(EchoCancellation {
        //     suppression_level: EchoCancellationSuppressionLevel::High,
        //     enable_delay_agnostic: false,
        //     enable_extended_filter: false,
        //     stream_delay_ms: None,
        // }),
        // gain_control: Some(GainControl {
        //     mode: GainControlMode::AdaptiveDigital,
        //     target_level_dbfs: 50i32,
        //     compression_gain_db: 20i32,
        //     enable_limiter: false,
        // }),
        noise_suppression: Some(NoiseSuppression {
            suppression_level: NoiseSuppressionLevel::High,
        }),
        // voice_detection: Some(VoiceDetection {
        //     detection_likelihood: VoiceDetectionLikelihood::Moderate,
        // }),
        ..Config::default()
    };
    ap.set_config(config);

    ap.process_render_frame(&mut samples).unwrap();

    println!("Successfully processed a render and capture frame through WebRTC!");
}

// thread 'main' panicked at /root/.cargo/registry/src/mirrors.bfsu.edu.cn-eb00624c53167367/webrtc-audio-processing-0.4.0/src/lib.rs:143:9:
// assertion `left == right` failed
//   left: 4431872
//  right: 480
