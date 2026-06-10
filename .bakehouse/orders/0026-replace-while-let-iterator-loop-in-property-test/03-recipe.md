# Recipe - Order #26 Replace while-let iterator loop in property test

Replace the manual iterator `while let` loop with a `for` loop over `line.chars()`.

Keep the `escaped` state machine unchanged so raw comma counting still ignores commas escaped by an odd backslash.
