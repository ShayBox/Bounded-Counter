# Bounded Counter

Generic Incremental Bounded Counter

## Description

The `BoundedCounter` is a generic type that provides an incremental counter with a type specified upper bound.  
It increments every time it is iterated until it reaches the upper bound, at which point it resets back to 0.

If you need full-range wrapping (including signed types overflowing into negative values), use
`WrappedCounter`, which wraps on overflow instead of resetting to zero.

## Usage

```rust
use bounded_counter::WrappedCounter;

fn main() {
    // let counter = BoundedCounter::<Type>::new(); // behind 'constructor' feature
    // let counter = BoundedCounter::<Type>::default(); // uses type default value
    let counter = BoundedCounter::<i128>(0);

    for count in counter {
        println!("{count}")
    }
}
```

## Features

- `constructor`: Uses `derive_more` to derive a `new` constructor impl.
- `deref_mut`: Uses `derive_more` to derive a mutable deref impl.
- `deref`: Uses `derive_more` to derive a deref impl.

## Numbers

| Type | Max  |
|------|------|
| i8   | 127  |
| u8   | 255  |
| i16  | 32k  |
| u16  | 64k  |
| i32  | 2b   |
| u32  | 4b   |
| i64  | 9q   |
| u64  | 18q  |
| i128 | 170u |
| u128 | 340u |

* k = thousand
* b = billion
* q = quadrillion
* u = quintillion
