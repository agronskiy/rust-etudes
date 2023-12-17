load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index//:defs.bzl", "all_crate_deps")

rust_binary(
    name = "main_bin_crate",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(),
)

# This is the first option to define another binary
rust_binary(
    name = "binary_one",
    srcs = ["src/bin/binary_one/main.rs"],
    deps = all_crate_deps(),
)
