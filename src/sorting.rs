#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    fn assert_sorted(nums: [u32; 4]) {
        nums.iter()
            .zip(&nums[1..])
            .for_each(|(n, n_1)| assert!(n <= n_1))
    }

    #[test]
    fn order_inversion() {
        // just gather some random test data
        let testdata = permut().into_iter().map(|x| x.map(|i| i as u32));
        let encodings = permut();

        for t in testdata {
            for encoding in encodings.iter().cloned() {
                let t1 = apply_encoding(t, encoding);
                let inverse_encoding = inverse_encoding(encoding);

                let result = apply_encoding(t1, inverse_encoding);

                assert_eq!(
                    result, t,
                    "expect inverse encoding applied to encoding to preduce original input value"
                );
            }
        }
    }

    #[test]
    fn sorting() {
        let input = permut().into_iter().map(|x| x.map(|i| i as u32));

        for nums in input {
            let (_encoding, result) = sorting_order(nums);
            // verify that the result is actually sorted
            assert_sorted(result);
        }
    }

    #[test]
    fn sorting_encoding() {
        let input = permut().into_iter().map(|x| x.map(|i| i as u32));

        for nums in input {
            let (encoding, _result) = sorting_order(nums);
            for i in [1, 2, 3] {
                assert!(
                    nums[encoding[i] as usize] >= nums[encoding[i - 1] as usize],
                    "expect encoding to be right"
                );
            }
        }
    }

    #[test]
    fn encoding_applied() {
        let input = permut().into_iter().map(|x| x.map(|i| i as u32));

        for nums in input {
            let (encoding, result) = sorting_order(nums);
            // verify that sorting encoding matches output
            let r1 = apply_encoding(nums, encoding);
            assert_eq!(
                r1, result,
                "expect to match {result:?} after order-encoding is applied"
            );
        }
    }

    fn permut() -> Vec<[u8; 4]> {
        // should be all, we need !4, so 4*3 = 12, 12*2 = 24, looks about right
        let e = vec![
            // 0 starting
            [0, 1, 2, 3],
            [0, 1, 3, 2],
            [0, 2, 1, 3],
            [0, 2, 3, 1],
            [0, 3, 1, 2],
            [0, 3, 2, 1],
            // 1 starting
            [1, 2, 3, 0],
            [1, 2, 0, 3],
            [1, 3, 2, 0],
            [1, 3, 0, 2],
            [1, 0, 2, 3],
            [1, 0, 3, 2],
            // 2 starting
            [2, 1, 0, 3],
            [2, 1, 3, 0],
            [2, 0, 1, 3],
            [2, 0, 3, 1],
            [2, 3, 1, 0],
            [2, 3, 0, 1],
            // 3 starting
            [3, 2, 1, 0],
            [3, 2, 0, 1],
            [3, 1, 2, 0],
            [3, 1, 0, 2],
            [3, 0, 2, 1],
            [3, 0, 1, 2],
        ];

        let permutations = 24;
        assert_eq!(e.len(), permutations, "expect {permutations} permutations");

        // i know this is dumb, but I have to work on my thesis and I'm so tired
        for i in 0..permutations {
            assert_eq!(
                e[i].iter().cloned().collect::<HashSet<_>>().len(),
                4,
                "expect 4 distinct elements in {i}th element of permutations"
            );

            for j in 0..permutations {
                if i == j {
                    continue;
                }

                assert_ne!(
                    e[i], e[j],
                    "expect no two elements of the vector to be the same"
                );
            }
        }

        e
    }

    #[test]
    fn order_encoding() {
        // first generate all permutations of 0,1,2,3
        let p = permut();

        for p1 in p {
            let e = encode_sorting_order(p1);

            let decoded = decode_sorting_order(e);

            assert_eq!(decoded, p1, "expect to match sorting order {p1:?}");
        }
    }
}
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
    let max2nd = (order >> 1) & 0b11;

    // get the numbers in between.
    let (a, b) = if max2nd > max {
        remaining(max, max2nd)
    } else {
        remaining(max2nd, max)
    };

    let c = (order & 1) == 1;

    if c {
        // a and b need to switch
        [b, a, max2nd, max]
    } else {
        [a, b, max2nd, max]
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

    let nums = apply_encoding(nums, res);

    (res, nums)
}

pub(crate) fn apply_encoding(input: [u32; 4], encoding: [u8; 4]) -> [u32; 4] {
    // TODO check that no bound checks are inserted here. (e.g. they get optimized out)
    encoding.map(|i|
        // get element of input at position i
        input[i as usize])
}

pub(crate) fn inverse_encoding(encoding: [u8; 4]) -> [u8; 4] {
    todo!("implement algorithm to reverse encoding")
}
