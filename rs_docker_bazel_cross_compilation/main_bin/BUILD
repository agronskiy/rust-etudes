load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_pkg//pkg:tar.bzl", "pkg_tar")
load("@rules_oci//oci:defs.bzl", "oci_image", "oci_tarball")
load("@crate_index//:defs.bzl", "all_crate_deps")
load(
    "@aspect_bazel_lib//lib:transitions.bzl",
    "platform_transition_binary",
)

rust_binary(
    name = "main_bin",
    srcs = ["src/main.rs"],
    deps = all_crate_deps(),
)

# Necessary to cross-compile into linux
platform_transition_binary(
    name = "main_bin_linux_x86",
    binary = ":main_bin",
    target_platform = "//bazel/platforms:linux_x86_64",
)

# Step 2: Compress it to layer using pkg_tar
pkg_tar(
    name = "main_bin_layer",
    # Note we use the cross-compiled binary from linux_x86_64, because the
    # base image is linux
    srcs = [":main_bin_linux_x86"],
)

# Step 3: Build image and add built layer to it
oci_image(
    name = "main_bin_image",
    base = "@distroless_cc",
    entrypoint = ["/main_bin"],
    tars = [":main_bin_layer"],
)

oci_tarball(
    name = "main_bin_tarball",
    image = ":main_bin_image",
    repo_tags = ["main_bin/main_bin:latest"],
)
