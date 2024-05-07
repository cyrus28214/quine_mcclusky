# quine_mcclusky

This is a Rust implementation of the Quine-McClusky algorithm and Petrick's method for optimizing Boolean functions.

[Quine McClusky Algorithm](https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm)

[Petrick's Method](https://en.wikipedia.org/wiki/Petrick%27s_method)

## Example

```txt
Enter the minimum terms, separated by spaces: 1 6 11 12 13 15
Enter don't care terms (if any), separated by spaces: 5 7
Variables: 4
Minterms: ["0001", "0110", "1011", "1100", "1101", "1111"]
Don't care terms: ["0101", "0111"]
Merge 1
        Prime implicants: []
        Merged terms: ["01-1", "110-", "0-01", "011-", "11-1", "-111", "-101", "1-11"]
Merge 2
        Prime implicants: ["110-", "0-01", "011-", "1-11"]
        Merged terms: ["-1-1"]
Merge 3
        Prime implicants: ["-1-1"]
        Merged terms: []
Prime Implicants: [
    "110-",
    "0-01",
    "011-",
    "1-11",
    "-1-1",
]
Minimum cost: G = 16
Best solution(s):
Res = ABC' + A'C'D + A'BC + ACD
```