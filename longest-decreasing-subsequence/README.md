## Longest Decreasing Subsequence Problem

_The longest increasing subsequence problem is to find a subsequence of a given sequence in which the subsequence's elements are in sorted order, lowest to highest, and in which the subsequence is as long as possible. This subsequence is not necessarily contiguous, or unique._ — [Wikipedia](https://en.wikipedia.org/wiki/Longest_increasing_subsequence)

This is a simple implementation fo the described problem in the Rust Programming Language, which receives a sequence of integer values separated by comma.

For instance, the input `10,14,12,15,27,30,7,5,4,3,33,48,60,49,41,35,26,17` should output the subsequence `14,12,7,5,4,3`, which is the longest decreasing subsequence.

### How to run

First of all, you must have the [Rust](https://www.rust-lang.org) (and [Cargo](https://github.com/rust-lang/cargo)) installed on your machine.

So, you need to build the project to get the executable binary file. For this, run the `cargo build` command.

And now, you should execute the binary file passing the sequence value to be calculated. For this, you should run the `./target/debug/longest-decreasing-subsequence --sequence YOUR_SEQUENCE_VALUE` command, which `YOUR_SEQUENCE_VALUE` is your sequence value to be calculated.

For example, the command `./target/debug/longest-decreasing-subsequence --sequence 10,14,12,15,27,30,7,5,4,3,33,48,60,49,41,35,26,17` will print the subsequence `14,12,7,5,4,3`, as mentioned above.