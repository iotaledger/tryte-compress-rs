use std::collections::HashMap;
#[macro_use] extern crate lazy_static;

lazy_static! {
    static ref HUFFMAN_TABLE: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('A', "0100");
        m.insert('B', "11110");
        m.insert('C', "0001");
        m.insert('D', "11111");
        m.insert('E', "11101");
        m.insert('F', "11100");
        m.insert('G', "10010");
        m.insert('H', "11000");
        m.insert('I', "10111");
        m.insert('J', "10000");
        m.insert('K', "10001");
        m.insert('L', "10011");
        m.insert('M', "00001");
        m.insert('N', "11001");
        m.insert('O', "10101");
        m.insert('P', "11011");
        m.insert('Q', "01010");
        m.insert('R', "11010");
        m.insert('S', "0010");
        m.insert('T', "10100");
        m.insert('U', "01011");
        m.insert('V', "01111");
        m.insert('W', "10110");
        m.insert('X', "01101");
        m.insert('Y', "01100");
        m.insert('Z', "01110");
        m.insert('9', "0011");
        m.insert('1', "000001");
        m.insert('2', "0000001");
        m.insert('3', "00000001");
        m.insert('_', "00000000");
        m
    };

    static ref HUFFMAN_TABLE_REVERSE: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("0100", 'A');
        m.insert("11110", 'B');
        m.insert("0001", 'C');
        m.insert("11111", 'D');
        m.insert("11101", 'E');
        m.insert("11100", 'F');
        m.insert("10010", 'G');
        m.insert("11000", 'H');
        m.insert("10111", 'I');
        m.insert("10000", 'J');
        m.insert("10001", 'K');
        m.insert("10011", 'L');
        m.insert("00001", 'M');
        m.insert("11001", 'N');
        m.insert("10101", 'O');
        m.insert("11011", 'P');
        m.insert("01010", 'Q');
        m.insert("11010", 'R');
        m.insert("0010", 'S');
        m.insert("10100", 'T');
        m.insert("01011", 'U');
        m.insert("01111", 'V');
        m.insert("10110", 'W');
        m.insert("01101", 'X');
        m.insert("01100", 'Y');
        m.insert("01110", 'Z');
        m.insert("0011", '9');
        m.insert("000001", '1');
        m.insert("0000001", '2');
        m.insert("00000001", '3');
        m.insert("00000000", '_');
        m
    };    
}

lazy_static! {
    static ref RLE_ALPHABET: Vec<char> = vec!['9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
}

const END_OF_DATA: char = '_';

const RUN_MIN_LENGTH: usize = 3;
const ONE_TRYTE_MAX: usize = 26;
const TWO_TRYTE_MAX: usize = 728;
const THREE_TRYTE_MAX: usize = 19682;

pub fn compress(trytes: &str) -> Vec<u8> {
    // First run length encode the data to reduce the content
    let mut rle = run_length_encode(trytes);

    // Add the end of data marker so the decompress knows
    // when to finish processing the data
    rle.push(END_OF_DATA);

    // Convert the rle encoded trytes to their huffman form
    let mut bytes: Vec<u8> = Vec::new();
    let mut encoded: String = String::new();
    for c in rle.chars() {
        encoded += HUFFMAN_TABLE[&c];

        while encoded.len() >= 8 {
            let val = &encoded[0..8];
            let res = u8::from_str_radix(&val, 2);
            bytes.push(res.unwrap());
            encoded = encoded[8..].to_string();
        }
    }

    // If there are any remaining bits make sure we don't miss them
    if encoded.len() > 0 {
        // Pad the remaining bits with 0
        let remaining = 8-encoded.len();
        for _i in 0 .. remaining {
            encoded += "0";
        }

        bytes.push(u8::from_str_radix(&encoded, 2).unwrap());
    }

    bytes
}

pub fn decompress(buffer: Vec<u8>) -> String {
    // Convert the huffman encoded data back to the full rle
    let mut decoded: String = String::new();
    let mut buffer_pos = 0;
    let mut found_end = false;
    let mut bit_index = 7;
    while !found_end {
        let mut key: String = String::new();

        // Build a key until we find it in the huffman table
        while !HUFFMAN_TABLE_REVERSE.contains_key(&*key) {
            // if bufferPos > buffer.len() {
            //     throw new Error("End of data reached while decompressing");
            // }

            // If we have run out of bits in the current buffer value
            // move to the next one
            if bit_index == -1 {
                buffer_pos = buffer_pos + 1;
                bit_index = 7;
            }

            // Add the next bit from the buffer to the key
            key.push(if (buffer[buffer_pos] >> bit_index) & 0x01 == 0x01 { '1' } else { '0' });
            bit_index = bit_index -1;
        }

        // A key was found, if it was end of data then
        // finished the decompress, otherwise just add it
        // to the decompressed data
        let val = HUFFMAN_TABLE_REVERSE[&*key];
        if val == END_OF_DATA {
            found_end = true;
        } else {
            decoded.push(val);
        }
    }

    // Run length decode the data
    return run_length_decode(decoded);
}


pub fn run_length_encode(trytes: &str) -> String {
    let mut chars = trytes.chars();

    let mut encoded: String = String::new();
    let mut prev = chars.next().unwrap();
    let mut count: usize = 1;

    for c in chars {
        if c != prev {
            encoded += &append_run(count, prev);
            count = 1;
            prev = c;
        } else {
            count = count + 1;
        }
    }

    encoded += &append_run(count, prev);

    encoded
}

pub fn append_run(count: usize, prev: char) -> String {
    let mut encoded: String = String::new();
    let mut remaining: usize = count;

    while remaining >= RUN_MIN_LENGTH {
        let current_run_length = if remaining > THREE_TRYTE_MAX { THREE_TRYTE_MAX } else { remaining };
        encoded.push_str(&number_to_rle(current_run_length));
        encoded.push(prev);
        remaining -= current_run_length;
    }

    if remaining > 0 {
        for _i in 0..remaining {
            encoded.push(prev);
        }
    }

    encoded
}

pub fn run_length_decode(encoded: String) -> String {
    let mut output: String = String::new();
    let mut it = encoded.chars();

    while let Some(c) = it.next() {
        if c == '1' {
            let rle_length_1 = it.next().unwrap();
            let ch = it.next().unwrap();
            let length = rle_to_number(rle_length_1, ' ', ' ');
            for _i in 0..length {
                output.push(ch);
            }
        } else if c == '2' {
            let rle_length_1 = it.next().unwrap();
            let rle_length_2 = it.next().unwrap();
            let ch = it.next().unwrap();
            let length = rle_to_number(rle_length_1, rle_length_2, ' ');
            for _i in 0..length {
                output.push(ch);
            }
        } else if c == '3' {
            let rle_length_1 = it.next().unwrap();
            let rle_length_2 = it.next().unwrap();
            let rle_length_3 = it.next().unwrap();
            let ch = it.next().unwrap();
            let length = rle_to_number(rle_length_1, rle_length_2, rle_length_3);
            for _i in 0..length {
                output.push(ch);
            }
        } else {
            output.push(c);
        }
    }

    return output;
}

pub fn number_to_rle(val: usize) -> String {
    let mut rle: String = String::new();
    if val <= ONE_TRYTE_MAX {
        rle.push('1');
        rle.push(RLE_ALPHABET[val]);
    } else if val <= TWO_TRYTE_MAX {
        let val1 = val % 27;
        let val2 = (val - val1) / 27;
        rle.push('2');
        rle.push(RLE_ALPHABET[val1]);
        rle.push(RLE_ALPHABET[val2]);
    } else {
        let val1 = val % 27;
        let val2 = ((val - val1) / 27) % 27;
        let val3 = (val - (val2 * 27) - val1) / (27 * 27);
        rle.push('3');
        rle.push(RLE_ALPHABET[val1]);
        rle.push(RLE_ALPHABET[val2]);
        rle.push(RLE_ALPHABET[val3]);
    }
    rle
}

pub fn rle_to_number(t1: char, t2: char, t3: char) -> usize {
    let mut it1 = RLE_ALPHABET.iter();
    let mut val = it1.position(|&c| c == t1).unwrap();
    if t2 != ' ' {
        let mut it2 = RLE_ALPHABET.iter();
        let v = it2.position(|&c| c == t2).unwrap();
        val += v * 27;
    }
    if t3 != ' ' {
        let mut it3 = RLE_ALPHABET.iter();
        let v = it3.position(|&c| c == t3).unwrap();
        val += v * 27 * 27;
    }
    return val;
}