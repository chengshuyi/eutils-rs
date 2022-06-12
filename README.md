# eutils-rs
some useful ebpf utils tools.





### timestamp

Mainly used to obtain realtime or monotonic time. And get the difference between the two, this difference eliminates the effect of system call delay and is more accurate.

See `examples/timestamp.rs` for more information.


### latency distribution

We often obtain delay information, such as scheduling delay, network delay, etc., through the ebpf program. `DelayDistribution` module provides the delay distribution function, which is convenient for us to quickly display the delay information.

See `examples/delay_distribution.rs` for more information.