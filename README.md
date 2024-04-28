# quine_mcclusky

This is a Rust implementation of the Quine-McClusky algorithm for minimizing Boolean functions.

[Quine McClusky Algorithm](https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm)

[Petrick's Method](https://en.wikipedia.org/wiki/Petrick%27s_method)

## Example

```txt
Enter the minimum terms, separated by spaces: 1 5 6 7 11 12 13 15
Enter don't care terms (if any), separated by spaces: 
Variables: 4
Minterms: ["0001", "0101", "0110", "0111", "1011", "1100", "1101", "1111"]
Don't care terms: []
Merge 1
        Prime implicants: []
        Merged terms: ["110-", "-101", "0-01", "1-11", "01-1", "-111", "11-1", "011-"]
Merge 2
        Prime implicants: ["110-", "0-01", "1-11", "011-"]
        Merged terms: ["-1-1"]
Merge 3
        Prime implicants: ["-1-1"]
        Merged terms: []
Prime Implicants: [
    "110-",
    "0-01",
    "1-11",
    "011-",
    "-1-1",
]
Minimum cost: G = 17
Best solution(s):
Res = ABC' + A'C'D + ACD + A'BC
```