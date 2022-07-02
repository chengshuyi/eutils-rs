use std::fs;
use eutils_rs::proc::Pid;

fn main() {
    env_logger::init();
    // let mut pids = Vec::default();
    for entry in fs::read_dir("/proc").unwrap() {
        let name = entry.unwrap();

        if name.file_name().to_string_lossy().parse::<u32>().is_ok() {
            // pids.push(Pid::from_file(name.path()).unwrap());
            println!(
                "{:?}: {:?}",
                name.path(),
                Pid::from_file(name.path()).unwrap()
            );
        }
    }
}
