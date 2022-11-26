mod sorting;

#[cfg(test)]
mod tests {
    use rand::RngCore;

    use super::*;

    #[test]
    fn it_works() {
        let cases = [
            [0u32, 0, 0, 0],
            [7, 6, 4, 3],
            [0, 1, 2, 3],
            [123, 123, 123, 234],
            [255, 0, 0, 0],
            [255, 2, 3, 6],
        ];

        let mut buffer = Vec::new();
        for c in cases {
            encode_4(&mut buffer, c);
            let (rest, result) = decode_4(&buffer);
            assert_eq!(rest, b"", "no data remains after decoding");

            assert_eq!(result, c, "same numbers before and after decoding");
            buffer.clear();
        }
    }

    #[test]
    fn random_tests() {
        use rand::thread_rng;
        let mut rng = thread_rng();
        let cases = (0..1_000_000).map(|_| {
            [
                rng.next_u32(),
                rng.next_u32(),
                rng.next_u32(),
                rng.next_u32(),
            ]
        });

        // 4 numbers a 4 bytes
        let inputlen = 1_000_000 * 4 * 4;
        let mut outputlen = 0;

        let mut buffer = Vec::new();
        for c in cases {
            encode_4(&mut buffer, c);
            let (rest, result) = decode_4(&buffer);
            assert_eq!(rest, b"", "no data remains after decoding");

            assert_eq!(result, c, "same numbers before and after decoding");
            outputlen += buffer.len();
            buffer.clear();
        }

        println!("input length:  {inputlen} bytes");
        println!("output length: {outputlen} bytes");
        println!("compression rate (higher is better):");
        let compressionrate = inputlen as f64 / outputlen as f64;
        println!("{compressionrate}");
    }
}

/// Expects data to be encoded using quick compression.
/// Will panic otherwise.
/// Will encode 4 numbers and return the remaining input data.
pub fn decode_4(data: &[u8]) -> (&[u8], [u32; 4]) {
    let encoded_decoding_info = data[0];
    // the first 5 bits are the encoded order.
    let order = encoded_decoding_info & 0b11111;
    let offset = encoded_decoding_info >> 5;
    let offset = decode_offset(offset);

    let (rest, mut nums) = group_varint_encoding::decompress_4(&data[1..]);

    // resolve delta encoding
    // afterwards all numbers should be sorted.
    nums[0] += offset;
    for i in [1, 2, 3] {
        nums[i] += nums[i - 1];
    }

    // TODO we don't want to apply the order a second time
    // we want the inverse

    let order = sorting::inverse_encoding(order);
    let nums = sorting::apply_encoding(nums, order);

    (rest, nums)
}

/// Will push encoded numbers to the end of the buffer.
pub fn encode_4(buffer: &mut Vec<u8>, ns: [u32; 4]) {
    use group_varint_encoding::compress_block;

    let (block, order, offset) = delta_encode(ns);

    let encoded_decoding_info = sorting::encode_sorting_order(order) | (offset << 5);

    buffer.push(encoded_decoding_info);

    // let group varint encoding compress the remaining block for us.
    compress_block(buffer, block);
}

fn decode_offset(offset: u8) -> u32 {
    match offset {
        0b000 => 0x0,
        0b001 => 0xff,
        0b010 => 0xfff,
        0b011 => 0xffff,
        0b100 => 0xfffff,
        0b101 => 0xffffff,
        0b110 => 0xfffffff,
        0b111 => 0xffffffff,
        _ => unreachable!("error in implementation, offset: 0b{offset:b}"),
    }
}

// returns the best offset for a given number, encoded
fn best_offset(n: u32) -> u8 {
    for offset in (1..=0b111).rev() {
        // check that this offset is not greater than n itself.
        // and pick the first one for which it applies.
        // Works because we start checking the big ones.
        if decode_offset(offset) <= n {
            return offset;
        }
    }
    0
}

// returns delta encoded numbers, sorting order and initial offset (pre encoded)
fn delta_encode(nums: [u32; 4]) -> ([u32; 4], [u8; 4], u8) {
    // sort numbers
    let (order, nums) = sorting::sorting_order(nums);
    // pick lowest number first.
    let first = nums[0];
    let offset = best_offset(first);
    let first_ = first - decode_offset(offset);

    // TODO SIMD here? (and then overwrite first)

    // res are the values subtracted from one another
    // works fine because sorting order is preserved
    let mut res = [first_, 0, 0, 0];
    for i in [1, 2, 3] {
        // TODO ensure that subtraction is not checked
        res[i] = nums[i] - nums[i - 1];
    }

    (res, order, offset)
}
