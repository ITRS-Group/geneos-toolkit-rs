# Measure - Order #26 Replace while-let iterator loop in property test

Clippy reported `clippy::while_let_on_iterator` for a property-test loop that consumed `line.chars()` through `while let Some(c) = chars.next()`.

The loop only needs sequential iteration while carrying an `escaped` state flag, so a `for c in line.chars()` loop preserves behavior and is clearer.
