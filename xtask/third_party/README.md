# third_party

BPF source repositories used by the generation task live here. BCC is vendored
as the default git submodule at `xtask/third_party/bcc`.

Initialize or refresh it with:

```bash
git submodule update --init --recursive
```

The `gen-skel` xtask uses configured `*.bpf.c` files as read-only input. It
invokes each workspace Makefile under `xtask/bpf/` to generate `.bpf.o` files
under that workspace's local `.output` directory, then generates Rust skeleton
modules from those objects into `src/skel`.

Do not patch third-party files in place for this crate; keep local adaptations
in Rust build configuration or upstream them.
