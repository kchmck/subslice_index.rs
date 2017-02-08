# subslice\_index

[Documentation](https://docs.rs/subslice_index)

Get the index of a subslice into its parent slice.

## Example

```rust
use subslice_index::subslice_index;

let arr = [1, 2, 3, 4, 5, 6];

let slice = &arr[..];
let subslice = &slice[2..4];

assert_eq!(subslice, [3, 4]);
assert_eq!(subslice_index(slice, subslice), 2);
```
## Usage

This [crate](https://crates.io/crates/subslice_index) can be used through cargo by adding
it as a dependency in `Cargo.toml`:

```toml
[dependencies]
subslice_index = "0.5.0"
```
and importing it in the crate root:

```rust
extern crate subslice_index;
```
