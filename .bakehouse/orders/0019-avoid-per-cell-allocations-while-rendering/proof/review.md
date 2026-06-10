# Review - Order #19

Reviewed scoped diff: `src/dataview.rs`.

Dataview cell storage is now nested by row then column, allowing render lookups by borrowed `&str` without allocating `(String, String)` keys per cell.

Decision: pass.
