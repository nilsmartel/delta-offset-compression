mod sorting;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

pub fn decode_4(data: &[u8]) -> (&[u8], [u32; 4]) {
    let byte = data[0];
    let order = byte & 0b11111;
    let order = sorting::decode_sorting_order(order);
    let offset = (byte >> 5) & 0b11;
    let offset = decode_offset(offset);

    let (rest, mut nums) = group_varint_encoding::decompress_4(&data[1..]);

    nums[0] += offset;
    for i in [1, 2, 3] {
        nums[i] += nums[i - 1];
    }

    // TODO check that no bound checks are inserted here. (e.g. they get optimized out)
    let nums = order.map(|i| nums[i as usize]);

    (rest, nums)
}

pub fn encode_4(buffer: &mut Vec<u8>, ns: [u32; 4]) {
    use group_varint_encoding::compress_block;

    let (block, order, offset) = delta_encode(ns);

    let byte = sorting::encode_sorting_order(order) | (offset << 5);

    buffer.push(byte);

    compress_block(buffer, block);
}

fn decode_offset(offset: u8) -> u32 {
    match offset {
        0 => 0,
        1 => 0xff,
        2 => 0xffff,
        3 => 0xffffff,
        _ => unreachable!("error in implementation"),
    }
}

fn best_offset(n: u32) -> u8 {
    for offset in [3, 2, 1] {
        if decode_offset(offset) <= n {
            return offset;
        }
    }
    0
}

// returns delta encoded numbers, sorting order and initial offset (pre encoded)
fn delta_encode(ns: [u32; 4]) -> ([u32; 4], [u8; 4], u8) {
    // sort numbers
    let (order, ns) = sorting::sorting_order(ns);
    // pick lowest number first.
    let first = ns[0];
    let offset = best_offset(first);
    let first_ = first - decode_offset(offset);

    // TODO SIMD here? (and then overwrite first)
    let mut res = [first_, 0, 0, 0];
    for i in [1, 2, 3] {
        // TODO ensure that this is unchecked.
        res[i] = ns[i] - ns[i - 1];
    }

    (res, order, offset)
}
