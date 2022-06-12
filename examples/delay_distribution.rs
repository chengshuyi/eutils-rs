use eutils_rs::delay_distribution::DelayDistribution;

fn main() {
    let mut dd = DelayDistribution::new();

    for i in 0..100 {
        dd.insert(i);
    }

    // show distribution in ten categories
    dd.show(10);
    println!("\n");
    // show distribution in five categories
    dd.show(5);
}
