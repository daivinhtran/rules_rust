load("@rules_rust//rust:defs.bzl", "rust_static_library")

rust_static_library(
    name = "librusty",
    srcs = ["librusty.rs"],
    hdrs = ["rusty.h"],
    crate_name = "rusty",
    includes = ["."],
    visibility = ["//visibility:public"],
)
