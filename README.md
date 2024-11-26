# Bounded Counter

Generic Incremental Bounded Counter

## Description

The `BoundedCounter` is a generic type that provides an incremental counter with a type specified upper bound.  
It increments every time it is iterated until it reaches the upper bound, at which point it resets back to 0.

## Usage

```rust
use bounded_counter::BoundedCounter;

fn main() {
    type Int = i32;
    const MOD: Int = Int::MAX / 100;

    for int in BoundedCounter::<Int>::default() {
        if int % MOD == 0 {
            println!("{:.0}%", int / MOD);
        }

        if int == Int::MAX {
            break;
        }
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
