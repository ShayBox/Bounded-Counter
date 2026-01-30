use bounded_counter::WrappedCounter;

type Type = u128;

const PERCENT_DECIMALS: usize = 27;
const PERCENT_SCALE: u128 = 10_u128.pow(PERCENT_DECIMALS as u32);
const UPDATES: Type = 100_000_000_000_000_000_000_000_000_000;
const STEPS: Type = Type::MAX / UPDATES;

fn main() {
    // let counter = BoundedCounter::<Type>::new(); // behind 'constructor' feature
    // let counter = BoundedCounter::<Type>::default(); // uses type default value
    let counter = WrappedCounter::<Type>(0);
    println!("Starting counter...");

    let mut tick: Type = 0;
    for count in counter {
        if count == 0 && tick != 0 {
            break;
        }

        if count % STEPS == 0 {
            let percent_scaled = (tick as u128) * 100 * PERCENT_SCALE / (UPDATES as u128);
            let percent_whole = percent_scaled / PERCENT_SCALE;
            let percent_frac = percent_scaled % PERCENT_SCALE;
            println!(
                "Progress: {}.{:0width$}%",
                percent_whole,
                percent_frac,
                width = PERCENT_DECIMALS
            );

            tick += 1;
        }
    }

    println!("Finished counting!");
}
