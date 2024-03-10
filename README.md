# Minicel

The idea is to implement a batch program that can accept a CSV file that looks like this:

```csv
A      | B
1      | 2
3      | 4
=A1+B1 | =A2+B2
```

And outputs:

```csv
A      | B
1      | 2
3      | 4
3      | 7
```

Basically a simple Excel engine without any UI.

---

Based on the assignment from ts-coding stream, FYI: [https://github.com/tsoding/minicel](https://github.com/tsoding/minicel)

---

# Syntax

## Types of Cells


| Type       | Description                                                                                                        | Examples                          |
| ---        | ---                                                                                                                | ---                               |
| Text       | Just a human readiable text.                                                                                       | `A`, `B`, `C`, etc                |
| Number     | Anything that can be parsed as a double by [strtod][strtod]                                                                | `1`, `2.0`, `1e-6`, etc           |
| Expression | Always starts with `=`. Excel style math expression that involves numbers and other cells.                         | `=A1+B1`, `=69+420`, `=A1+69` etc |
| Clone      | Always starts with `:`. Clones a neighbor cell in a particular direction denoted by characters `<`, `>`, `v`, `^`. | `:<`, `:>`, `:v`, `:^`             |
