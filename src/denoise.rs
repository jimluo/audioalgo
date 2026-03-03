use std::mem::transmute_copy;

use sherpa_rs::sherpa_rs_sys as sherpa;

//  $env:SHERPA_LIB_PATH="D:\repos\asr\k2\sherpa-rs\sherpa-onnx-v1.11.5-win-x64-shared"

// pub struct SherpaOnnxOfflineSpeechDenoiserGtcrnModelConfig {
//     pub model: *const ::std::os::raw::c_char,
// }

// pub struct SherpaOnnxOfflineSpeechDenoiserModelConfig {
//     pub gtcrn: SherpaOnnxOfflineSpeechDenoiserGtcrnModelConfig,
//     pub num_threads: i32,
//     pub debug: i32,
//     pub provider: *const ::std::os::raw::c_char,
// }

// pub struct SherpaOnnxOfflineSpeechDenoiserConfig {
//     pub model: SherpaOnnxOfflineSpeechDenoiserModelConfig,
// }

// pub struct SherpaOnnxOfflineSpeechDenoiser {
//     _unused: [u8; 0],
// }

// pub struct SherpaOnnxDenoisedAudio {
//     pub samples: *const f32,
//     pub n: i32,
//     pub sample_rate: i32,
// }SherpaOnnxOfflineSpeechDenoiserConfig

// extern "C" {
//     pub fn SherpaOnnxCreateOfflineSpeechDenoiser(config: *const SherpaOnnxOfflineSpeechDenoiserConfig) -> *const SherpaOnnxOfflineSpeechDenoiser;

//     pub fn SherpaOnnxDestroyOfflineSpeechDenoiser(sd: *const SherpaOnnxOfflineSpeechDenoiser);

//     pub fn SherpaOnnxOfflineSpeechDenoiserGetSampleRate(sd: *const SherpaOnnxOfflineSpeechDenoiser) -> i32;

//     pub fn SherpaOnnxOfflineSpeechDenoiserRun(
//         sd: *const SherpaOnnxOfflineSpeechDenoiser,
//         samples: *const f32,
//         n: i32,
//         sample_rate: i32,
//     ) -> *const SherpaOnnxDenoisedAudio;

//     pub fn SherpaOnnxDestroyDenoisedAudio(p: *const SherpaOnnxDenoisedAudio);
// }
pub struct Denoise {
    denoiser: *const sherpa::SherpaOnnxOfflineSpeechDenoiser,
}

impl Denoise {
    pub fn new() -> Self {
        unsafe {
            let cfg = sherpa::SherpaOnnxOfflineSpeechDenoiserConfig {
                model: sherpa::SherpaOnnxOfflineSpeechDenoiserModelConfig {
                    gtcrn: sherpa::SherpaOnnxOfflineSpeechDenoiserGtcrnModelConfig {
                        model: "./gtcrn.onnx".as_ptr() as *const i8,
                    },
                    num_threads: 4,
                    debug: 0,
                    provider: "k2".as_ptr() as *const i8,
                },
            };

            Denoise {
                denoiser: sherpa::SherpaOnnxCreateOfflineSpeechDenoiser(&cfg),
            }
        }
    }

    pub fn process(&self, pcm: &[f32]) -> Vec<f32> {
        unsafe {
            let denoised = sherpa::SherpaOnnxOfflineSpeechDenoiserRun(
                self.denoiser,
                pcm.as_ptr(),
                pcm.len() as i32,
                16000,
            );
            let n = (*denoised).n as usize;
            let pcm2: &[f32] = std::slice::from_raw_parts((*denoised).samples, n);
            let pcm2 = pcm2.to_vec();
            sherpa::SherpaOnnxDestroyDenoisedAudio(denoised);

            pcm2
        }
    }
}

impl Drop for Denoise {
    fn drop(&mut self) {
        unsafe {
            sherpa::SherpaOnnxDestroyOfflineSpeechDenoiser(self.denoiser);
        }
    }
}
