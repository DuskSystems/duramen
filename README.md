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
| cedar           |     39.1 µs |         150 |   28.99 KB |
| duramen         |    894.8 ns |           5 |      384 B |
| cedar (serde)   |    45.69 µs |         244 |   46.84 KB |
| duramen (serde) |    4.498 µs |         104 |   15.07 KB |
| duramen (facet) |    5.499 µs |          62 |   24.31 KB |
| cedar (prost)   |    40.57 µs |         172 |   33.58 KB |
| duramen (prost) |    2.458 µs |          31 |    5.45 KB |

#### decimal_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    46.07 µs |         174 |   35.51 KB |
| duramen         |    1.286 µs |           8 |      480 B |
| cedar (serde)   |    56.32 µs |         319 |   62.51 KB |
| duramen (serde) |    6.735 µs |         151 |   18.38 KB |
| duramen (facet) |    7.921 µs |          92 |   32.56 KB |
| cedar (prost)   |    47.78 µs |         207 |   40.84 KB |
| duramen (prost) |     3.77 µs |          48 |   6.318 KB |

#### example_1a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    30.42 µs |         120 |      24 KB |
| duramen         |    674.3 ns |           2 |      288 B |
| cedar (serde)   |    33.49 µs |         175 |   35.05 KB |
| duramen (serde) |    3.125 µs |          67 |   11.33 KB |
| duramen (facet) |     3.82 µs |          38 |   17.33 KB |
| cedar (prost)   |     30.7 µs |         136 |   28.12 KB |
| duramen (prost) |    1.762 µs |          20 |   3.929 KB |

#### example_2a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    32.29 µs |         125 |   24.32 KB |
| duramen         |    679.6 ns |           2 |      288 B |
| cedar (serde)   |    36.98 µs |         182 |   35.55 KB |
| duramen (serde) |    3.167 µs |          67 |   11.35 KB |
| duramen (facet) |    3.861 µs |          38 |   17.35 KB |
| cedar (prost)   |    33.48 µs |         141 |   28.46 KB |
| duramen (prost) |    1.762 µs |          20 |   3.953 KB |

#### example_2b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    31.83 µs |         125 |    24.3 KB |
| duramen         |    674.3 ns |           2 |      288 B |
| cedar (serde)   |    36.39 µs |         182 |   35.52 KB |
| duramen (serde) |    3.208 µs |          67 |   11.32 KB |
| duramen (facet) |     3.82 µs |          38 |   17.33 KB |
| cedar (prost)   |    32.98 µs |         141 |    28.4 KB |
| duramen (prost) |    1.763 µs |          20 |   3.917 KB |

#### example_2c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    40.06 µs |         148 |   31.19 KB |
| duramen         |    804.6 ns |           2 |      288 B |
| cedar (serde)   |    47.37 µs |         236 |    49.7 KB |
| duramen (serde) |    4.017 µs |          88 |   13.38 KB |
| duramen (facet) |    4.722 µs |          50 |   20.79 KB |
| cedar (prost)   |    40.91 µs |         170 |   35.63 KB |
| duramen (prost) |    2.334 µs |          30 |   5.531 KB |

#### example_3a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.55 µs |         120 |   22.31 KB |
| duramen         |    576.1 ns |           2 |      288 B |
| cedar (serde)   |    33.12 µs |         167 |   31.46 KB |
| duramen (serde) |    2.638 µs |          57 |   10.87 KB |
| duramen (facet) |    3.368 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.62 µs |         134 |   26.36 KB |
| duramen (prost) |    1.403 µs |          16 |   3.761 KB |

#### example_3b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.97 µs |         148 |   31.18 KB |
| duramen         |    804.6 ns |           2 |      288 B |
| cedar (serde)   |    46.54 µs |         236 |   49.69 KB |
| duramen (serde) |    3.975 µs |          88 |   13.38 KB |
| duramen (facet) |    4.722 µs |          50 |   20.79 KB |
| cedar (prost)   |     40.2 µs |         170 |   35.63 KB |
| duramen (prost) |    2.334 µs |          30 |   5.536 KB |

#### example_3c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.51 µs |         120 |   22.32 KB |
| duramen         |      576 ns |           2 |      288 B |
| cedar (serde)   |     33.2 µs |         167 |   31.48 KB |
| duramen (serde) |    2.741 µs |          57 |   10.87 KB |
| duramen (facet) |     3.41 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.74 µs |         134 |   26.37 KB |
| duramen (prost) |    1.424 µs |          16 |   3.761 KB |

#### example_4a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    49.43 µs |         181 |   34.04 KB |
| duramen         |    1.143 µs |           6 |      416 B |
| cedar (serde)   |    59.06 µs |         327 |   58.43 KB |
| duramen (serde) |    6.533 µs |         146 |   16.32 KB |
| duramen (facet) |    7.718 µs |          89 |   35.43 KB |
| cedar (prost)   |     50.6 µs |         217 |   39.39 KB |
| duramen (prost) |    3.193 µs |          44 |    6.35 KB |

#### example_4d

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    35.24 µs |         136 |   24.17 KB |
| duramen         |    905.2 ns |           5 |      384 B |
| cedar (serde)   |    41.12 µs |         235 |   38.65 KB |
| duramen (serde) |    4.822 µs |         113 |   14.59 KB |
| duramen (facet) |     5.99 µs |          71 |   30.73 KB |
| cedar (prost)   |    36.33 µs |         161 |   28.94 KB |
| duramen (prost) |    2.003 µs |          28 |   5.371 KB |

#### example_4e

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    37.36 µs |         138 |   24.44 KB |
| duramen         |    989.1 ns |           4 |      352 B |
| cedar (serde)   |    43.49 µs |         241 |   39.24 KB |
| duramen (serde) |    5.108 µs |         117 |   14.73 KB |
| duramen (facet) |    6.321 µs |          71 |   30.65 KB |
| cedar (prost)   |     38.5 µs |         165 |   29.21 KB |
| duramen (prost) |    2.313 µs |          30 |   5.391 KB |

#### example_4f

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     43.3 µs |         156 |   30.32 KB |
| duramen         |    1.234 µs |           8 |      480 B |
| cedar (serde)   |    51.72 µs |         298 |    53.1 KB |
| duramen (serde) |     6.65 µs |         154 |   16.59 KB |
| duramen (facet) |    8.166 µs |          98 |   39.81 KB |
| cedar (prost)   |    43.92 µs |         190 |   35.64 KB |
| duramen (prost) |    2.864 µs |          40 |   5.975 KB |

#### example_5b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    65.55 µs |         223 |   46.11 KB |
| duramen         |    2.139 µs |          16 |      736 B |
| cedar (serde)   |    84.12 µs |         521 |   93.76 KB |
| duramen (serde) |    13.97 µs |         308 |   26.24 KB |
| duramen (facet) |    15.88 µs |         195 |   74.65 KB |
| cedar (prost)   |     68.3 µs |         294 |    53.4 KB |
| duramen (prost) |    6.429 µs |          91 |   10.38 KB |

#### ip_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    37.47 µs |         149 |   27.71 KB |
| duramen         |    858.3 ns |           5 |      384 B |
| cedar (serde)   |    44.11 µs |         244 |   44.22 KB |
| duramen (serde) |    4.497 µs |         106 |   14.69 KB |
| duramen (facet) |    5.455 µs |          64 |   25.91 KB |
| cedar (prost)   |    38.53 µs |         172 |   32.29 KB |
| duramen (prost) |    2.333 µs |          31 |   5.462 KB |

#### ip_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    42.87 µs |         162 |   31.06 KB |
| duramen         |    1.192 µs |           8 |      480 B |
| cedar (serde)   |    50.94 µs |         287 |    53.6 KB |
| duramen (serde) |    6.042 µs |         139 |   16.79 KB |
| duramen (facet) |    7.179 µs |          86 |   33.31 KB |
| cedar (prost)   |    43.75 µs |         193 |   36.09 KB |
| duramen (prost) |    3.153 µs |          42 |    5.98 KB |

#### ip_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.92 µs |         153 |   29.11 KB |
| duramen         |    889.6 ns |           5 |      384 B |
| cedar (serde)   |    46.44 µs |         247 |   46.96 KB |
| duramen (serde) |    4.498 µs |         104 |   15.07 KB |
| duramen (facet) |    5.458 µs |          62 |   24.31 KB |
| cedar (prost)   |    40.48 µs |         175 |    33.7 KB |
| duramen (prost) |    2.458 µs |          31 |   5.446 KB |

#### multi_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    45.69 µs |         177 |   37.46 KB |
| duramen         |     1.24 µs |           2 |      288 B |
| cedar (serde)   |    56.58 µs |         308 |   63.14 KB |
| duramen (serde) |    6.504 µs |         135 |    15.5 KB |
| duramen (facet) |    7.443 µs |          73 |   27.39 KB |
| cedar (prost)   |    47.07 µs |         207 |   42.45 KB |
| duramen (prost) |    3.529 µs |          40 |    5.89 KB |

#### multi_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    41.21 µs |         161 |   31.19 KB |
| duramen         |    1.105 µs |           2 |      288 B |
| cedar (serde)   |    48.55 µs |         262 |    50.1 KB |
| duramen (serde) |    5.486 µs |         114 |   14.48 KB |
| duramen (facet) |    6.457 µs |          62 |      25 KB |
| cedar (prost)   |     42.1 µs |         185 |   35.95 KB |
| duramen (prost) |    2.957 µs |          31 |   5.304 KB |

#### multi_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    54.57 µs |         204 |   36.16 KB |
| duramen         |    1.496 µs |           6 |      416 B |
| cedar (serde)   |     65.2 µs |         380 |   63.93 KB |
| duramen (serde) |    8.687 µs |         192 |   20.36 KB |
| duramen (facet) |    10.31 µs |         113 |   45.11 KB |
| cedar (prost)   |    55.86 µs |         247 |    42.1 KB |
| duramen (prost) |    4.058 µs |          52 |   8.404 KB |

#### multi_4

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    72.69 µs |         279 |   49.52 KB |
| duramen         |     2.48 µs |          10 |      544 B |
| cedar (serde)   |    92.84 µs |         582 |   98.42 KB |
| duramen (serde) |    15.07 µs |         326 |    30.8 KB |
| duramen (facet) |    17.59 µs |         189 |   73.76 KB |
| cedar (prost)   |     75.2 µs |         344 |   57.51 KB |
| duramen (prost) |     6.61 µs |          77 |   13.49 KB |

#### multi_5

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    59.09 µs |         226 |   37.49 KB |
| duramen         |    1.775 µs |           7 |      448 B |
| cedar (serde)   |    69.82 µs |         418 |   66.98 KB |
| duramen (serde) |     9.83 µs |         212 |    25.3 KB |
| duramen (facet) |    11.63 µs |         123 |   52.38 KB |
| cedar (prost)   |    59.91 µs |         268 |   43.76 KB |
| duramen (prost) |    4.353 µs |          48 |   12.17 KB |

#### parser_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    744.6 µs |        3534 |   722.9 KB |
| duramen         |    40.77 µs |         149 |   6.308 KB |
| cedar (serde)   |      736 µs |        3534 |   722.9 KB |
| duramen (serde) |    40.81 µs |         149 |   6.308 KB |
| duramen (facet) |    40.68 µs |         149 |   6.308 KB |
| cedar (prost)   |    738.3 µs |        3534 |   722.9 KB |
| duramen (prost) |    40.85 µs |         149 |   6.308 KB |

#### corpus_502da

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    72.75 µs |         224 |   73.29 KB |
| duramen         |    5.347 µs |          18 |      800 B |
| cedar (serde)   |    100.9 µs |         441 |   131.5 KB |
| duramen (serde) |    18.42 µs |         206 |    27.9 KB |
| duramen (facet) |    21.11 µs |         139 |   54.89 KB |
| cedar (prost)   |    76.03 µs |         267 |   83.66 KB |
| duramen (prost) |    14.11 µs |          73 |   16.72 KB |

#### corpus_c7e64

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     91.2 µs |         239 |   53.75 KB |
| duramen         |     4.89 µs |          14 |      672 B |
| cedar (serde)   |    120.8 µs |         538 |     108 KB |
| duramen (serde) |    16.31 µs |         254 |   29.16 KB |
| duramen (facet) |     19.4 µs |         162 |   60.86 KB |
| cedar (prost)   |    96.21 µs |         378 |   68.36 KB |
| duramen (prost) |    15.51 µs |         179 |   17.69 KB |

#### corpus_f4174

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    94.66 µs |         238 |   54.77 KB |
| duramen         |    5.307 µs |          14 |      672 B |
| cedar (serde)   |      126 µs |         537 |   110.3 KB |
| duramen (serde) |    17.22 µs |         254 |   29.97 KB |
| duramen (facet) |    20.36 µs |         162 |   61.25 KB |
| cedar (prost)   |    100.4 µs |         377 |   70.36 KB |
| duramen (prost) |    16.34 µs |         179 |    18.4 KB |

### Schema

#### sandbox_a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |      394 µs |        2232 |   716.6 KB |
| duramen         |    4.029 µs |           2 |      288 B |
| cedar (serde)   |    459.4 µs |        2941 |   920.4 KB |
| duramen (serde) |    26.61 µs |         543 |    63.1 KB |
| duramen (facet) |    29.65 µs |         323 |   96.35 KB |

#### sandbox_b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    427.5 µs |        2454 |   785.3 KB |
| duramen         |    4.488 µs |           2 |      288 B |
| cedar (serde)   |    500.1 µs |        3259 |   1.034 MB |
| duramen (serde) |    32.04 µs |         643 |   82.48 KB |
| duramen (facet) |    33.87 µs |         386 |   123.6 KB |

#### sandbox_b_exts

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    485.1 µs |        2783 |   893.4 KB |
| duramen         |    5.154 µs |           2 |      288 B |
| cedar (serde)   |    561.3 µs |        3611 |   1.147 MB |
| duramen (serde) |    33.13 µs |         673 |   84.88 KB |
| duramen (facet) |    35.59 µs |         406 |   127.4 KB |

#### validator_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    264.5 µs |        1465 |     459 KB |
| duramen         |    2.614 µs |           2 |      288 B |
| cedar (serde)   |    291.5 µs |        1839 |   595.6 KB |
| duramen (serde) |    15.13 µs |         306 |   51.96 KB |
| duramen (facet) |    16.97 µs |         194 |   71.18 KB |

#### corpus_011ec

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |       72 ms |      465123 |   156.7 MB |
| duramen         |    342.6 µs |           2 |      288 B |
| cedar (serde)   |    73.87 ms |      479176 |   159.1 MB |
| duramen (serde) |    1.024 ms |       13317 |   1.222 MB |
| duramen (facet) |    1.018 ms |        8222 |   2.088 MB |

#### corpus_37250

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    16.58 ms |       97157 |   32.03 MB |
| duramen         |    158.8 µs |           2 |      288 B |
| cedar (serde)   |     18.1 ms |      116265 |   35.15 MB |
| duramen (serde) |    1.142 ms |       25442 |   1.487 MB |
| duramen (facet) |    1.127 ms |       15634 |   3.257 MB |

#### corpus_bd2fe

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     19.5 ms |      113977 |   37.63 MB |
| duramen         |    212.1 µs |           2 |      288 B |
| cedar (serde)   |    21.24 ms |      136078 |   41.18 MB |
| duramen (serde) |    1.368 ms |       29724 |   1.681 MB |
| duramen (facet) |    1.355 ms |       18223 |   3.771 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
