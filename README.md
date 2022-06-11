# eutils-rs
some useful ebpf utils tools.





### timestamp

Mainly used to obtain realtime or monotonic time. And get the difference between the two, this difference eliminates the effect of system call delay and is more accurate.

See `examples/timestamp.rs` for more information.