# Advent of Code 2024, day 19

Towels: r, wr, b, g, bwu, rb, gb, br
Pattern: rrbgbr

- _
  - _
- r
  - r (r: reach back by 1)
- rr
  - r r (r: reach back by 1)
- rrb
  - r r b (b: reach back by 1)
  - r rb (rb: reach back by 2)
- rrbg
  - r r b g
  - r rb g
- rrbgb
  - r r b g b
  - r rb g b
  - r r b gb (gb: reach back by 2)
  - r rb gb
- rrbgbr
  - r r b g b r (r: reach back by 1)
  - r rb g b r
  - r r b gb r
  - r rb gb r
  - r r b g br (br: reach back by 2)
  - r rb g br

| _           | r | r | b | g | b | r |
|-------------|---|---|---|---|---|---|
| 1 (implied) | 1 | 1 | 2 | 2 | 4 | 6 |

```rust
        towels.iter()
            .map(|towel| towel.as_ref())
            .filter(|towel| pattern.starts_with(towel))
            .for_each(|towel| paths[towel.len() - 1] = 1);
```
