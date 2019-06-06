use std::collections::HashMap;
#[macro_use] extern crate lazy_static;

struct Bits {
    bits: u8,
    length: u8,
}

lazy_static! {
    static ref HUFFMAN_TABLE: HashMap<u8, Bits> = {
        let mut m = HashMap::new();
        m.insert(65, Bits { bits: 0b0100, length: 4 });
        m.insert(66, Bits { bits: 0b11110, length: 5 });
        m.insert(67, Bits { bits: 0b0001, length: 4 });
        m.insert(68, Bits { bits: 0b11111, length: 5 });
        m.insert(69, Bits { bits: 0b11101, length: 5 });
        m.insert(70, Bits { bits: 0b11100, length: 5 });
        m.insert(71, Bits { bits: 0b10010, length: 5 });
        m.insert(72, Bits { bits: 0b11000, length: 5 });
        m.insert(73, Bits { bits: 0b10111, length: 5 });
        m.insert(74, Bits { bits: 0b10000, length: 5 });
        m.insert(75, Bits { bits: 0b10001, length: 5 });
        m.insert(76, Bits { bits: 0b10011, length: 5 });
        m.insert(77, Bits { bits: 0b00001, length: 5 });
        m.insert(78, Bits { bits: 0b11001, length: 5 });
        m.insert(79, Bits { bits: 0b10101, length: 5 });
        m.insert(80, Bits { bits: 0b11011, length: 5 });
        m.insert(81, Bits { bits: 0b01010, length: 5 });
        m.insert(82, Bits { bits: 0b11010, length: 5 });
        m.insert(83, Bits { bits: 0b0010, length: 4 });
        m.insert(84, Bits { bits: 0b10100, length: 5 });
        m.insert(85, Bits { bits: 0b01011, length: 5 });
        m.insert(86, Bits { bits: 0b01111, length: 5 });
        m.insert(87, Bits { bits: 0b10110, length: 5 });
        m.insert(88, Bits { bits: 0b01101, length: 5 });
        m.insert(89, Bits { bits: 0b01100, length: 5 });
        m.insert(90, Bits { bits: 0b01110, length: 5 });
        m.insert(57, Bits { bits: 0b0011, length: 4 });
        m.insert(49, Bits { bits: 0b000001, length: 6 });
        m.insert(50, Bits { bits: 0b0000001, length: 7 });
        m.insert(51, Bits { bits: 0b00000001, length: 8 });
        m.insert(95, Bits { bits: 0b00000000, length: 8 });
        m
    };

    static ref HUFFMAN_TABLE_REVERSE: Vec<HashMap<u8, u8>> = {
        let mut m = Vec::new();
        m.push(HashMap::new());
        m.push(HashMap::new());
        m.push(HashMap::new());
        m.push(HashMap::new());

        let mut rev_4: HashMap<u8, u8> = HashMap::new();
        rev_4.insert(0b0010, 65);
        rev_4.insert(0b1000, 67);
        rev_4.insert(0b0100, 83);
        rev_4.insert(0b1100, 57);
        m.push(rev_4);

        let mut rev_5: HashMap<u8, u8> = HashMap::new();
        rev_5.insert(0b01111, 66);
        rev_5.insert(0b11111, 68);
        rev_5.insert(0b10111, 69);
        rev_5.insert(0b00111, 70);
        rev_5.insert(0b01001, 71);
        rev_5.insert(0b00011, 72);
        rev_5.insert(0b11101, 73);
        rev_5.insert(0b00001, 74);
        rev_5.insert(0b10001, 75);
        rev_5.insert(0b11001, 76);
        rev_5.insert(0b10000, 77);
        rev_5.insert(0b10011, 78);
        rev_5.insert(0b10101, 79);
        rev_5.insert(0b11011, 80);
        rev_5.insert(0b01010, 81);
        rev_5.insert(0b01011, 82);
        rev_5.insert(0b00101, 84);
        rev_5.insert(0b11010, 85);
        rev_5.insert(0b11110, 86);
        rev_5.insert(0b01101, 87);
        rev_5.insert(0b10110, 88);
        rev_5.insert(0b00110, 89);
        rev_5.insert(0b01110, 90);        
        m.push(rev_5);

        let mut rev_6: HashMap<u8, u8> = HashMap::new();
        rev_6.insert(0b100000, 49);
        m.push(rev_6);

        let mut rev_7: HashMap<u8, u8> = HashMap::new();
        rev_7.insert(0b1000000, 50);
        m.push(rev_7);

        let mut rev_8: HashMap<u8, u8> = HashMap::new();
        rev_8.insert(0b10000000, 51);
        rev_8.insert(0b00000000, 95);
        m.push(rev_8);
        m
    };
}

const END_OF_DATA: u8 = 95;

const RUN_MIN_LENGTH: u16 = 3;
const ONE_TRYTE_MAX: u16 = 26;
const TWO_TRYTE_MAX: u16 = 728;
const THREE_TRYTE_MAX: u16 = 19682;

pub fn compress(trytes: &Vec<u8>) -> Vec<u8> {
    // First run length encode the data to reduce the content
    let mut rle_encoded: Vec<u8> = Vec::new();
    run_length_encode(trytes, &mut rle_encoded);

    // Add the end of data marker so the decompress knows
    // when to finish processing the data
    rle_encoded.push(END_OF_DATA);

    // Convert the rle encoded trytes to their huffman form
    let mut huffman_encoded: Vec<u8> = Vec::new();
    let mut encoded_bits = 0;
    let mut encoded_bits_length = 0;

    for i in 0..rle_encoded.len() {
        let h_bits = &HUFFMAN_TABLE[&rle_encoded[i]];
        for j in (0..(h_bits.length)).rev() {
            encoded_bits |= ((h_bits.bits >> j) & 0x01) << encoded_bits_length;
            encoded_bits_length = encoded_bits_length + 1;
            if encoded_bits_length == 8 {
                huffman_encoded.push(encoded_bits);
                encoded_bits_length = 0;
                encoded_bits = 0;
            }
        }
    }

    // If there are any remaining bits make sure we don't miss them
    if encoded_bits_length > 0 {
        huffman_encoded.push(encoded_bits);
    }

    huffman_encoded
}

pub fn decompress(bytes: &Vec<u8>) -> Vec<u8> {
    // Convert the huffman encoded data back to the full rle
    let mut decoded: Vec<u8> = Vec::new();
    let mut read_pos = 0;
    let mut found_end = false;
    let mut bit_index = 0;

    while !found_end {
        let mut key = 0;
        let mut key_bit_count = 0;

        // Build a key until we find it in the huffman table
        while HUFFMAN_TABLE_REVERSE[key_bit_count].get(&key) == None {
            key |= ((bytes[read_pos] >> bit_index) & 0x01) << key_bit_count;
            key_bit_count = key_bit_count + 1;
            bit_index = bit_index + 1;

            if bit_index == 8 {
                read_pos= read_pos + 1;
                bit_index = 0;
            }
        }

        // A key was found, if it was end of data then
        // finished the decompress, otherwise just add it
        // to the decompressed data
        let val = HUFFMAN_TABLE_REVERSE[key_bit_count][&key];
        if val == END_OF_DATA {
            found_end = true;
        } else {
            decoded.push(val);
        }
    }

    // Run length decode the data
    run_length_decode(decoded)
}

fn run_length_encode(trytes: &Vec<u8>, rle_encoded: &mut Vec<u8>) {
    let mut prev = trytes[0];
    let mut count: u16 = 1;

    for i in 1..trytes.len() {
        if trytes[i] != prev {
            append_run(rle_encoded, count, prev);
            count = 1;
            prev = trytes[i];
        } else {
            count = count + 1;
        }
    }

    append_run(rle_encoded, count, prev);
}

fn append_run(encoded: &mut Vec<u8>, count: u16, prev: u8) {
    if count == 1 {
        encoded.push(prev);
    } else {
        let mut remaining: u16 = count;

        while remaining >= RUN_MIN_LENGTH {
            let current_run_length = if remaining > THREE_TRYTE_MAX { THREE_TRYTE_MAX } else { remaining };
            number_to_rle(encoded, prev, current_run_length);
            remaining -= current_run_length;
        }

        if remaining > 0 {
            for _i in 0..remaining {
                encoded.push(prev);
            }
        }
    }
}

fn run_length_decode(encoded: Vec<u8>) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();
    let mut i = 0;

    while i < encoded.len() {
        if encoded[i] == 49 {
            rle_to_number(&mut decoded, encoded[i + 2], encoded[i + 1], 0, 0);
            i += 2;
        } else if encoded[i] == 50 {
            rle_to_number(&mut decoded, encoded[i + 3], encoded[i + 1], encoded[i + 2], 0);
            i += 3;
        } else if encoded[i] == 50 {
            rle_to_number(&mut decoded, encoded[i + 4], encoded[i + 1], encoded[i + 2], encoded[i + 3]);
            i += 4;
        } else {
            decoded.push(encoded[i]);
            i = i + 1;
        }
    }

    decoded
}

fn number_to_rle(encoded: &mut Vec<u8>, char_code: u8, val: u16) {
    if val <= ONE_TRYTE_MAX {
        encoded.push(49);
        encoded.push(if val == 0 { 57 } else { val as u8 + 64 });
    } else if val <= TWO_TRYTE_MAX {
        let val1 = val % 27;
        let val2 = (val - val1) / 27;
        encoded.push(50);
        encoded.push(if val1 == 0 { 57 } else { val1 as u8 + 64 });
        encoded.push(if val2 == 0 { 57 } else { val2 as u8 + 64 });
    } else {
        let val1 = val % 27;
        let val2 = ((val - val1) / 27) % 27;
        let val3 = (val - (val2 * 27) - val1) / (27 * 27);
        encoded.push(51);
        encoded.push(if val1 == 0 { 57 } else { val1 as u8 + 64 });
        encoded.push(if val2 == 0 { 57 } else { val2 as u8 + 64 });
        encoded.push(if val3 == 0 { 57 } else { val3 as u8 + 64 });
    }
    encoded.push(char_code);
}

fn rle_to_number(decoded: &mut Vec<u8>, char_code: u8, t1: u8, t2: u8, t3: u8) {
    let mut val: u16 = if t1 == 57 { 0 } else { t1 as u16 - 64 };
    if t2 != 0 {
        val += (if t2 == 57 { 0 } else { t2 - 64 }) as u16 * 27;
    }
    if t3 != 0 {
        val += (if t3 == 57 { 0 } else { t3 - 64 }) as u16 * 27 * 27;
    }
    for _i in 0..val {
        decoded.push(char_code);
    }
}