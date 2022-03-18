use silk_rs::{decode_silk, encode_silk};

fn main() {
    let input = std::fs::read("test.pcm").unwrap();
    let output = encode_silk(input, 24000, 24000, 24000, 0, 2, false, false, true).unwrap();
    std::fs::write("output.silk", output).unwrap();

    let input = std::fs::read("output.silk").unwrap();
    let output = decode_silk(input, 24000,0,1,false,0, false).unwrap();
    std::fs::write("output.pcm", output).unwrap();
}
