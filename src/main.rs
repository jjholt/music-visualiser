use std::fs;

use symphonia::core::{
    audio::{AudioBuffer, AudioBufferRef, Signal},
    codecs::DecoderOptions,
    dsp::{complex::Complex, fft::Fft},
    formats::FormatOptions,
    io::{MediaSourceStream, MediaSourceStreamOptions},
    meta::MetadataOptions,
    probe::Hint, sample,
};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("file path not provided");
    let source = fs::File::open(path).expect("failed to open media");

    let mss = MediaSourceStream::new(Box::new(source), Default::default());

    let registry = symphonia::default::get_codecs();
    let mut hint = Hint::new();
    hint.with_extension("flac");

    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .expect("Unsupported format");

    let mut format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|f| f.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .expect("No supported audio tracks");
    let samples_per_window: usize = 512;

    let sample_rate = track.codec_params.sample_rate.unwrap();
    let mut sample_buffer: Vec<f32> = Vec::with_capacity(samples_per_window);

    let mut decoder = registry
        .make(&track.codec_params, &DecoderOptions::default())
        .expect("unsupported codec");
    let fft = Fft::new(samples_per_window);

    while let Ok(packet) = format.next_packet() {
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }
        if let Ok(buffer) = decoder.decode(&packet) {
            match buffer {
                AudioBufferRef::S32(buf) => {
                    for &sample in buf.chan(0) {
                        // let sample = sample as f32 / i32::MAX as f32;
                        sample_buffer.push(sample as f32);
                        if sample_buffer.len() >= samples_per_window {
                            let mut fft_input: Vec<Complex> = sample_buffer
                                .iter()
                                .take(fft.size())
                                .map(|&c| Complex::new(c, 0.0))
                                .collect();

                            fft.fft_inplace(&mut fft_input);
                            println!("Raw input: {:?}", fft_input);
                            std::process::exit(0)
                        }
                    }
                },
                _ => unimplemented!()
            }
        }
    }
}
