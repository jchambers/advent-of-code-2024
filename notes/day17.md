# Advent of Code 2024, day 17

Oh it's one of _those_ problems. We're going to need to analyze the program to find patterns rather than just brute-forcing all possible values of register A to find a quine.

My particular program (and I suspect lots of other people's) is: `2,4,1,2,7,5,4,3,0,3,1,7,5,5,3,0`. Let's make a first pass at breaking that down. As a reminder, the opcode-to-instruction map is:

| Opcode | Instruction |
|--------|-------------|
|      0 |       `ADV` |
|      1 |       `BXL` |
|      2 |       `BST` |
|      3 |       `JNZ` |
|      4 |       `BXC` |
|      5 |       `OUT` |
|      6 |       `BDV` |
|      7 |       `CDV` |

So doing direct substitution without interpreting the operands, we have:

```
BST 4
BXL 2
CDV 5
BXC 3
ADV 3
BXL 7
OUT 5
JNZ 0
```

The rules for "combo operands" are:

- Combo operands 0 through 3 represent literal values 0 through 3.
- Combo operand 4 represents the value of register A.
- Combo operand 5 represents the value of register B.
- Combo operand 6 represents the value of register C.
- Combo operand 7 is reserved and will not appear in valid programs.

…and turning the above into pseudocode, we get:

```
b = a % 8
b ^= 2
c = a >> b
b ^= c
a >>= 3
b ^= 7
out(b % 8)
if a != 0 { goto start }
```

…and turning that into Rust:

```rust
fn run_program(a: u64) -> Vec<u8> {
    let mut a = a;

    let mut out = Vec::new();

    while a != 0 {
        let mut b = (a % 8) ^ 2;
        let c = a >> b;
        b ^= c;
        b ^= 7;

        a >>= 3;

        out.push((b % 8) as u8);
    }

    out
}
```

Narrating this a bit, the core thing driving the loop is that we're grinding away at `a` 3 bits at a time. When `a` is 0, we exit the loop and the program terminates. Within the loop, we're doing some mild number crunching to get `b`, which we then yield on every iteration. `b` is ultimately a function of the lowest (at most) 11 bits of `a` (we can shift `a` by up to 8 bits to get `c`, then consider the lowest 3 bits of the result to get the final value of `b`).

We can "pick the lock" by working from the highest to lowest bits of `a` (or, conversely, the last digits of the program to the first digits). We know that the program has to terminate after the last digit, which means that all the bits "above" the bits that produce the last digit must be zero. That means if we initialize `a` to be 0, we can check each of the possible values for the 3 lowest bits, find one that gives us the output we want, then shift left and repeat.
