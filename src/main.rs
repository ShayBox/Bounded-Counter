use bounded_counter::BoundedCounter;

fn main() {
    let counter = BoundedCounter::<i8>::default();

    for i in counter {
        println!("{i}");

        if i == i8::MAX {
            break;
        }
    }
}
