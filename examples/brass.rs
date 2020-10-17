use fon::{
    mono::Mono64,
    Audio,
};
use twang::{Synth, Pink, Mix};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new();
    // Create the pink noise generator.
    let mut pink = Pink::new();
    // Generate audio samples.
    synth.gen(audio.sink(..), |fc| {
        let pink = pink.noise();
        let tone = fc.freq(220.0).amp(12.0).clamp().amp(0.75);
        let airy = tone.abs().amp(pink.abs().into());
        
        let pone = fc.freq(220.0).amp(12.0).clamp().abs();
        let ptwo = fc.freq(220.0).triangle();
        let main = pone.amp(ptwo.into());

        [airy, main].iter().cloned().mix()
    }); 

    // Write synthesized audio to WAV file.
    wav::write(audio, "brass.wav").expect("Failed to write WAV file");
}
