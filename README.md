# importunate

[<img alt="github" src="https://img.shields.io/badge/github-wainwrightmark/importunate-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/wainwrightmark/importunate)
[<img alt="crates.io" src="https://img.shields.io/crates/v/importunate.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/importunate)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/importunate/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/importunate)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/wainwrightmark/importunate/build/main?style=for-the-badge" height="22">](https://github.com/wainwrightmark/importunate/actions?query=branch%3Amain)

Apply and manipulate permutations of small, constant sized sets.

- `calculate` a `Permutation` and `apply` it later
- `combine` two `Permutation`s
- `invert` (undo) a `Permutation`


`no_std` by default.

The name of the crate is an anagram of 'permutation'.

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
importunate = "0.1.0"
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
