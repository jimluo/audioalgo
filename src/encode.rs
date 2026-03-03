use fdk_aac::enc::{AudioObjectType, BitRate, ChannelMode, Encoder, EncoderParams, Transport};
use std::collections::VecDeque;

pub struct Encode {
    enc: Encoder,
    queue: VecDeque<i16>,
}

impl Encode {
    pub fn new(sample_rate: u32) -> Self {
        let params = EncoderParams {
            bit_rate: BitRate::VbrVeryLow,
            sample_rate,
            transport: Transport::Adts,
            channels: ChannelMode::Mono,
            audio_object_type: AudioObjectType::Mpeg4LowComplexity,
        };

        Encode {
            enc: Encoder::new(params).unwrap(),
            queue: VecDeque::new(),
        }
    }

    pub fn encode(&mut self, pcm: &[i16]) -> Option<Vec<u8>> {
        const SAMPLES_PER_CHUNK: usize = 1024;

        self.queue.extend(pcm);
        if self.queue.len() >= SAMPLES_PER_CHUNK {
            let buf: Vec<i16> = self.queue.drain(0..SAMPLES_PER_CHUNK).collect();
            let mut out_enc = [0u8; 1536];
            if let Ok(info) = self.enc.encode(&buf, &mut out_enc) {
                return Some(out_enc[..info.output_size].to_vec());
            }
        }

        return None;
    }
}

#[test]
fn test_encode() {
    let mut enc = Encode::new(8000);
    let aac = enc.encode(&[0i16; 1023]);
    assert!(aac.is_none());

    let aac = enc.encode(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert!(aac.is_some());
    assert!(aac.unwrap().len() > 1);
}
