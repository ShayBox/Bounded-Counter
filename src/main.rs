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
