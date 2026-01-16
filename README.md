![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

![rust: 1.89+](https://img.shields.io/badge/rust-1.89+-orange.svg)
![no-std: compatible](https://img.shields.io/badge/no--std-compatible-success.svg)
![wasm: compatible](https://img.shields.io/badge/wasm-compatible-success.svg)
![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)

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
| cedar           |    39.51 µs |         150 |   28.99 KB |
| duramen         |    1.082 µs |           5 |      384 B |
| cedar (serde)   |    46.95 µs |         244 |   46.84 KB |
| duramen (serde) |    4.988 µs |         113 |   15.36 KB |
| duramen (facet) |    5.905 µs |          71 |    24.6 KB |
| cedar (prost)   |    41.24 µs |         172 |   33.58 KB |
| duramen (prost) |    2.822 µs |          40 |   5.736 KB |

#### decimal_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    46.32 µs |         174 |   35.51 KB |
| duramen         |    1.505 µs |           8 |      480 B |
| cedar (serde)   |    57.42 µs |         319 |   62.51 KB |
| duramen (serde) |    7.588 µs |         169 |   19.33 KB |
| duramen (facet) |    8.483 µs |         110 |    33.5 KB |
| cedar (prost)   |    48.32 µs |         207 |   40.84 KB |
| duramen (prost) |     4.25 µs |          66 |    7.26 KB |

#### example_1a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    30.26 µs |         120 |      24 KB |
| duramen         |    846.2 ns |           2 |      288 B |
| cedar (serde)   |    34.88 µs |         175 |   35.05 KB |
| duramen (serde) |    3.455 µs |          70 |   11.78 KB |
| duramen (facet) |     4.15 µs |          41 |   17.79 KB |
| cedar (prost)   |     31.2 µs |         136 |   28.12 KB |
| duramen (prost) |    1.926 µs |          23 |   4.381 KB |

#### example_2a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    32.71 µs |         125 |   24.32 KB |
| duramen         |    856.5 ns |           2 |      288 B |
| cedar (serde)   |    37.98 µs |         182 |   35.55 KB |
| duramen (serde) |    3.455 µs |          70 |   11.81 KB |
| duramen (facet) |    4.191 µs |          41 |   17.81 KB |
| cedar (prost)   |    34.07 µs |         141 |   28.46 KB |
| duramen (prost) |    1.967 µs |          23 |   4.413 KB |

#### example_2b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    32.06 µs |         125 |    24.3 KB |
| duramen         |    846.3 ns |           2 |      288 B |
| cedar (serde)   |    37.31 µs |         182 |   35.52 KB |
| duramen (serde) |    3.455 µs |          70 |   11.76 KB |
| duramen (facet) |     4.15 µs |          41 |   17.77 KB |
| cedar (prost)   |    33.63 µs |         141 |    28.4 KB |
| duramen (prost) |    1.947 µs |          23 |   4.363 KB |

#### example_2c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    40.54 µs |         148 |   31.19 KB |
| duramen         |    1.033 µs |           2 |      288 B |
| cedar (serde)   |    48.23 µs |         236 |    49.7 KB |
| duramen (serde) |    4.387 µs |          93 |   12.94 KB |
| duramen (facet) |    5.092 µs |          55 |   20.35 KB |
| cedar (prost)   |    41.53 µs |         170 |   35.63 KB |
| duramen (prost) |    2.662 µs |          35 |   5.092 KB |

#### example_3a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.92 µs |         120 |   22.31 KB |
| duramen         |    721.8 ns |           2 |      288 B |
| cedar (serde)   |     34.2 µs |         167 |   31.46 KB |
| duramen (serde) |    2.874 µs |          59 |   11.37 KB |
| duramen (facet) |    3.575 µs |          35 |   16.67 KB |
| cedar (prost)   |    31.33 µs |         134 |   26.36 KB |
| duramen (prost) |    1.609 µs |          18 |   4.258 KB |

#### example_3b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.89 µs |         148 |   31.18 KB |
| duramen         |    1.002 µs |           2 |      288 B |
| cedar (serde)   |    47.62 µs |         236 |   49.69 KB |
| duramen (serde) |    4.345 µs |          93 |   12.94 KB |
| duramen (facet) |    5.113 µs |          55 |   20.35 KB |
| cedar (prost)   |    40.82 µs |         170 |   35.63 KB |
| duramen (prost) |      2.6 µs |          35 |   5.097 KB |

#### example_3c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.84 µs |         120 |   22.32 KB |
| duramen         |    701.1 ns |           2 |      288 B |
| cedar (serde)   |    34.08 µs |         167 |   31.48 KB |
| duramen (serde) |    2.864 µs |          59 |   11.37 KB |
| duramen (facet) |    3.575 µs |          35 |   16.67 KB |
| cedar (prost)   |    31.16 µs |         134 |   26.37 KB |
| duramen (prost) |    1.609 µs |          18 |   4.259 KB |

#### example_4a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    49.73 µs |         181 |   34.04 KB |
| duramen         |    1.403 µs |           6 |      416 B |
| cedar (serde)   |    60.27 µs |         327 |   58.43 KB |
| duramen (serde) |    7.057 µs |         161 |   16.73 KB |
| duramen (facet) |    8.242 µs |         104 |   35.84 KB |
| cedar (prost)   |    51.64 µs |         217 |   39.39 KB |
| duramen (prost) |    3.675 µs |          59 |   6.762 KB |

#### example_4d

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    35.59 µs |         136 |   24.17 KB |
| duramen         |    1.092 µs |           5 |      384 B |
| cedar (serde)   |    41.67 µs |         235 |   38.65 KB |
| duramen (serde) |     5.31 µs |         123 |   14.87 KB |
| duramen (facet) |    6.395 µs |          81 |   31.02 KB |
| cedar (prost)   |    37.21 µs |         161 |   28.94 KB |
| duramen (prost) |    2.367 µs |          38 |   5.654 KB |

#### example_4e

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    37.36 µs |         138 |   24.44 KB |
| duramen         |    1.197 µs |           4 |      352 B |
| cedar (serde)   |    43.95 µs |         241 |   39.24 KB |
| duramen (serde) |    5.783 µs |         128 |   14.89 KB |
| duramen (facet) |    6.726 µs |          82 |   30.81 KB |
| cedar (prost)   |     39.2 µs |         165 |   29.21 KB |
| duramen (prost) |    2.654 µs |          41 |   5.547 KB |

#### example_4f

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    43.28 µs |         156 |   30.32 KB |
| duramen         |    1.474 µs |           8 |      480 B |
| cedar (serde)   |    52.55 µs |         298 |    53.1 KB |
| duramen (serde) |    7.755 µs |         171 |   17.34 KB |
| duramen (facet) |    8.771 µs |         115 |   40.56 KB |
| cedar (prost)   |     44.9 µs |         190 |   35.64 KB |
| duramen (prost) |    3.344 µs |          57 |    6.72 KB |

#### example_5b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     64.4 µs |         223 |   46.11 KB |
| duramen         |    2.557 µs |          16 |      736 B |
| cedar (serde)   |    85.83 µs |         521 |   93.76 KB |
| duramen (serde) |    16.33 µs |         349 |   25.82 KB |
| duramen (facet) |    17.48 µs |         236 |   74.23 KB |
| cedar (prost)   |     69.3 µs |         294 |    53.4 KB |
| duramen (prost) |    7.757 µs |         132 |   9.965 KB |

#### ip_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    37.39 µs |         149 |   27.71 KB |
| duramen         |     1.04 µs |           5 |      384 B |
| cedar (serde)   |     44.9 µs |         244 |   44.22 KB |
| duramen (serde) |    4.945 µs |         114 |    14.8 KB |
| duramen (facet) |    5.946 µs |          72 |   26.02 KB |
| cedar (prost)   |    39.24 µs |         172 |   32.29 KB |
| duramen (prost) |    2.657 µs |          39 |   5.573 KB |

#### ip_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    42.48 µs |         162 |   31.06 KB |
| duramen         |     1.38 µs |           8 |      480 B |
| cedar (serde)   |    52.44 µs |         287 |    53.6 KB |
| duramen (serde) |    6.817 µs |         153 |   17.02 KB |
| duramen (facet) |    7.622 µs |         100 |   33.55 KB |
| cedar (prost)   |    44.86 µs |         193 |   36.09 KB |
| duramen (prost) |    3.512 µs |          56 |   6.218 KB |

#### ip_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.38 µs |         153 |   29.11 KB |
| duramen         |    1.072 µs |           5 |      384 B |
| cedar (serde)   |     47.4 µs |         247 |   46.96 KB |
| duramen (serde) |    4.905 µs |         113 |   15.36 KB |
| duramen (facet) |    5.905 µs |          71 |    24.6 KB |
| cedar (prost)   |    41.28 µs |         175 |    33.7 KB |
| duramen (prost) |    2.822 µs |          40 |   5.735 KB |

#### multi_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    45.79 µs |         177 |   37.46 KB |
| duramen         |    1.584 µs |           2 |      288 B |
| cedar (serde)   |    57.62 µs |         308 |   63.14 KB |
| duramen (serde) |    7.246 µs |         142 |   14.95 KB |
| duramen (facet) |    8.019 µs |          80 |   26.84 KB |
| cedar (prost)   |     47.9 µs |         207 |   42.45 KB |
| duramen (prost) |    4.021 µs |          47 |   5.342 KB |

#### multi_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    40.58 µs |         161 |   31.19 KB |
| duramen         |    1.459 µs |           2 |      288 B |
| cedar (serde)   |     49.7 µs |         262 |    50.1 KB |
| duramen (serde) |    6.148 µs |         118 |   13.82 KB |
| duramen (facet) |    6.953 µs |          66 |   24.34 KB |
| cedar (prost)   |    42.93 µs |         185 |   35.95 KB |
| duramen (prost) |    3.327 µs |          35 |   4.645 KB |

#### multi_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    53.16 µs |         204 |   36.16 KB |
| duramen         |    1.861 µs |           6 |      416 B |
| cedar (serde)   |    66.33 µs |         380 |   63.93 KB |
| duramen (serde) |    10.02 µs |         206 |   18.53 KB |
| duramen (facet) |    11.04 µs |         127 |   43.28 KB |
| cedar (prost)   |    56.44 µs |         247 |    42.1 KB |
| duramen (prost) |    4.583 µs |          66 |   6.573 KB |

#### multi_4

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    71.23 µs |         279 |   49.52 KB |
| duramen         |    3.022 µs |          10 |      544 B |
| cedar (serde)   |    94.36 µs |         582 |   98.42 KB |
| duramen (serde) |    16.83 µs |         350 |   25.67 KB |
| duramen (facet) |    18.94 µs |         213 |   68.63 KB |
| cedar (prost)   |    76.24 µs |         344 |   57.51 KB |
| duramen (prost) |    7.625 µs |         101 |   8.357 KB |

#### multi_5

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    57.59 µs |         226 |   37.49 KB |
| duramen         |     2.15 µs |           7 |      448 B |
| cedar (serde)   |    71.33 µs |         418 |   66.98 KB |
| duramen (serde) |    10.89 µs |         225 |   19.88 KB |
| duramen (facet) |    12.24 µs |         136 |   46.95 KB |
| cedar (prost)   |    60.71 µs |         268 |   43.76 KB |
| duramen (prost) |    4.797 µs |          61 |   6.753 KB |

#### parser_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    754.7 µs |        3534 |   722.9 KB |
| duramen         |     49.6 µs |         149 |   6.308 KB |
| cedar (serde)   |    741.6 µs |        3534 |   722.9 KB |
| duramen (serde) |    49.81 µs |         149 |   6.308 KB |
| duramen (facet) |    49.77 µs |         149 |   6.308 KB |
| cedar (prost)   |    742.1 µs |        3534 |   722.9 KB |
| duramen (prost) |    49.73 µs |         149 |   6.308 KB |

#### corpus_502da

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    73.39 µs |         224 |   73.29 KB |
| duramen         |    5.931 µs |          18 |      800 B |
| cedar (serde)   |      102 µs |         441 |   131.5 KB |
| duramen (serde) |    19.98 µs |         221 |    27.6 KB |
| duramen (facet) |    22.51 µs |         154 |   54.59 KB |
| cedar (prost)   |    76.55 µs |         267 |   83.66 KB |
| duramen (prost) |    15.01 µs |          88 |   16.42 KB |

#### corpus_c7e64

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     91.5 µs |         239 |   53.75 KB |
| duramen         |    5.724 µs |          14 |      672 B |
| cedar (serde)   |    122.4 µs |         538 |     108 KB |
| duramen (serde) |    18.99 µs |         268 |   27.48 KB |
| duramen (facet) |    20.76 µs |         176 |   59.18 KB |
| cedar (prost)   |    97.96 µs |         378 |   68.36 KB |
| duramen (prost) |    17.61 µs |         193 |   16.02 KB |

#### corpus_f4174

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    95.04 µs |         238 |   54.77 KB |
| duramen         |    6.182 µs |          14 |      672 B |
| cedar (serde)   |    127.4 µs |         537 |   110.3 KB |
| duramen (serde) |    19.53 µs |         268 |    28.6 KB |
| duramen (facet) |    21.67 µs |         176 |   59.88 KB |
| cedar (prost)   |    101.7 µs |         377 |   70.36 KB |
| duramen (prost) |     18.4 µs |         193 |   17.03 KB |

### Schema

#### sandbox_a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    396.8 µs |        2232 |   716.6 KB |
| duramen         |     4.61 µs |           2 |      288 B |
| cedar (serde)   |    464.6 µs |        2941 |   920.4 KB |
| duramen (serde) |    32.83 µs |         676 |   66.54 KB |
| duramen (facet) |    34.54 µs |         456 |   99.79 KB |

#### sandbox_b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    433.4 µs |        2454 |   785.3 KB |
| duramen         |     5.28 µs |           2 |      288 B |
| cedar (serde)   |      508 µs |        3259 |   1.034 MB |
| duramen (serde) |    39.02 µs |         784 |   88.72 KB |
| duramen (facet) |    39.45 µs |         527 |   129.8 KB |

#### sandbox_b_exts

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    490.5 µs |        2783 |   893.4 KB |
| duramen         |     5.95 µs |           2 |      288 B |
| cedar (serde)   |    567.8 µs |        3611 |   1.147 MB |
| duramen (serde) |    38.24 µs |         826 |   92.11 KB |
| duramen (facet) |     41.6 µs |         559 |   134.6 KB |

#### validator_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    266.9 µs |        1465 |     459 KB |
| duramen         |      3.2 µs |           2 |      288 B |
| cedar (serde)   |    295.8 µs |        1839 |   595.6 KB |
| duramen (serde) |    17.53 µs |         360 |   54.93 KB |
| duramen (facet) |    19.15 µs |         248 |   74.15 KB |

#### corpus_011ec

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    72.73 ms |      465123 |   156.7 MB |
| duramen         |     1.09 ms |           2 |      288 B |
| cedar (serde)   |    74.62 ms |      477968 |   158.7 MB |
| duramen (serde) |     1.26 ms |       17190 |   1.162 MB |
| duramen (facet) |     1.23 ms |       12095 |   2.027 MB |

#### corpus_37250

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    16.71 ms |       97157 |   32.03 MB |
| duramen         |    206.1 µs |           2 |      288 B |
| cedar (serde)   |    17.98 ms |      116265 |   35.15 MB |
| duramen (serde) |     1.51 ms |       32885 |   1.609 MB |
| duramen (facet) |     1.47 ms |       23077 |   3.379 MB |

#### corpus_bd2fe

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    19.63 ms |      113977 |   37.63 MB |
| duramen         |    267.2 µs |           2 |      288 B |
| cedar (serde)   |    21.28 ms |      136078 |   41.18 MB |
| duramen (serde) |     1.74 ms |       39508 |   1.807 MB |
| duramen (facet) |     1.72 ms |       28007 |   3.897 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
