![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

![rust: 1.89+](https://img.shields.io/badge/rust-1.89+-orange.svg)
![no-std: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)
![wasm: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

[![codecov](https://codecov.io/gh/DuskSystems/duramen/graph/badge.svg)](https://codecov.io/gh/DuskSystems/duramen)

# `duramen`

A Cedar parser.

> [!WARNING]
> Not ready for use.

## Benchmarks

All benchmarks are ran on a MBP (aarch64-linux, 2021 Apple M1 Pro).

### Context

We use [`divan`](https://github.com/nvzqz/divan) for our benchmarks, taking the 'median' results from its output to create the following tables.

### Policy

#### decimal_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.84 µs |         150 |   28.99 KB |
| duramen         |     1.04 µs |           5 |      384 B |
| cedar (serde)   |    46.28 µs |         244 |   46.84 KB |
| duramen (serde) |    4.788 µs |         104 |   15.07 KB |
| duramen (facet) |    5.665 µs |          62 |   24.31 KB |
| cedar (prost)   |      105 µs |         172 |   33.58 KB |
| duramen (prost) |    2.665 µs |          31 |    5.45 KB |

#### decimal_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    45.15 µs |         174 |   35.51 KB |
| duramen         |    1.463 µs |           8 |      480 B |
| cedar (serde)   |    56.36 µs |         319 |   62.51 KB |
| duramen (serde) |    6.859 µs |         151 |   18.38 KB |
| duramen (facet) |    8.044 µs |          92 |   32.56 KB |
| cedar (prost)   |    47.78 µs |         207 |   40.84 KB |
| duramen (prost) |    3.936 µs |          48 |   6.318 KB |

#### example_1a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     29.3 µs |         120 |      24 KB |
| duramen         |    820.1 ns |           2 |      288 B |
| cedar (serde)   |    33.94 µs |         175 |   35.05 KB |
| duramen (serde) |    3.291 µs |          67 |   11.33 KB |
| duramen (facet) |    3.985 µs |          38 |   17.33 KB |
| cedar (prost)   |    30.57 µs |         136 |   28.12 KB |
| duramen (prost) |    1.866 µs |          20 |   3.929 KB |

#### example_2a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    31.87 µs |         125 |   24.32 KB |
| duramen         |    825.3 ns |           2 |      288 B |
| cedar (serde)   |    37.23 µs |         182 |   35.55 KB |
| duramen (serde) |    3.312 µs |          67 |   11.35 KB |
| duramen (facet) |    3.986 µs |          38 |   17.35 KB |
| cedar (prost)   |    33.77 µs |         141 |   28.46 KB |
| duramen (prost) |    1.888 µs |          20 |   3.953 KB |

#### example_2b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     31.5 µs |         125 |    24.3 KB |
| duramen         |    820.1 ns |           2 |      288 B |
| cedar (serde)   |    36.43 µs |         182 |   35.52 KB |
| duramen (serde) |    3.333 µs |          67 |   11.32 KB |
| duramen (facet) |    3.986 µs |          38 |   17.33 KB |
| cedar (prost)   |    33.19 µs |         141 |    28.4 KB |
| duramen (prost) |    1.887 µs |          20 |   3.917 KB |

#### example_2c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.26 µs |         148 |   31.19 KB |
| duramen         |    1.002 µs |           2 |      288 B |
| cedar (serde)   |    47.54 µs |         236 |    49.7 KB |
| duramen (serde) |    4.184 µs |          88 |   13.38 KB |
| duramen (facet) |    4.931 µs |          50 |   20.79 KB |
| cedar (prost)   |     40.7 µs |         170 |   35.63 KB |
| duramen (prost) |    2.542 µs |          30 |   5.531 KB |

#### example_3a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.13 µs |         120 |   22.31 KB |
| duramen         |    690.5 ns |           2 |      288 B |
| cedar (serde)   |    33.66 µs |         167 |   31.46 KB |
| duramen (serde) |    2.803 µs |          57 |   10.87 KB |
| duramen (facet) |    3.452 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.66 µs |         134 |   26.36 KB |
| duramen (prost) |    1.497 µs |          16 |   3.761 KB |

#### example_3b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.64 µs |         148 |   31.18 KB |
| duramen         |    1.002 µs |           2 |      288 B |
| cedar (serde)   |    46.74 µs |         236 |   49.69 KB |
| duramen (serde) |    4.101 µs |          88 |   13.38 KB |
| duramen (facet) |    4.931 µs |          50 |   20.79 KB |
| cedar (prost)   |    40.24 µs |         170 |   35.63 KB |
| duramen (prost) |    2.563 µs |          30 |   5.536 KB |

#### example_3c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    28.84 µs |         120 |   22.32 KB |
| duramen         |    685.3 ns |           2 |      288 B |
| cedar (serde)   |    33.33 µs |         167 |   31.48 KB |
| duramen (serde) |    2.782 µs |          57 |   10.87 KB |
| duramen (facet) |    3.493 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.53 µs |         134 |   26.37 KB |
| duramen (prost) |    1.507 µs |          16 |   3.761 KB |

#### example_4a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    48.81 µs |         181 |   34.04 KB |
| duramen         |    1.372 µs |           6 |      416 B |
| cedar (serde)   |    58.64 µs |         327 |   58.43 KB |
| duramen (serde) |    6.657 µs |         146 |   16.32 KB |
| duramen (facet) |    7.883 µs |          89 |   35.43 KB |
| cedar (prost)   |    50.77 µs |         217 |   39.39 KB |
| duramen (prost) |    3.442 µs |          44 |    6.35 KB |

#### example_4d

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    34.65 µs |         136 |   24.17 KB |
| duramen         |    1.061 µs |           5 |      384 B |
| cedar (serde)   |    40.87 µs |         235 |   38.65 KB |
| duramen (serde) |    4.947 µs |         113 |   14.59 KB |
| duramen (facet) |    6.155 µs |          71 |   30.73 KB |
| cedar (prost)   |    36.96 µs |         161 |   28.94 KB |
| duramen (prost) |    2.127 µs |          28 |   5.371 KB |

#### example_4e

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     36.4 µs |         138 |   24.44 KB |
| duramen         |    1.166 µs |           4 |      352 B |
| cedar (serde)   |    43.11 µs |         241 |   39.24 KB |
| duramen (serde) |    5.274 µs |         117 |   14.73 KB |
| duramen (facet) |    6.488 µs |          71 |   30.65 KB |
| cedar (prost)   |    38.78 µs |         165 |   29.21 KB |
| duramen (prost) |      2.5 µs |          30 |   5.391 KB |

#### example_4f

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    42.34 µs |         156 |   30.32 KB |
| duramen         |    1.442 µs |           8 |      480 B |
| cedar (serde)   |    51.75 µs |         298 |    53.1 KB |
| duramen (serde) |    6.981 µs |         154 |   16.59 KB |
| duramen (facet) |     8.29 µs |          98 |   39.81 KB |
| cedar (prost)   |    44.67 µs |         190 |   35.64 KB |
| duramen (prost) |    3.114 µs |          40 |   5.975 KB |

#### example_5b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    63.84 µs |         223 |   46.11 KB |
| duramen         |    2.431 µs |          16 |      736 B |
| cedar (serde)   |    84.28 µs |         521 |   93.76 KB |
| duramen (serde) |    13.88 µs |         308 |   26.24 KB |
| duramen (facet) |    16.38 µs |         195 |   74.65 KB |
| cedar (prost)   |     68.3 µs |         294 |    53.4 KB |
| duramen (prost) |    6.927 µs |          91 |   10.38 KB |

#### ip_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    36.64 µs |         149 |   27.71 KB |
| duramen         |      999 ns |           5 |      384 B |
| cedar (serde)   |     43.9 µs |         244 |   44.22 KB |
| duramen (serde) |    4.579 µs |         106 |   14.69 KB |
| duramen (facet) |    5.622 µs |          64 |   25.91 KB |
| cedar (prost)   |    38.74 µs |         172 |   32.29 KB |
| duramen (prost) |    2.499 µs |          31 |   5.462 KB |

#### ip_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    41.66 µs |         162 |   31.06 KB |
| duramen         |    1.328 µs |           8 |      480 B |
| cedar (serde)   |    50.77 µs |         287 |    53.6 KB |
| duramen (serde) |    6.207 µs |         139 |   16.79 KB |
| duramen (facet) |     7.22 µs |          86 |   33.31 KB |
| cedar (prost)   |    44.13 µs |         193 |   36.09 KB |
| duramen (prost) |    3.277 µs |          42 |    5.98 KB |

#### ip_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.59 µs |         153 |   29.11 KB |
| duramen         |     1.04 µs |           5 |      384 B |
| cedar (serde)   |    46.56 µs |         247 |   46.96 KB |
| duramen (serde) |    4.581 µs |         104 |   15.07 KB |
| duramen (facet) |    5.623 µs |          62 |   24.31 KB |
| cedar (prost)   |    40.61 µs |         175 |    33.7 KB |
| duramen (prost) |    2.624 µs |          31 |   5.446 KB |

#### multi_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    44.89 µs |         177 |   37.46 KB |
| duramen         |    1.542 µs |           2 |      288 B |
| cedar (serde)   |    56.62 µs |         308 |   63.14 KB |
| duramen (serde) |     6.67 µs |         135 |    15.5 KB |
| duramen (facet) |    7.817 µs |          73 |   27.39 KB |
| cedar (prost)   |    47.19 µs |         207 |   42.45 KB |
| duramen (prost) |    3.862 µs |          40 |    5.89 KB |

#### multi_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    40.08 µs |         161 |   31.19 KB |
| duramen         |    1.386 µs |           2 |      288 B |
| cedar (serde)   |    48.63 µs |         262 |    50.1 KB |
| duramen (serde) |    5.735 µs |         114 |   14.48 KB |
| duramen (facet) |    6.831 µs |          62 |      25 KB |
| cedar (prost)   |    41.97 µs |         185 |   35.95 KB |
| duramen (prost) |    3.207 µs |          31 |   5.304 KB |

#### multi_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    51.95 µs |         204 |   36.16 KB |
| duramen         |    1.777 µs |           6 |      416 B |
| cedar (serde)   |    64.82 µs |         380 |   63.93 KB |
| duramen (serde) |    8.854 µs |         192 |   20.36 KB |
| duramen (facet) |    10.56 µs |         113 |   45.11 KB |
| cedar (prost)   |    55.81 µs |         247 |    42.1 KB |
| duramen (prost) |    4.349 µs |          52 |   8.404 KB |

#### multi_4

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    70.19 µs |         279 |   49.52 KB |
| duramen         |     2.98 µs |          10 |      544 B |
| cedar (serde)   |    92.09 µs |         582 |   98.42 KB |
| duramen (serde) |    15.77 µs |         326 |    30.8 KB |
| duramen (facet) |    18.05 µs |         189 |   73.76 KB |
| cedar (prost)   |    74.78 µs |         344 |   57.51 KB |
| duramen (prost) |    7.152 µs |          77 |   13.49 KB |

#### multi_5

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    55.83 µs |         226 |   37.49 KB |
| duramen         |    2.088 µs |           7 |      448 B |
| cedar (serde)   |    69.57 µs |         418 |   66.98 KB |
| duramen (serde) |    10.24 µs |         212 |    25.3 KB |
| duramen (facet) |       12 µs |         123 |   52.38 KB |
| cedar (prost)   |    59.95 µs |         268 |   43.76 KB |
| duramen (prost) |    4.644 µs |          48 |   12.17 KB |

#### parser_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    738.1 µs |        3534 |   722.9 KB |
| duramen         |    49.22 µs |         149 |   6.308 KB |
| cedar (serde)   |    1.799 ms |        3534 |   722.9 KB |
| duramen (serde) |    49.14 µs |         149 |   6.308 KB |
| duramen (facet) |    49.14 µs |         149 |   6.308 KB |
| cedar (prost)   |    736.6 µs |        3534 |   722.9 KB |
| duramen (prost) |     49.1 µs |         149 |   6.308 KB |

#### corpus_502da

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    71.83 µs |         224 |   73.29 KB |
| duramen         |    5.888 µs |          18 |      800 B |
| cedar (serde)   |    100.4 µs |         441 |   131.5 KB |
| duramen (serde) |    18.62 µs |         206 |    27.9 KB |
| duramen (facet) |    21.73 µs |         139 |   54.89 KB |
| cedar (prost)   |    187.6 µs |         267 |   83.66 KB |
| duramen (prost) |    14.65 µs |          73 |   16.72 KB |

#### corpus_c7e64

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    89.95 µs |         239 |   53.75 KB |
| duramen         |     5.64 µs |          14 |      672 B |
| cedar (serde)   |    121.7 µs |         538 |     108 KB |
| duramen (serde) |    16.85 µs |         254 |   29.16 KB |
| duramen (facet) |    20.07 µs |         162 |   60.86 KB |
| cedar (prost)   |    244.5 µs |         378 |   68.36 KB |
| duramen (prost) |    16.34 µs |         179 |   17.69 KB |

#### corpus_f4174

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     93.7 µs |         238 |   54.77 KB |
| duramen         |    6.057 µs |          14 |      672 B |
| cedar (serde)   |      126 µs |         537 |   110.3 KB |
| duramen (serde) |    17.76 µs |         254 |   29.97 KB |
| duramen (facet) |    21.15 µs |         162 |   61.25 KB |
| cedar (prost)   |    254.1 µs |         377 |   70.36 KB |
| duramen (prost) |    17.34 µs |         179 |    18.4 KB |

### Schema

#### sandbox_a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    395.4 µs |        2232 |   716.6 KB |
| duramen         |    4.488 µs |           2 |      288 B |
| cedar (serde)   |    461.8 µs |        2941 |   920.4 KB |
| duramen (serde) |    28.02 µs |         543 |    63.1 KB |
| duramen (facet) |    30.19 µs |         323 |   96.35 KB |

#### sandbox_b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    428.4 µs |        2454 |   785.3 KB |
| duramen         |    5.113 µs |           2 |      288 B |
| cedar (serde)   |    501.9 µs |        3259 |   1.034 MB |
| duramen (serde) |     33.2 µs |         643 |   82.48 KB |
| duramen (facet) |    34.49 µs |         386 |   123.6 KB |

#### sandbox_b_exts

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    486.9 µs |        2783 |   893.4 KB |
| duramen         |    5.779 µs |           2 |      288 B |
| cedar (serde)   |      564 µs |        3611 |   1.147 MB |
| duramen (serde) |    34.71 µs |         673 |   84.88 KB |
| duramen (facet) |    36.51 µs |         406 |   127.4 KB |

#### validator_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    703.4 µs |        1465 |     459 KB |
| duramen         |    3.114 µs |           2 |      288 B |
| cedar (serde)   |    295.3 µs |        1839 |   595.6 KB |
| duramen (serde) |    16.13 µs |         306 |   51.96 KB |
| duramen (facet) |    17.59 µs |         194 |   71.18 KB |

#### corpus_011ec

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    73.01 ms |      465123 |   156.7 MB |
| duramen         |    399.1 µs |           2 |      288 B |
| cedar (serde)   |    74.81 ms |      477968 |   158.7 MB |
| duramen (serde) |    1.077 ms |       13317 |   1.222 MB |
| duramen (facet) |    1.067 ms |        8222 |   2.088 MB |

#### corpus_37250

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     16.7 ms |       97157 |   32.03 MB |
| duramen         |    204.6 µs |           2 |      288 B |
| cedar (serde)   |    18.14 ms |      116265 |   35.15 MB |
| duramen (serde) |    1.197 ms |       25442 |   1.487 MB |
| duramen (facet) |    1.176 ms |       15634 |   3.257 MB |

#### corpus_bd2fe

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    19.59 ms |      113977 |   37.63 MB |
| duramen         |    265.6 µs |           2 |      288 B |
| cedar (serde)   |    21.28 ms |      136078 |   41.18 MB |
| duramen (serde) |    1.423 ms |       29724 |   1.681 MB |
| duramen (facet) |    1.412 ms |       18223 |   3.771 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
