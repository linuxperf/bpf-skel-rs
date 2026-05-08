# Codex Project Guide

## Project Overview

- Project name: `bpf-skel-rs`
- Goal: provide Rust-facing libbpf skeleton modules generated from third-party
  eBPF programs.
- Current shape: this repository is a Cargo workspace. The publishable crate
  lives in `crates/bpf-skel` and ships pre-generated skeleton sources under
  `crates/bpf-skel/src/skel`. Generation tooling lives under `xtask`.
- Generation flow: `xtask` scans direct child directories under `xtask/bpf/`,
  invokes each workspace Makefile, reads `xtask/bpf/<name>/.output/*.bpf.o`,
  and regenerates Rust skeleton modules in `crates/bpf-skel/src/skel`.
- Default BPF workspace: `xtask/bpf/bcc`, which delegates to BCC
  `libbpf-tools` under the `xtask/third_party/bcc` git submodule.

## Language and Character Constraints

- All text must be written in English
- Only use ASCII characters

## Code Style

- Prioritize human readability above all else

## Working Style

- Read the existing code and README before editing files.
- Prefer the repository's existing style. When there is no local pattern, use
  conventional Rust layout and naming.
- Keep changes focused. Do not perform unrelated refactors.
- Do not overwrite uncommitted user changes. Check `git status --short` before
  editing.
- If adding dependencies, explain why and prefer actively maintained,
  purpose-specific crates.

## Expected Repository Layout

The repository uses this layout:

```text
.
├── Cargo.toml
├── crates/
│   └── bpf-skel/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── skel/
├── xtask/
│   ├── Cargo.toml
│   ├── src/main.rs
│   ├── bpf/
│   │   └── bcc/
│   │       └── Makefile
│   └── third_party/
│       ├── bcc/
│       │   └── libbpf-tools/
│       │       └── *.bpf.c
│       └── <repo>/
│           └── ...
└── .github/
    └── workflows/
        └── publish.yml
```

`xtask/third_party/bcc` is the default BCC git submodule. Do not modify vendor
sources directly. Prefer adaptations in `xtask`, environment variables, or
upstream patches.

## Common Commands

Common checks:

```bash
cargo fmt --all
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

Regenerate skeletons:

```bash
cargo run -p xtask -- gen-skel
```

Check package contents and verify the publishable crate:

```bash
cargo package -p bpf-skel --list
cargo package -p bpf-skel
```

When eBPF compilation is involved, the machine usually also needs:

```bash
clang --version
llvm-strip --version
make --version
bpftool version
```

After cloning, initialize missing submodules with:

```bash
git submodule update --init --recursive
```

If a command depends on the local kernel, root privileges, BTF, or network
downloads, clearly state what was actually verified and what was not.

## Rust Conventions

- Use stable Rust unless an eBPF-related capability truly requires nightly.
- Public APIs should have clear error semantics. Avoid `unwrap()` and
  `expect()` in library logic.
- Prefer a project-level error enum when one exists. Examples and tests may be
  simpler.
- Keep unsafe code tightly scoped and document nearby safety assumptions.
- Be careful with cross-kernel behavior. Do not treat one local kernel's
  behavior as universal.
- Prefer well-known third-party crates where they fit the task.

## eBPF Conventions

- BPF objects are produced by `xtask/bpf/<name>/Makefile` in that workspace's
  local `.output` directory.
- Do not commit `.bpf.o` files or `.output` directories.
- `crates/bpf-skel/src/skel` is generated and ignored by default, but release
  commits must force-add refreshed skeleton sources so GitHub Actions can
  publish them:

```bash
git add -f crates/bpf-skel/src/skel
```

- Add clear comments for map, program, attach point, and kernel-version
  requirements where applicable.
- Tests should not require root by default. Privileged tests should be isolated
  or explicitly marked.

## Before Commit

Try to run:

```bash
cargo fmt --all
cargo check --workspace --locked
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo package -p bpf-skel --list
```

If the local machine lacks `xtask/third_party/bcc`, the eBPF toolchain, or
network access for dependency downloads, mention that in the final result.
