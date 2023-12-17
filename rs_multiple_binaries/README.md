## `main_bin_crate`

#### What it is

This is an examplar repository showing two possibilities to imitate the multi-binary crates in Rust, using Bazel. 


#### Running 

To run it, make sure you have Bazel installed (e.g. [here](https://bazel.build/install)) and execute the following  

```console
> bazel run //main_bin_crate

> bazel run //main_bin_crate:binary_one

> bazel run //main_bin_crate/src/bin/binary_two
```


