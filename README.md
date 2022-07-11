# Split PDF

A simple wrapper around GhostScript to split a pdf file into odd and even pages.

Some printers don't support printing on both sides of paper, especially on macOS.
A common workaround is to print all even pages, place pages again in paper tray, then print the odd pages.

## Requirements
- Unix like environment (Linux, macOS) for `rm` command.
- GhostScript installed and available in path as `gs` command.

---

## Usage:

Let's split a pdf file names `input.pdf`

```
cargo run --release -- input.pdf 
```
This will create two new pdf files `input_even.pdf` (containing all even pages) and `input_odd.pdf` (containing all odd pages).


Also supports selecting a range of pages using a start page and end page, both inclusive.
```
cargo run --release -- input.pdf -s 20 -e 30
```
This will create two new pdf files `input_inter_even.pdf` (containing all even pages) and `input_inter_odd.pdf` (containing all odd pages).
