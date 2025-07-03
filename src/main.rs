use std::fs;

use symphonia::
    core::{
        codecs::{DecoderOptions},
        formats::FormatOptions,
        io::{MediaSourceStream, MediaSourceStreamOptions},
        meta::MetadataOptions,
        probe::Hint,
    }
;
fn main() {
    let registry = symphonia::default::get_codecs();
    let probe = symphonia::default::get_probe();
    let source = fs::File::open("song.flac").unwrap();
    let options = MediaSourceStreamOptions::default();
    let mss = MediaSourceStream::new(Box::new(source), options);
    let hint = Hint::default();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let probe_format = probe
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();

    let mut format = probe_format.format;
    let track = format.tracks().iter().find(|f| f.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL).unwrap();

    let mut decoder = registry.make(&track.codec_params, &DecoderOptions::default()).unwrap();
    let packet = format.next_packet().unwrap();
    let buffer = decoder.decode(&packet).unwrap();

    //     for track in tracks {
    // }
}
