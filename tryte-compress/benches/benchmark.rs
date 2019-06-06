#[macro_use]
extern crate criterion;
extern crate rand;

use criterion::Criterion;
use tryte_compress::{compress, decompress};
use rand::Rng;

const PACKET_SIZE: usize = 2673;

fn generate_random() -> Vec<u8> {
    let tryte_alphabet: Vec<u8> = vec![57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90];

    let mut output: Vec<u8> = Vec::new();

    for _i in 0..PACKET_SIZE {
        let num = rand::thread_rng().gen_range(0, tryte_alphabet.len());
        output.push(tryte_alphabet[num]);
    }

    output
}

fn test_compress(trytes: &Vec<u8>) {
    compress(&trytes);
} 

fn test_decompress(compressed: &Vec<u8>) {
    decompress(&compressed);
}

fn criterion_benchmark(c: &mut Criterion) {
    let trytes = generate_random();
    let compressed = compress(&trytes);
    c.bench_function("Compress", move |b| b.iter(|| test_compress(&trytes)));
    c.bench_function("Decompress", move |b| b.iter(|| test_decompress(&compressed)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);