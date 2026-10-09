//! Net heap growth while each kind of asset body is being loaded (iOS memory hunt).
//!
//! `begin()` notes this thread's net allocated bytes; `end(kind, mark)` adds the
//! difference to that kind's total. Only allocations made by the calling thread are
//! counted, so parallel loads on other threads do not leak in. It is a net figure:
//! an asset that frees a buffer another thread allocated earlier shows less, and
//! nested assets are counted by every enclosing kind as well as their own.
//! Read the numbers as a ranking of where the heap grows, not as exact sizes.
//! Zone decompression buffers are allocated outside any asset body and do not appear.
//!
//! The per-thread counter lives in the global allocator and is always on (it does not
//! depend on `IW4L_COUNTING_ALLOC`); the launcher installs that allocator on every platform.
use std::sync::Mutex;

use crate::alloc_count;

#[derive(Clone, Copy)]
struct Row {
    kind: &'static str,
    count: u64,
    net: i64,
}

static ROWS: Mutex<Vec<Row>> = Mutex::new(Vec::new());

/// Opaque marker taken at the start of an asset body.
#[derive(Clone, Copy)]
pub struct Mark(i64);

pub fn begin() -> Mark {
    Mark(alloc_count::thread_net_bytes())
}

pub fn end(kind: &'static str, mark: Mark) {
    let grown = alloc_count::thread_net_bytes() - mark.0;
    let Ok(mut rows) = ROWS.lock() else { return };
    match rows.iter_mut().find(|row| row.kind == kind) {
        Some(row) => {
            row.count += 1;
            row.net += grown;
        }
        None => rows.push(Row { kind, count: 1, net: grown }),
    }
}

/// One line, largest net growth first, empty when nothing was recorded.
pub fn report() -> String {
    let Ok(rows) = ROWS.lock() else { return String::new() };
    if rows.is_empty() {
        return String::new();
    }
    let mut sorted: Vec<Row> = rows.clone();
    sorted.sort_by(|a, b| b.net.cmp(&a.net));
    let mib = |bytes: i64| bytes / (1024 * 1024);
    let parts: Vec<String> = sorted
        .iter()
        .take(10)
        .map(|row| format!("{} {}x {}MiB", row.kind, row.count, mib(row.net)))
        .collect();
    format!(
        "heap net growth per asset kind (top 10, per-thread net, nested assets counted in each enclosing kind): {}",
        parts.join(" | ")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_ranks_by_net_growth() {
        ROWS.lock().unwrap().clear();
        let mark = Mark(0);
        {
            let mut rows = ROWS.lock().unwrap();
            rows.push(Row { kind: "small", count: 2, net: 10 << 20 });
            rows.push(Row { kind: "big", count: 1, net: 300 << 20 });
        }
        let line = report();
        assert!(line.find("big").unwrap() < line.find("small").unwrap());
        assert!(line.contains("big 1x 300MiB"));
        let _ = mark;
        ROWS.lock().unwrap().clear();
        assert!(report().is_empty());
    }
}
