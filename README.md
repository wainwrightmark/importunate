# importunate

[<img alt="github" src="https://img.shields.io/badge/github-wainwrightmark/importunate-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/wainwrightmark/importunate)
[<img alt="crates.io" src="https://img.shields.io/crates/v/importunate.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/importunate)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/importunate/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/importunate)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/wainwrightmark/importunate/build/main?style=for-the-badge" height="22">](https://github.com/wainwrightmark/importunate/actions?query=branch%3Amain)

Apply and manipulate permutations of small, const sized, distinct sets.

- `calculate` a `Permutation` and `apply` it later
- Get `element_at_index` or `index_of_element`
- `combine` two `Permutation`s, including the built in ones: `reverse`, `rotate_right` and `rotate_left`
- `invert` (undo) a `Permutation`
- convert `to_le_byte_array` or `try_from_le_byte_array` to store in as few bytes as mathematically possible



`no_std` by default. Features for `serde` and `arbitrary`

The name of the crate is an anagram of 'permutation'.

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
importunate = "0.1.1"
```

## Getting started

```rust
use importunate::*;

fn main() {
    let arr1 = [2,0,1,3];
    let mut arr2 = ["zero", "one", "two", "three"];
    let perm = Permutation::<u8,4>::calculate_unchecked(&arr1, |&x|x);
    perm.apply(arr2);

    assert_eq!(arr2,["two","zero", "one",  "three"] );
}
```

`Permutation`s take two generic parameters, the `Inner` type (an unsigned integer) and the number of `Elements`.

The number of elements must not be greater than the maximum allowed for that inner type.
The following table shows how many elements can fit into each type as well as the minimum number of bytes needed.

|Max Elements|Bytes|Type  |
|:----------:|:---:|:----:|
|     5      |  1  | `u8` |
|     8      |  2  |`u16` |
|     10     |  3  |`u32` |
|     12     |  4  |      |
|     14     |  5  |`u64` |
|     16     |  6  |      |
|     18     |  7  |      |
|     20     |  8  |      |
|     22     |  9  |`u128`|
|     24     | 10  |      |
|     25     | 11  |      |
|     27     | 12  |      |
|     29     | 13  |      |
|     30     | 14  |      |
|     32     | 15  |      |
|     34     | 16  |      |

There are three different methods for calculating a permutation:

`calculate_incomplete` will calculate the permuation for any array whose elements implement `Ord` but it is comparatively slow. It will even work if the array contains duplicate elements but keep in mind that permuations describing such arrays will not be unique.

`try_calculate` and `calculate_unchecked` both expect arrays of elements and functions mapping those elements to `u8`. Every element should map to a different `u8` in the range `0..ELEMENTS`. If this condition is not met, `try_calculate` will return `None` and `calculate_unchecked` will panic or loop forever. *Do not use it on user input*




## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/wainwrightmark/importunate/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`importunate` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

- [Mark Wainwright](https://github.com/wainwrightmark)
