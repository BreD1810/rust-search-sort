Sorting/Search Visualisation
===

This is a visualisation of various sorting and searching algorithms, written in Rust using the [Piston](https://www.piston.rs/) framework.
It is my first project with Rust!

This repo is inspired by [dmitmel's](https://github.com/dmitmel/sorting-visualization).

Compilation
---
Simply run:
```bash
git clone https://github.com/bred1810/rust-search-sort
cd rust-search-sort
cargo build --release
```

Usage
---
```bash
# Run bubble sort
cargo run -- --sort-algorithm bubble
# Run quicksort with 1000 elements
cargo run -- --sort-algorithm quick --length 1000
# Run binary search
cargo run -- --search-algorithm binary
# For more information, refer to the help page
cargo run -- --help
```

Sorting Algorithms
---
- Bubble sort
- Cocktail Sort
- Comb Sort
- Gnome Sort
- Heapsort
- Insertion Sort
- Merge Sort
- Quicksort
- Selection Sort
- Shellsort

Search Algorithms
---
- Binary Search
- Linear Search
