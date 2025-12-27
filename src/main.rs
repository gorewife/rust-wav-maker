use std::io::{Write, Result};
use std::fs::File;
use std::f32::consts::PI;

const RIFF_SIGNATURE: &[u8; 4] = b"RIFF";
const WAVE_SIGNATURE: &[u8; 4] = b"WAVE";
const FMT_SUBCHUNK: &[u8; 4] = b"fmt ";
const DATA_SUBCHUNK: &[u8; 4] = b"data";
const PCM_AUDIO_FORMAT: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;
const FREQ: u32 = 44100;
const DURATION: u32 = 3;
const TONE_HZ: f32 = 440.0;

fn write_le_16<W: Write>(writer: &mut W, n: u16) -> Result<()> {
    writer.write_all(&n.to_le_bytes())
}

fn write_le_32<W: Write>(writer: &mut W, n: u32) -> Result<()> {
    writer.write_all(&n.to_le_bytes())
}

fn main() -> Result<()> {
    let mut file = File::create("test.wav")?;
    
    let num_samples: u32 = DURATION * FREQ;
    let data_chunk_size = num_samples * (BITS_PER_SAMPLE as u32 / 8);
    let file_size: u32 = 36 + data_chunk_size;

    file.write_all(RIFF_SIGNATURE)?;
    write_le_32(&mut file, file_size)?;
    file.write_all(WAVE_SIGNATURE)?;

    file.write_all(FMT_SUBCHUNK)?;
    write_le_32(&mut file, 16)?; // Subchunk1Size for PCM
    write_le_16(&mut file, PCM_AUDIO_FORMAT)?; // AudioFormat
    write_le_16(&mut file, 1)?; // NumChannels (mono)
    write_le_32(&mut file, FREQ)?; // SampleRate
    write_le_32(&mut file, FREQ * (BITS_PER_SAMPLE as u32 / 8))?; // ByteRate
    write_le_16(&mut file, (BITS_PER_SAMPLE / 8) as u16)?; // BlockAlign
    write_le_16(&mut file, BITS_PER_SAMPLE)?; // BitsPerSample

    file.write_all(DATA_SUBCHUNK)?;
    write_le_32(&mut file, data_chunk_size)?;

    for n in 0..num_samples {
        let t = n as f32 / FREQ as f32;
        let sample = (i16::MAX as f32 * (2.0 * PI * TONE_HZ * t).sin()) as i16;
        file.write_all(&sample.to_le_bytes())?;
    }

    println!("File created succesfully");
    Ok(())
}
