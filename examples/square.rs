use fon::{chan::Ch16, Audio, Frame, Stream};
use twang::osc::Pulse;
use twang::Synth;

mod wav;

// State of the synthesizer.
struct Processors {
    pulse: Pulse,
}

fn main() {
    // Initialize audio
    let mut audio = Audio::<Ch16, 2>::new(48_000);
    // Create audio processors
    let proc = Processors {
        pulse: Pulse::new(),
    };
    // Build synthesis algorithm
    let mut synth = Synth::new(proc, |proc, frame: Frame<_, 2>| {
        // Calculate the next sample for each processor
        let square = proc.pulse.next(440.0, Default::default());
        // Pan the generated audio center
        frame.pan(square, 0.0)
    });
    // Synthesize 5 seconds of audio
    synth.extend(&mut audio, 48_000 * 5);
    // Write synthesized audio to WAV file
    wav::write(audio, "square.wav").expect("Failed to write WAV file");
}
