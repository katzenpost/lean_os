# lean_os

Operating system calls for Lean 4 programs that Lean's standard library lacks, on Linux and
Android. The Rust side uses [lean-sys](https://github.com/katzenpost/lean-sys) for Lean's object
model.

- `LeanOs.fsync`: have what was written to a file or directory reach the disk, as `fsync(2)`
  does. `IO.FS.Handle.flush` only hands Lean's buffer to the kernel.

## Use

In your `lakefile`:

```toml
[[require]]
name = "lean_os"
git = "https://github.com/katzenpost/lean_os"
rev = "<commit>"
```

To replace a file whole, so that a crash or a power cut leaves the old one or the new one and
never part of either:

```lean
import LeanOs

def replace (path : System.FilePath) (bytes : ByteArray) : IO Unit := do
  let tmp := path.addExtension "tmp"
  IO.FS.writeBinFile tmp bytes
  LeanOs.fsync tmp
  IO.FS.rename tmp path
  LeanOs.fsync (path.parent.getD ".")
```

## Building and testing

Building needs Lean, the version in `lean-toolchain`, from
[elan](https://github.com/leanprover/elan), and Rust, from [rustup](https://rustup.rs). Lake
builds the crate in `rust/` with cargo.

```
lake build lean-os-test && lake exe lean-os-test
cd rust && cargo test
```

[Paw's Android build](https://github.com/katzenpost/paw#android) builds the crate for phones
with the rest of an app, as it does every package's `rust/`.
