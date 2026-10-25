import Lake
open Lake DSL System

package lean_os

@[default_target]
lean_lib LeanOs

lean_exe «lean-os-test» where
  root := `LeanOs.Tests

/-- The Rust crate in `rust/`, built by cargo. -/
extern_lib lean_os pkg := Job.async do
  proc { cmd := "cargo", args := #["build", "--release", "--quiet"], cwd := pkg.dir / "rust" }
  let lib := pkg.dir / "rust" / "target" / "release" / nameToStaticLib "lean_os"
  setTrace (← computeTrace lib)
  return lib
