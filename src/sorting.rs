pub fn encode_sorting_order(order: [u8; 4]) -> u8 {
    // 000aabbc

    let c = if order[0] > order[1] { 1 } else { 0 };

    let aa = order[3];

    let bb = order[2];

    aa << 3 | bb << 1 | c
}

pub fn decode_sorting_order(order: u8) -> [u8; 4] {
    // 000aabbc
    let max = order >> 3;
    let min = (order >> 1) & 0b11;

    // get the numbers in between.
    let (a, b) = if min > max {
        remaining(max, min)
    } else {
        remaining(min, max)
    };

    let c = (order & 1) == 1;

    if c {
        // a and b need to switch
        [min, b, a, max]
    } else {
        [min, a, b, max]
    }
}

/// remaining 2 numbers out of 0..=3, sorted
/// expects a<b, a!=b, a,b \in 0..=3
fn remaining(a: u8, b: u8) -> (u8, u8) {
    match (a, b) {
        (0, 1) => (2, 3),
        (0, 2) => (1, 3),
        (0, 3) => (1, 2),

        (1, 2) => (0, 3),
        (1, 3) => (0, 2),

        (2, 3) => (0, 1),
        _ => unreachable!("error in implementation. a:{a}, b:{b}"),
    }
}

/// Retuns the proper sorting order and the numbers, sorted.
pub fn sorting_order(nums: [u32; 4]) -> ([u8; 4], [u32; 4]) {
    let (mut mindex, mut maxdex) = if nums[0] < nums[1] { (0, 1) } else { (1, 0) };

    for i in 2..=3 {
        if nums[i] < nums[mindex] {
            mindex = i;
        }
        // since both numbers are the same to begin with
        // it's safe to only ever adjust one.
        // also, we need this else clause to be sure that
        // mindex != maxdex
        else if nums[i] > nums[maxdex] {
            maxdex = i;
        }
    }

    // binary encoding if numbers for faster matching
    let index = mindex << 2 | maxdex;
    // we have 4 * 3 options for min and maxdex

    // We care a lot about performace,
    // so we're building a static branching table here.
    let res: [u8; 4] = match index {
        0b01_00 => {
            if nums[2] < nums[3] {
                [1, 2, 3, 0]
            } else {
                [1, 3, 2, 0]
            }
        }
        0b10_00 => {
            if nums[1] < nums[3] {
                [2, 1, 3, 0]
            } else {
                [2, 3, 1, 0]
            }
        }
        0b11_00 => {
            if nums[1] < nums[2] {
                [3, 1, 2, 0]
            } else {
                [3, 2, 1, 0]
            }
        }
        0b00_01 => {
            if nums[2] < nums[3] {
                [0, 2, 3, 1]
            } else {
                [0, 3, 2, 1]
            }
        }
        0b10_01 => {
            if nums[0] < nums[3] {
                [2, 0, 3, 1]
            } else {
                [2, 3, 0, 1]
            }
        }
        0b11_01 => {
            if nums[0] < nums[2] {
                [3, 0, 2, 1]
            } else {
                [3, 2, 0, 1]
            }
        }

        0b00_10 => {
            if nums[1] < nums[3] {
                [0, 1, 3, 2]
            } else {
                [0, 3, 1, 2]
            }
        }
        0b01_10 => {
            if nums[0] < nums[3] {
                [1, 0, 3, 2]
            } else {
                [1, 3, 0, 2]
            }
        }
        0b11_10 => {
            if nums[0] < nums[1] {
                [3, 0, 1, 2]
            } else {
                [3, 1, 0, 2]
            }
        }

        0b00_11 => {
            if nums[1] < nums[2] {
                [0, 1, 2, 3]
            } else {
                [0, 2, 1, 3]
            }
        }
        0b01_11 => {
            if nums[0] < nums[2] {
                [1, 0, 2, 3]
            } else {
                [1, 2, 0, 3]
            }
        }
        0b10_11 => {
            if nums[0] < nums[1] {
                [2, 0, 1, 3]
            } else {
                [2, 1, 0, 3]
            }
        }
        x => unreachable!("these bit patterns can be proven to not exist: 0b{x:b}"),
    };

    // TODO verify bounds checks get optimized out
    let nums = res.map(|i| nums[i as usize]);

    (res, nums)
}
