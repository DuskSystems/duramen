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
| cedar           |     38.3 µs |         150 |   28.99 KB |
| duramen         |    1.061 µs |           5 |      384 B |
| cedar (serde)   |    46.07 µs |         244 |   46.84 KB |
| duramen (serde) |    4.539 µs |         104 |   15.07 KB |
| duramen (facet) |    5.791 µs |          62 |   24.31 KB |
| cedar (prost)   |    40.44 µs |         172 |   33.58 KB |
| duramen (prost) |    2.665 µs |          31 |    5.45 KB |

#### decimal_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     44.9 µs |         174 |   35.51 KB |
| duramen         |    1.484 µs |           8 |      480 B |
| cedar (serde)   |    56.57 µs |         319 |   62.51 KB |
| duramen (serde) |    6.777 µs |         151 |   18.38 KB |
| duramen (facet) |    8.087 µs |          92 |   32.56 KB |
| cedar (prost)   |    47.57 µs |         207 |   40.84 KB |
| duramen (prost) |    3.895 µs |          48 |   6.318 KB |

#### example_1a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.63 µs |         120 |      24 KB |
| duramen         |    830.5 ns |           2 |      288 B |
| cedar (serde)   |    33.86 µs |         175 |   35.05 KB |
| duramen (serde) |    3.209 µs |          67 |   11.33 KB |
| duramen (facet) |    4.028 µs |          38 |   17.33 KB |
| cedar (prost)   |    30.91 µs |         136 |   28.12 KB |
| duramen (prost) |    1.887 µs |          20 |   3.929 KB |

#### example_2a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    32.29 µs |         125 |   24.32 KB |
| duramen         |    840.9 ns |           2 |      288 B |
| cedar (serde)   |    37.19 µs |         182 |   35.55 KB |
| duramen (serde) |    3.167 µs |          67 |   11.35 KB |
| duramen (facet) |     4.07 µs |          38 |   17.35 KB |
| cedar (prost)   |    33.82 µs |         141 |   28.46 KB |
| duramen (prost) |    1.908 µs |          20 |   3.953 KB |

#### example_2b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    31.83 µs |         125 |    24.3 KB |
| duramen         |    830.5 ns |           2 |      288 B |
| cedar (serde)   |    36.73 µs |         182 |   35.52 KB |
| duramen (serde) |    3.209 µs |          67 |   11.32 KB |
| duramen (facet) |    4.028 µs |          38 |   17.33 KB |
| cedar (prost)   |    32.94 µs |         141 |    28.4 KB |
| duramen (prost) |    1.929 µs |          20 |   3.917 KB |

#### example_2c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    39.06 µs |         148 |   31.19 KB |
| duramen         |    1.012 µs |           2 |      288 B |
| cedar (serde)   |    47.37 µs |         236 |    49.7 KB |
| duramen (serde) |      4.1 µs |          88 |   13.38 KB |
| duramen (facet) |    5.014 µs |          50 |   20.79 KB |
| cedar (prost)   |    40.86 µs |         170 |   35.63 KB |
| duramen (prost) |    2.542 µs |          30 |   5.531 KB |

#### example_3a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.59 µs |         120 |   22.31 KB |
| duramen         |      701 ns |           2 |      288 B |
| cedar (serde)   |    33.16 µs |         167 |   31.46 KB |
| duramen (serde) |    2.762 µs |          57 |   10.87 KB |
| duramen (facet) |    3.577 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.91 µs |         134 |   26.36 KB |
| duramen (prost) |    1.518 µs |          16 |   3.761 KB |

#### example_3b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.18 µs |         148 |   31.18 KB |
| duramen         |    1.012 µs |           2 |      288 B |
| cedar (serde)   |    46.54 µs |         236 |   49.69 KB |
| duramen (serde) |    4.101 µs |          88 |   13.38 KB |
| duramen (facet) |    4.973 µs |          50 |   20.79 KB |
| cedar (prost)   |    40.36 µs |         170 |   35.63 KB |
| duramen (prost) |    2.584 µs |          30 |   5.536 KB |

#### example_3c

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    29.01 µs |         120 |   22.32 KB |
| duramen         |      701 ns |           2 |      288 B |
| cedar (serde)   |    33.37 µs |         167 |   31.48 KB |
| duramen (serde) |    2.783 µs |          57 |   10.87 KB |
| duramen (facet) |    3.577 µs |          33 |   16.18 KB |
| cedar (prost)   |    30.87 µs |         134 |   26.37 KB |
| duramen (prost) |    1.528 µs |          16 |   3.761 KB |

#### example_4a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    48.48 µs |         181 |   34.04 KB |
| duramen         |    1.382 µs |           6 |      416 B |
| cedar (serde)   |    59.27 µs |         327 |   58.43 KB |
| duramen (serde) |    6.575 µs |         146 |   16.32 KB |
| duramen (facet) |    7.968 µs |          89 |   35.43 KB |
| cedar (prost)   |    51.06 µs |         217 |   39.39 KB |
| duramen (prost) |    3.443 µs |          44 |    6.35 KB |

#### example_4d

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    34.57 µs |         136 |   24.17 KB |
| duramen         |    1.082 µs |           5 |      384 B |
| cedar (serde)   |       41 µs |         235 |   38.65 KB |
| duramen (serde) |    4.864 µs |         113 |   14.59 KB |
| duramen (facet) |    6.281 µs |          71 |   30.73 KB |
| cedar (prost)   |    36.87 µs |         161 |   28.94 KB |
| duramen (prost) |     2.17 µs |          28 |   5.371 KB |

#### example_4e

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    36.36 µs |         138 |   24.44 KB |
| duramen         |    1.176 µs |           4 |      352 B |
| cedar (serde)   |    43.24 µs |         241 |   39.24 KB |
| duramen (serde) |    5.275 µs |         117 |   14.73 KB |
| duramen (facet) |    6.571 µs |          71 |   30.65 KB |
| cedar (prost)   |     38.7 µs |         165 |   29.21 KB |
| duramen (prost) |      2.5 µs |          30 |   5.391 KB |

#### example_4f

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |       42 µs |         156 |   30.32 KB |
| duramen         |    1.453 µs |           8 |      480 B |
| cedar (serde)   |    51.55 µs |         298 |    53.1 KB |
| duramen (serde) |    6.816 µs |         154 |   16.59 KB |
| duramen (facet) |    8.457 µs |          98 |   39.81 KB |
| cedar (prost)   |    44.59 µs |         190 |   35.64 KB |
| duramen (prost) |    3.155 µs |          40 |   5.975 KB |

#### example_5b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    63.09 µs |         223 |   46.11 KB |
| duramen         |    2.514 µs |          16 |      736 B |
| cedar (serde)   |    83.45 µs |         521 |   93.76 KB |
| duramen (serde) |    14.55 µs |         308 |   26.24 KB |
| duramen (facet) |    16.42 µs |         195 |   74.65 KB |
| cedar (prost)   |    68.26 µs |         294 |    53.4 KB |
| duramen (prost) |    6.804 µs |          91 |   10.38 KB |

#### ip_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    36.55 µs |         149 |   27.71 KB |
| duramen         |    1.019 µs |           5 |      384 B |
| cedar (serde)   |    44.15 µs |         244 |   44.22 KB |
| duramen (serde) |    4.497 µs |         106 |   14.69 KB |
| duramen (facet) |    5.747 µs |          64 |   25.91 KB |
| cedar (prost)   |    38.99 µs |         172 |   32.29 KB |
| duramen (prost) |    2.541 µs |          31 |   5.462 KB |

#### ip_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    41.75 µs |         162 |   31.06 KB |
| duramen         |    1.338 µs |           8 |      480 B |
| cedar (serde)   |    51.39 µs |         287 |    53.6 KB |
| duramen (serde) |    6.125 µs |         139 |   16.79 KB |
| duramen (facet) |    7.389 µs |          86 |   33.31 KB |
| cedar (prost)   |    44.38 µs |         193 |   36.09 KB |
| duramen (prost) |    3.278 µs |          42 |    5.98 KB |

#### ip_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    38.55 µs |         153 |   29.11 KB |
| duramen         |    1.051 µs |           5 |      384 B |
| cedar (serde)   |    46.53 µs |         247 |   46.96 KB |
| duramen (serde) |     4.54 µs |         104 |   15.07 KB |
| duramen (facet) |    5.749 µs |          62 |   24.31 KB |
| cedar (prost)   |    40.78 µs |         175 |    33.7 KB |
| duramen (prost) |    2.666 µs |          31 |   5.446 KB |

#### multi_1

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |     45.1 µs |         177 |   37.46 KB |
| duramen         |    1.543 µs |           2 |      288 B |
| cedar (serde)   |    56.66 µs |         308 |   63.14 KB |
| duramen (serde) |    6.712 µs |         135 |    15.5 KB |
| duramen (facet) |    7.902 µs |          73 |   27.39 KB |
| cedar (prost)   |    47.32 µs |         207 |   42.45 KB |
| duramen (prost) |    3.904 µs |          40 |    5.89 KB |

#### multi_2

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    40.21 µs |         161 |   31.19 KB |
| duramen         |    1.428 µs |           2 |      288 B |
| cedar (serde)   |    48.51 µs |         262 |    50.1 KB |
| duramen (serde) |    15.81 µs |         114 |   14.48 KB |
| duramen (facet) |    6.956 µs |          62 |      25 KB |
| cedar (prost)   |     42.3 µs |         185 |   35.95 KB |
| duramen (prost) |    3.248 µs |          31 |   5.304 KB |

#### multi_3

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    52.57 µs |         204 |   36.16 KB |
| duramen         |    1.819 µs |           6 |      416 B |
| cedar (serde)   |    65.66 µs |         380 |   63.93 KB |
| duramen (serde) |    25.27 µs |         192 |   20.36 KB |
| duramen (facet) |    10.73 µs |         113 |   45.11 KB |
| cedar (prost)   |    55.52 µs |         247 |    42.1 KB |
| duramen (prost) |    4.433 µs |          52 |   8.404 KB |

#### multi_4

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    69.94 µs |         279 |   49.52 KB |
| duramen         |    3.021 µs |          10 |      544 B |
| cedar (serde)   |    92.34 µs |         582 |   98.42 KB |
| duramen (serde) |     43.4 µs |         326 |    30.8 KB |
| duramen (facet) |     18.3 µs |         189 |   73.76 KB |
| cedar (prost)   |    74.82 µs |         344 |   57.51 KB |
| duramen (prost) |    7.194 µs |          77 |   13.49 KB |

#### multi_5

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    56.17 µs |         226 |   37.49 KB |
| duramen         |    2.109 µs |           7 |      448 B |
| cedar (serde)   |    69.41 µs |         418 |   66.98 KB |
| duramen (serde) |    28.45 µs |         212 |    25.3 KB |
| duramen (facet) |    12.25 µs |         123 |   52.38 KB |
| cedar (prost)   |    59.83 µs |         268 |   43.76 KB |
| duramen (prost) |    4.604 µs |          48 |   12.17 KB |

#### parser_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    738.5 µs |        3534 |   722.9 KB |
| duramen         |    49.89 µs |         149 |   6.308 KB |
| cedar (serde)   |    736.7 µs |        3534 |   722.9 KB |
| duramen (serde) |    132.8 µs |         149 |   6.308 KB |
| duramen (facet) |    49.73 µs |         149 |   6.308 KB |
| cedar (prost)   |    736.9 µs |        3534 |   722.9 KB |
| duramen (prost) |    49.68 µs |         149 |   6.308 KB |

#### corpus_502da

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    72.33 µs |         224 |   73.29 KB |
| duramen         |    5.889 µs |          18 |      800 B |
| cedar (serde)   |    101.4 µs |         441 |   131.5 KB |
| duramen (serde) |    19.13 µs |         206 |    27.9 KB |
| duramen (facet) |    22.36 µs |         139 |   54.89 KB |
| cedar (prost)   |    75.57 µs |         267 |   83.66 KB |
| duramen (prost) |    15.03 µs |          73 |   16.72 KB |

#### corpus_c7e64

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    89.87 µs |         239 |   53.75 KB |
| duramen         |    5.683 µs |          14 |      672 B |
| cedar (serde)   |    120.8 µs |         538 |     108 KB |
| duramen (serde) |    17.34 µs |         264 |   34.44 KB |
| duramen (facet) |    20.85 µs |         172 |   66.14 KB |
| cedar (prost)   |     97.5 µs |         378 |   68.36 KB |
| duramen (prost) |    16.83 µs |         189 |   22.98 KB |

#### corpus_f4174

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    93.87 µs |         238 |   54.77 KB |
| duramen         |    6.099 µs |          14 |      672 B |
| cedar (serde)   |    124.9 µs |         537 |   110.3 KB |
| duramen (serde) |    18.05 µs |         263 |   35.57 KB |
| duramen (facet) |    21.73 µs |         171 |   66.85 KB |
| cedar (prost)   |    99.88 µs |         377 |   70.36 KB |
| duramen (prost) |    17.37 µs |         188 |      24 KB |

### Schema

#### sandbox_a

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    394.3 µs |        2232 |   716.6 KB |
| duramen         |    4.571 µs |           2 |      288 B |
| cedar (serde)   |    461.4 µs |        2941 |   920.4 KB |
| duramen (serde) |    25.86 µs |         543 |    63.1 KB |
| duramen (facet) |    30.19 µs |         323 |   96.35 KB |

#### sandbox_b

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |      430 µs |        2454 |   785.3 KB |
| duramen         |    5.238 µs |           2 |      288 B |
| cedar (serde)   |    502.4 µs |        3259 |   1.034 MB |
| duramen (serde) |    33.12 µs |         643 |   82.48 KB |
| duramen (facet) |    34.74 µs |         386 |   123.6 KB |

#### sandbox_b_exts

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    485.8 µs |        2783 |   893.4 KB |
| duramen         |    5.904 µs |           2 |      288 B |
| cedar (serde)   |    561.7 µs |        3611 |   1.147 MB |
| duramen (serde) |    34.71 µs |         673 |   84.88 KB |
| duramen (facet) |    36.51 µs |         406 |   127.4 KB |

#### validator_testfile

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    266.1 µs |        1465 |     459 KB |
| duramen         |    3.155 µs |           2 |      288 B |
| cedar (serde)   |    292.7 µs |        1839 |   595.6 KB |
| duramen (serde) |     16.3 µs |         306 |   51.96 KB |
| duramen (facet) |    17.46 µs |         194 |   71.18 KB |

#### corpus_011ec

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    72.56 ms |      465123 |   156.7 MB |
| duramen         |    385.7 µs |           2 |      288 B |
| cedar (serde)   |    74.48 ms |      479176 |   159.1 MB |
| duramen (serde) |    1.074 ms |       14011 |   1.227 MB |
| duramen (facet) |    1.062 ms |        8916 |   2.093 MB |

#### corpus_37250

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    16.68 ms |       97157 |   32.03 MB |
| duramen         |    212.5 µs |           2 |      288 B |
| cedar (serde)   |    17.99 ms |      116265 |   35.15 MB |
| duramen (serde) |    1.219 ms |       26500 |   1.626 MB |
| duramen (facet) |    1.209 ms |       16692 |   3.396 MB |

#### corpus_bd2fe

| Implementation  |        Time | Alloc Count | Alloc Size |
|-----------------|-------------|-------------|------------|
| cedar           |    19.58 ms |      113977 |   37.63 MB |
| duramen         |    253.6 µs |           2 |      288 B |
| cedar (serde)   |    21.08 ms |      136078 |   41.18 MB |
| duramen (serde) |    1.429 ms |       32166 |     1.7 MB |
| duramen (facet) |    1.437 ms |       20665 |   3.789 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
