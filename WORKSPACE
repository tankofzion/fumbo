load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")

http_archive(
    name = "io_bazel_rules_go",
    sha256 = "69de5c704a05ff37862f7e0f5534d4f479418afc21806c887db544a316f3cb6b",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
    ],
)

# http_archive(
#     name = "bazel_gazelle",
#     sha256 = "62ca106be173579c0a167deb23358fdfe71ffa1e4cfdddf5582af26520f1c66f",
#     urls = [
#         "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
#         "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
#     ],
# )

# load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

# go_rules_dependencies()

#go_register_toolchains(version = "1.16.2")

#load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")
#load("//:tooling/bazel/repositories.bzl", "go_repositories")

# gazelle:repository_macro third_party/repositories.bzl%go_repositories
#go_repositories()

#gazelle_dependencies()

#http_archive(
#    name = "com_google_protobuf",
#    sha256 = "1c744a6a1f2c901e68c5521bc275e22bdc66256eeb605c2781923365b7087e5f",
#    strip_prefix = "protobuf-3.13.0",
#    urls = ["https://github.com/protocolbuffers/protobuf/archive/v3.13.0.zip"],
#)

#load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

#protobuf_deps()

http_archive(
    name = "com_github_bazelbuild_buildtools",
    sha256 = "f5b666935a827bc2b6e2ca86ea56c796d47f2821c2ff30452d270e51c2a49708",
    strip_prefix = "buildtools-3.5.0",
    url = "https://github.com/bazelbuild/buildtools/archive/3.5.0.zip",
)

#git_repository(
#    name = "com_github_aignas_rules_shellcheck",
#    commit = "3497c019704c295f0cb2ea9b4dcd3d0be73703ef",
#    remote = "https://github.com/aignas/rules_shellcheck.git",
#)

#load("@com_github_aignas_rules_shellcheck//:deps.bzl", "shellcheck_dependencies")

#shellcheck_dependencies()

# ----------------------------------------------------------------------------------------------------------------------
# Cargo raze rules
# ----------------------------------------------------------------------------------------------------------------------

http_archive(
    name = "cargo_raze",
    sha256 = "c664e258ea79e7e4ec2f2b57bca8b1c37f11c8d5748e02b8224810da969eb681",
    strip_prefix = "cargo-raze-0.11.0",
    url = "https://github.com/google/cargo-raze/archive/v0.11.0.tar.gz",
)

load("@cargo_raze//:repositories.bzl", "cargo_raze_repositories")

cargo_raze_repositories()

load("@cargo_raze//:transitive_deps.bzl", "cargo_raze_transitive_deps")

cargo_raze_transitive_deps()

# ----------------------------------------------------------------------------------------------------------------------
# Rust rules
# ----------------------------------------------------------------------------------------------------------------------

git_repository(
    name = "rules_rust",
    commit = "96d5118f03411f80182fd45426e259eedf809d7a",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

http_archive(
    name = "bazel_skylib",
    sha256 = "97e70364e9249702246c0e9444bccdc4b847bed1eb03c5a3ece4f83dfe6abc44",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.0.2/bazel-skylib-1.0.2.tar.gz",
    ],
)

http_archive(
    name = "rules_pkg",
    sha256 = "aeca78988341a2ee1ba097641056d168320ecc51372ef7ff8e64b139516a4937",
    urls = [
        "https://github.com/bazelbuild/rules_pkg/releases/download/0.2.6-1/rules_pkg-0.2.6.tar.gz",
        "https://mirror.bazel.build/github.com/bazelbuild/rules_pkg/releases/download/0.2.6/rules_pkg-0.2.6.tar.gz",
    ],
)

load("@rules_pkg//:deps.bzl", "rules_pkg_dependencies")

rules_pkg_dependencies()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories( edition = "2021", version = "1.59.0" )

git_repository(
    name = "blackjack",
    commit = "38f41de67b95d10c1dcfa88a3717af1dd9786496",
    remote = "https://github.com/wildarch/blackjack.git",
)

load("@blackjack//:workspace.bzl", "blackjack_cargo")

blackjack_cargo()

#load("//src:cargo_dependencies.bzl", "cargo_dependencies")
load("//:language/modules/parser:cargo_dependencies.bzl", "cargo_dependencies")

cargo_dependencies()