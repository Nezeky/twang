use twang::{
    gen::{Triangle},
    mono::Mono64,
    Audio, Hz,
};

mod wav;

fn main() {
    let mut tri = Triangle::new(Hz(220.0)); // A4
    let mut out = Audio::<Mono64>::with_silence(48_000, 48_000 * 5);
    out.generate(&mut tri);
    wav::write(out, "triangle.wav").expect("Failed to write WAV file");
}
