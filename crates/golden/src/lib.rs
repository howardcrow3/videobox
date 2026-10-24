//! Golden-image tests for VideoBox's renderer (test-only crate; the library is empty).
//!
//! `tests/golden.rs` builds small procedural projects (no media files), renders one frame of each
//! through the CPU reference compositor (`videobox-render`) and compares it with a committed PNG
//! in `goldens/` using `videobox_testkit::golden`. The same scenes are composited on the GPU
//! (`videobox-gpu`) when an adapter is available and compared with the CPU result.
//!
//! Regenerate references with `VIDEOBOX_BLESS=1 cargo test -p videobox-golden`.
