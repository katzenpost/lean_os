/-
SPDX-FileCopyrightText: Copyright (C) 2026 David Stainton
SPDX-License-Identifier: AGPL-3.0-only
-/

import LeanOs

def check (name : String) (ok : Bool) : StateT Nat IO Unit := do
  IO.println s!"{if ok then "ok  " else "FAIL"} {name}"
  unless ok do modify (· + 1)

def tests : StateT Nat IO Unit := do
  let dir : System.FilePath := s!".lake/lean-os-test-{← IO.Process.getPID}"
  IO.FS.createDirAll dir
  let file := dir / "f"
  IO.FS.writeFile file "kept"
  check "fsync of a file" ((← (LeanOs.fsync file).toBaseIO) matches .ok ())
  check "fsync of a directory" ((← (LeanOs.fsync dir).toBaseIO) matches .ok ())
  let missing := dir / "missing"
  match ← (LeanOs.fsync missing).toBaseIO with
  | .error e@(.noFileOrDirectory ..) =>
    check "fsync of a missing file is the IO error for it, naming it" ((toString e).contains "missing")
  | _ => check "fsync of a missing file is the IO error for it, naming it" false
  IO.FS.removeDirAll dir

def main : IO UInt32 := do
  let ((), failed) ← tests.run 0
  return if failed == 0 then 0 else 1
