load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index//:defs.bzl", "aliases", "all_crate_deps")

rust_binary(
    name = "another_bin",
    # Specifies the source file for the binary.
    srcs = ["main.rs"],
    deps = all_crate_deps(
        package_name = "rs_yaml_parser_with_test",
    ),
)
