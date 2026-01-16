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
| cedar           |    38.59 µs |         150 |   28.99 KB |
| duramen         |    1.061 µs |           5 |      384 B |
| cedar (serde)   |    47.15 µs |         244 |   46.84 KB |
| duramen (serde) |     4.78 µs |         113 |   15.36 KB |
| duramen (facet) |    5.822 µs |          71 |    24.6 KB |
| cedar (prost)   |    106.1 µs |         172 |   33.58 KB |
| duramen (prost) |    2.739 µs |          40 |   5.736 KB |

#### decimal_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    45.32 µs |         174 |   35.51 KB |
| duramen         |    1.484 µs |           8 |      480 B |
| cedar (serde)   |    57.57 µs |         319 |   62.51 KB |
| duramen (serde) |    7.213 µs |         169 |   19.33 KB |
| duramen (facet) |    8.442 µs |         110 |    33.5 KB |
| cedar (prost)   |    125.4 µs |         207 |   40.84 KB |
| duramen (prost) |    4.124 µs |          66 |    7.26 KB |

#### example_1a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.34 µs |         120 |      24 KB |
| duramen         |    825.4 ns |           2 |      288 B |
| cedar (serde)   |    34.45 µs |         175 |   35.05 KB |
| duramen (serde) |    3.288 µs |          70 |   11.78 KB |
| duramen (facet) |    4.024 µs |          41 |   17.79 KB |
| cedar (prost)   |    78.78 µs |         136 |   28.12 KB |
| duramen (prost) |    1.884 µs |          23 |   4.381 KB |

#### example_2a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    31.92 µs |         125 |   24.32 KB |
| duramen         |    835.8 ns |           2 |      288 B |
| cedar (serde)   |    37.64 µs |         182 |   35.55 KB |
| duramen (serde) |     3.33 µs |          70 |   11.81 KB |
| duramen (facet) |    4.066 µs |          41 |   17.81 KB |
| cedar (prost)   |    86.73 µs |         141 |   28.46 KB |
| duramen (prost) |    1.905 µs |          23 |   4.413 KB |

#### example_2b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    31.04 µs |         125 |    24.3 KB |
| duramen         |    830.7 ns |           2 |      288 B |
| cedar (serde)   |    37.19 µs |         182 |   35.52 KB |
| duramen (serde) |    3.246 µs |          70 |   11.76 KB |
| duramen (facet) |    4.066 µs |          41 |   17.77 KB |
| cedar (prost)   |    85.48 µs |         141 |    28.4 KB |
| duramen (prost) |    1.884 µs |          23 |   4.363 KB |

#### example_2c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.18 µs |         148 |   31.19 KB |
| duramen         |    1.002 µs |           2 |      288 B |
| cedar (serde)   |     48.2 µs |         236 |    49.7 KB |
| duramen (serde) |    4.219 µs |          93 |   12.94 KB |
| duramen (facet) |    5.008 µs |          55 |   20.35 KB |
| cedar (prost)   |    105.7 µs |         170 |   35.63 KB |
| duramen (prost) |    2.536 µs |          35 |   5.092 KB |

#### example_3a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.09 µs |         120 |   22.31 KB |
| duramen         |      701 ns |           2 |      288 B |
| cedar (serde)   |    34.04 µs |         167 |   31.46 KB |
| duramen (serde) |    2.801 µs |          59 |   11.37 KB |
| duramen (facet) |    3.533 µs |          35 |   16.67 KB |
| cedar (prost)   |    31.03 µs |         134 |   26.36 KB |
| duramen (prost) |    1.557 µs |          18 |   4.258 KB |

#### example_3b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.39 µs |         148 |   31.18 KB |
| duramen         |    981.5 ns |           2 |      288 B |
| cedar (serde)   |    47.41 µs |         236 |   49.69 KB |
| duramen (serde) |    4.136 µs |          93 |   12.94 KB |
| duramen (facet) |    4.967 µs |          55 |   20.35 KB |
| cedar (prost)   |    40.86 µs |         170 |   35.63 KB |
| duramen (prost) |    2.578 µs |          35 |   5.097 KB |

#### example_3c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    28.92 µs |         120 |   22.32 KB |
| duramen         |    685.3 ns |           2 |      288 B |
| cedar (serde)   |    34.16 µs |         167 |   31.48 KB |
| duramen (serde) |    2.801 µs |          59 |   11.37 KB |
| duramen (facet) |    3.491 µs |          35 |   16.67 KB |
| cedar (prost)   |    30.95 µs |         134 |   26.37 KB |
| duramen (prost) |    1.547 µs |          18 |   4.259 KB |

#### example_4a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    48.43 µs |         181 |   34.04 KB |
| duramen         |    1.382 µs |           6 |      416 B |
| cedar (serde)   |     60.6 µs |         327 |   58.43 KB |
| duramen (serde) |    6.807 µs |         161 |   16.73 KB |
| duramen (facet) |    8.075 µs |         104 |   35.84 KB |
| cedar (prost)   |    51.77 µs |         217 |   39.39 KB |
| duramen (prost) |    3.592 µs |          59 |   6.762 KB |

#### example_4d

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    34.61 µs |         136 |   24.17 KB |
| duramen         |    1.071 µs |           5 |      384 B |
| cedar (serde)   |    41.41 µs |         235 |   38.65 KB |
| duramen (serde) |     5.06 µs |         123 |   14.87 KB |
| duramen (facet) |     6.31 µs |          81 |   31.02 KB |
| cedar (prost)   |    36.83 µs |         161 |   28.94 KB |
| duramen (prost) |    2.283 µs |          38 |   5.654 KB |

#### example_4e

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    36.19 µs |         138 |   24.44 KB |
| duramen         |    1.166 µs |           4 |      352 B |
| cedar (serde)   |    43.28 µs |         241 |   39.24 KB |
| duramen (serde) |    5.387 µs |         128 |   14.89 KB |
| duramen (facet) |    6.643 µs |          82 |   30.81 KB |
| cedar (prost)   |    39.12 µs |         165 |   29.21 KB |
| duramen (prost) |    2.529 µs |          41 |   5.547 KB |

#### example_4f

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    42.21 µs |         156 |   30.32 KB |
| duramen         |    1.443 µs |           8 |      480 B |
| cedar (serde)   |    52.76 µs |         298 |    53.1 KB |
| duramen (serde) |    7.088 µs |         171 |   17.34 KB |
| duramen (facet) |    8.562 µs |         115 |   40.56 KB |
| cedar (prost)   |    44.67 µs |         190 |   35.64 KB |
| duramen (prost) |    3.302 µs |          57 |    6.72 KB |

#### example_5b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    63.17 µs |         223 |   46.11 KB |
| duramen         |    2.473 µs |          16 |      736 B |
| cedar (serde)   |    86.45 µs |         521 |   93.76 KB |
| duramen (serde) |    14.79 µs |         349 |   25.82 KB |
| duramen (facet) |    17.21 µs |         236 |   74.23 KB |
| cedar (prost)   |    69.92 µs |         294 |    53.4 KB |
| duramen (prost) |    7.422 µs |         132 |   9.965 KB |

#### ip_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    36.18 µs |         149 |   27.71 KB |
| duramen         |    1.009 µs |           5 |      384 B |
| cedar (serde)   |    44.74 µs |         244 |   44.22 KB |
| duramen (serde) |    4.695 µs |         114 |    14.8 KB |
| duramen (facet) |    5.821 µs |          72 |   26.02 KB |
| cedar (prost)   |    39.15 µs |         172 |   32.29 KB |
| duramen (prost) |    2.532 µs |          39 |   5.573 KB |

#### ip_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    41.58 µs |         162 |   31.06 KB |
| duramen         |    1.338 µs |           8 |      480 B |
| cedar (serde)   |    52.19 µs |         287 |    53.6 KB |
| duramen (serde) |      6.4 µs |         153 |   17.02 KB |
| duramen (facet) |    7.497 µs |         100 |   33.55 KB |
| cedar (prost)   |    44.96 µs |         193 |   36.09 KB |
| duramen (prost) |    3.429 µs |          56 |   6.218 KB |

#### ip_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.42 µs |         153 |   29.11 KB |
| duramen         |     1.04 µs |           5 |      384 B |
| cedar (serde)   |    47.36 µs |         247 |   46.96 KB |
| duramen (serde) |    4.821 µs |         113 |   15.36 KB |
| duramen (facet) |     5.78 µs |          71 |    24.6 KB |
| cedar (prost)   |    41.28 µs |         175 |    33.7 KB |
| duramen (prost) |     2.78 µs |          40 |   5.735 KB |

#### multi_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    44.52 µs |         177 |   37.46 KB |
| duramen         |    1.532 µs |           2 |      288 B |
| cedar (serde)   |    57.66 µs |         308 |   63.14 KB |
| duramen (serde) |     6.87 µs |         142 |   14.95 KB |
| duramen (facet) |    7.852 µs |          80 |   26.84 KB |
| cedar (prost)   |    48.36 µs |         207 |   42.45 KB |
| duramen (prost) |     3.98 µs |          47 |   5.342 KB |

#### multi_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.79 µs |         161 |   31.19 KB |
| duramen         |    1.397 µs |           2 |      288 B |
| cedar (serde)   |    49.88 µs |         262 |    50.1 KB |
| duramen (serde) |    5.773 µs |         118 |   13.82 KB |
| duramen (facet) |    6.868 µs |          66 |   24.34 KB |
| cedar (prost)   |     42.8 µs |         185 |   35.95 KB |
| duramen (prost) |    3.244 µs |          35 |   4.645 KB |

#### multi_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    52.57 µs |         204 |   36.16 KB |
| duramen         |    1.798 µs |           6 |      416 B |
| cedar (serde)   |     66.2 µs |         380 |   63.93 KB |
| duramen (serde) |     9.13 µs |         206 |   18.53 KB |
| duramen (facet) |    10.75 µs |         127 |   43.28 KB |
| cedar (prost)   |    56.73 µs |         247 |    42.1 KB |
| duramen (prost) |    4.541 µs |          66 |   6.573 KB |

#### multi_4

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     70.9 µs |         279 |   49.52 KB |
| duramen         |     2.98 µs |          10 |      544 B |
| cedar (serde)   |    93.88 µs |         582 |   98.42 KB |
| duramen (serde) |    16.29 µs |         350 |   25.67 KB |
| duramen (facet) |    18.65 µs |         213 |   68.63 KB |
| cedar (prost)   |    76.95 µs |         344 |   57.51 KB |
| duramen (prost) |    7.417 µs |         101 |   8.357 KB |

#### multi_5

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    56.34 µs |         226 |   37.49 KB |
| duramen         |    2.088 µs |           7 |      448 B |
| cedar (serde)   |    70.82 µs |         418 |   66.98 KB |
| duramen (serde) |    10.31 µs |         225 |   19.88 KB |
| duramen (facet) |    12.16 µs |         136 |   46.95 KB |
| cedar (prost)   |    61.12 µs |         268 |   43.76 KB |
| duramen (prost) |    4.754 µs |          61 |   6.753 KB |

#### parser_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    738.5 µs |        3534 |   722.9 KB |
| duramen         |    46.81 µs |         149 |   6.308 KB |
| cedar (serde)   |    774.9 µs |        3534 |   722.9 KB |
| duramen (serde) |    46.85 µs |         149 |   6.308 KB |
| duramen (facet) |    47.18 µs |         149 |   6.308 KB |
| cedar (prost)   |    773.2 µs |        3534 |   722.9 KB |
| duramen (prost) |    46.98 µs |         149 |   6.308 KB |

#### corpus_502da

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    71.83 µs |         224 |   73.29 KB |
| duramen         |    5.888 µs |          18 |      800 B |
| cedar (serde)   |    102.9 µs |         441 |   131.5 KB |
| duramen (serde) |    19.69 µs |         221 |    27.6 KB |
| duramen (facet) |     22.3 µs |         154 |   54.59 KB |
| cedar (prost)   |    77.86 µs |         267 |   83.66 KB |
| duramen (prost) |    14.89 µs |          88 |   16.42 KB |

#### corpus_c7e64

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    89.79 µs |         239 |   53.75 KB |
| duramen         |    5.599 µs |          14 |      672 B |
| cedar (serde)   |    123.9 µs |         538 |     108 KB |
| duramen (serde) |    17.49 µs |         268 |   27.48 KB |
| duramen (facet) |    20.46 µs |         176 |   59.18 KB |
| cedar (prost)   |    100.2 µs |         378 |   68.36 KB |
| duramen (prost) |    16.65 µs |         193 |   16.02 KB |

#### corpus_f4174

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    94.25 µs |         238 |   54.77 KB |
| duramen         |    6.099 µs |          14 |      672 B |
| cedar (serde)   |    128.7 µs |         537 |   110.3 KB |
| duramen (serde) |    18.32 µs |         268 |    28.6 KB |
| duramen (facet) |    21.42 µs |         176 |   59.88 KB |
| cedar (prost)   |    259.8 µs |         377 |   70.36 KB |
| duramen (prost) |    17.32 µs |         193 |   17.03 KB |

### Schema

#### sandbox_a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    398.8 µs |        2232 |   716.6 KB |
| duramen         |    4.488 µs |           2 |      288 B |
| cedar (serde)   |    458.5 µs |        2941 |   920.4 KB |
| duramen (serde) |    30.16 µs |         676 |   66.54 KB |
| duramen (facet) |    34.29 µs |         456 |   99.79 KB |

#### sandbox_b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    433.7 µs |        2454 |   785.3 KB |
| duramen         |    5.196 µs |           2 |      288 B |
| cedar (serde)   |    499.5 µs |        3259 |   1.034 MB |
| duramen (serde) |    35.25 µs |         784 |   88.72 KB |
| duramen (facet) |    38.79 µs |         527 |   129.8 KB |

#### sandbox_b_exts

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    490.5 µs |        2783 |   893.4 KB |
| duramen         |    5.821 µs |           2 |      288 B |
| cedar (serde)   |    558.6 µs |        3611 |   1.147 MB |
| duramen (serde) |    37.12 µs |         826 |   92.11 KB |
| duramen (facet) |    40.79 µs |         559 |   134.6 KB |

#### validator_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    268.2 µs |        1465 |     459 KB |
| duramen         |    3.114 µs |           2 |      288 B |
| cedar (serde)   |    292.9 µs |        1839 |   595.6 KB |
| duramen (serde) |    16.61 µs |         360 |   54.93 KB |
| duramen (facet) |     18.9 µs |         248 |   74.15 KB |

#### corpus_011ec

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    73.74 ms |      465123 |   156.7 MB |
| duramen         |    378.5 µs |           2 |      288 B |
| cedar (serde)   |    74.44 ms |      477968 |   158.7 MB |
| duramen (serde) |    1.223 ms |       17190 |   1.162 MB |
| duramen (facet) |    1.199 ms |       12095 |   2.027 MB |

#### corpus_37250

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    16.85 ms |       97157 |   32.03 MB |
| duramen         |    204.2 µs |           2 |      288 B |
| cedar (serde)   |    18.15 ms |      116265 |   35.15 MB |
| duramen (serde) |     1.49 ms |       32885 |   1.609 MB |
| duramen (facet) |    1.456 ms |       23077 |   3.379 MB |

#### corpus_bd2fe

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    19.79 ms |      113977 |   37.63 MB |
| duramen         |    245.1 µs |           2 |      288 B |
| cedar (serde)   |    21.25 ms |      136078 |   41.18 MB |
| duramen (serde) |    1.708 ms |       39508 |   1.807 MB |
| duramen (facet) |    1.685 ms |       28007 |   3.897 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
