load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index//:defs.bzl", "all_crate_deps")

# This is the second option
rust_binary(
    name = "binary_two",
    srcs = ["main.rs"],
    deps = all_crate_deps(
        package_name = "main_bin_crate",
    ),
)
