# importunate

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/importunate)
![Crates.io](https://img.shields.io/crates/v/importunate)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/wainwrightmark/importunate/build.yml)
![docs](https://img.shields.io/docsrs/importunate)

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

`calculate_incomplete` will calculate the permutation for any array whose elements implement `Ord` but it is comparatively slow. It will even work if the array contains duplicate elements but keep in mind that permuations describing such arrays will not be unique.

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
