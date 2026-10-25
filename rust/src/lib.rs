// SPDX-FileCopyrightText: Copyright (C) 2026 David Stainton
// SPDX-License-Identifier: AGPL-3.0-only

//! Operating system calls Lean's standard library lacks, as functions with Lean's calling
//! convention. `LeanOs.lean` declares them.

#![allow(clippy::missing_safety_doc)]

use lean_sys::*;
use std::fs::File;
use std::io;
use std::path::Path;

/// Have what was written to the file or directory at `path` reach the disk.
pub fn fsync(path: &Path) -> io::Result<()> {
    let f = File::open(path)?;
    loop {
        match f.sync_all() {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            r => return r,
        }
    }
}

unsafe fn string(s: &str) -> *mut lean_object {
    lean_mk_string_from_bytes(s.as_ptr(), s.len())
}

unsafe fn str_of<'a>(o: b_lean_obj_arg) -> &'a str {
    let bytes = std::slice::from_raw_parts(lean_string_cstr(o), lean_string_size(o) - 1);
    std::str::from_utf8_unchecked(bytes)
}

/// An error as Lean's `IO.Error`: from its errno, naming `path`, when it has one.
unsafe fn io_error(e: io::Error, path: &str) -> *mut lean_object {
    match e.raw_os_error() {
        Some(n) => {
            let f = string(path);
            let r = lean_decode_io_error(n, f);
            lean_dec_ref(f);
            r
        }
        None => lean_mk_io_user_error(string(&e.to_string())),
    }
}

unsafe fn io_unit(r: io::Result<()>, path: &str) -> *mut lean_object {
    match r {
        Ok(()) => lean_io_result_mk_ok(lean_box(0)),
        Err(e) => lean_io_result_mk_error(io_error(e, path)),
    }
}

#[no_mangle]
pub unsafe extern "C" fn lean_os_fsync(path: b_lean_obj_arg) -> *mut lean_object {
    let path = str_of(path);
    io_unit(fsync(Path::new(path)), path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn a_file_and_its_directory() {
        let dir = std::env::temp_dir().join(format!("lean_os-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("f");
        File::create(&file).unwrap().write_all(b"kept").unwrap();
        fsync(&file).unwrap();
        fsync(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_missing_file() {
        let e = fsync(Path::new("/nonexistent/lean_os")).unwrap_err();
        assert_eq!(e.kind(), io::ErrorKind::NotFound);
    }
}
