use chrono;
use eutils_rs::timestamp;

fn main() {
    let ns = timestamp::current_monotime();
    let mono = chrono::NaiveDateTime::from_timestamp(
        (ns / 1000_000_000) as i64,
        (ns % 1000_000_000) as u32,
    );
    let ns = timestamp::current_realtime();
    let real = chrono::NaiveDateTime::from_timestamp(
        (ns / 1000_000_000) as i64,
        (ns % 1000_000_000) as u32,
    );
    let ns = timestamp::delta_of_mono_real_time();
    let delta = chrono::NaiveDateTime::from_timestamp(
        (ns / 1000_000_000) as i64,
        (ns % 1000_000_000) as u32,
    );

    println!("monotonic time: {}", mono);
    println!("real      time: {}", real);
    println!("delta     time: {}", delta);
}
