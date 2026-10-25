/-
SPDX-FileCopyrightText: Copyright (C) 2026 David Stainton
SPDX-License-Identifier: AGPL-3.0-only
-/

/-! # Operating system calls for Lean

What Lean's standard library lacks, implemented in `rust/src/lib.rs`. -/

namespace LeanOs

@[extern "lean_os_fsync"] private opaque fsyncPath (path : @& String) : IO Unit

/-- Have what was written to the file or directory at `path` reach the disk, as `fsync(2)` does.
`IO.FS.Handle.flush` only hands Lean's buffer to the kernel. To replace a file whole, write a
temporary file, `fsync` it, rename it over the old one and `fsync` the directory. -/
def fsync (path : System.FilePath) : IO Unit := fsyncPath path.toString

end LeanOs
