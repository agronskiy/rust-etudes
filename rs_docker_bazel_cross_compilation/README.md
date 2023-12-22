## Cross compiling Rust from MacOS

#### What it is

This is an examplar repository showing:
- how to compile a rust binary using `rules_rust`
- package it into a Linux OCI container using `rules_oci` (think better Docker)
- all while the host system is Darwin, so the cross-compilation has to take place
  - to that end, a fully hermetic `hermetic_cc_toolchain` ruleset is used, even up to the 
    point where the preserved `glibc` version is used

There are plenty of beautiful resources on Rust cross-compilation. To my knowledge, this approach 
has the minimal Starlark overhead. To read a full story, please read the blog post (TBA).

#### Running 

To run it, make sure you have Bazel installed (e.g. [here](https://bazel.build/install)) and execute the following  

```console
> bazel run //main_bin:main_bin_tarball

> docker run --rm -it main_bin/main_bin
```


