# Quick Compression

Compression that works on just 4 numbers at once.
Employs nullsupression and delta-, offset encoding.

## Compression Scheme

The Scheme compressed 4 numbers at once.

One byte, always the second one will be used for classical 4 wise group varint encoding. e.g. 1 byte to store the length of 4 integers, performing null supression.
Also, we perform a custom deta encoding:
Initially the 4 numbers will be sorted.
Then the we store the byte sized offset (offset $ \in {0, 0xff, 0xffff, 0xffffff }$ ) to the first number.
For the remaining numbers $n+1$ we only encode the delta to number $n$.

The scheme consist of at least $(2+4)$, but less than $(2+16)$ bytes per group of four.



### Sorting Order Encoding
The sorting order can be represented as a permutation of {0, 1, 2, 3}.
We need 2 bits to store the first index.
We need
$ \lceil log_2(3) \rceil =2 $
bits to store the second index (can be only 3 out of 4)
We need 1 bit to encode the third number (only 2 options left).
We don't need to encode the last postion, this information is redundant.

encoding the order uses the 5 lower bits out a byte like this: `[000aabbc]`
where a encodes the first index and so forth.

### Storing initial offset

We have 4 bits left to store our offset.
That means we have 8 options for our offset.
The mapping of bits to offset is as follows:

$$ 000_b \mapsto 0_x $$
$$ 001_b \mapsto f_x $$
$$ 010_b \mapsto ff_x $$
$$ 011_b \mapsto fff_x $$
$$ 100_b \mapsto ffff_x $$
$$ 101_b \mapsto fffff_x $$
$$ 110_b \mapsto ffffff_x $$
$$ 111_b \mapsto fffffff_x $$


These bits xy are stored in the same byte as the sorting order, like this:  `0xyaabbc`

## Improvements

After performing delta encoding, the maximum length of the numbers is severly limited.

My hunch is that we never have to use 4 bytes for each number, because of the sorted delta encoding.
TODO: Prove this and prove the max. amount of bytes.


The second number encoding the sorting order has lower entropy than the others.


The last number I think I can use for a mode to encode 64 bits numbers potentially.
