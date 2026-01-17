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

#### [decimal_1](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/decimal/policies_1.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    38.47 µs |      150 |      28 |   121.7 KB |
| duramen         |    915.5 ns |        5 |       8 |   4.704 KB |
| cedar (serde)   |    46.07 µs |      244 |      28 |   139.5 KB |
| duramen (serde) |    4.372 µs |      104 |       9 |   19.58 KB |
| duramen (facet) |    5.457 µs |       62 |      18 |   30.22 KB |
| cedar (prost)   |    40.53 µs |      172 |      28 |   126.3 KB |
| duramen (prost) |    2.457 µs |       31 |       9 |   10.01 KB |

#### [decimal_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/decimal/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    45.32 µs |      174 |      29 |   128.5 KB |
| duramen         |    1.307 µs |        8 |       9 |   8.384 KB |
| cedar (serde)   |    56.61 µs |      319 |      29 |   155.5 KB |
| duramen (serde) |    6.942 µs |      151 |      11 |   26.67 KB |
| duramen (facet) |    7.878 µs |       92 |      21 |   42.75 KB |
| cedar (prost)   |    47.78 µs |      207 |      29 |   133.8 KB |
| duramen (prost) |    3.728 µs |       48 |      11 |    14.7 KB |

#### [example_1a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_1a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    29.42 µs |      120 |      22 |   70.38 KB |
| duramen         |    684.6 ns |        2 |       8 |   4.608 KB |
| cedar (serde)   |    34.03 µs |      175 |      22 |   81.43 KB |
| duramen (serde) |    3.124 µs |       67 |       8 |   15.65 KB |
| duramen (facet) |    3.819 µs |       38 |      16 |   23.03 KB |
| cedar (prost)   |    30.66 µs |      136 |      22 |    74.5 KB |
| duramen (prost) |    1.762 µs |       20 |       8 |   8.249 KB |

#### [example_2a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |       32 µs |      125 |      24 |   104.4 KB |
| duramen         |    689.9 ns |        2 |       8 |   4.608 KB |
| cedar (serde)   |    37.18 µs |      182 |      24 |   115.7 KB |
| duramen (serde) |    3.041 µs |       67 |       8 |   15.67 KB |
| duramen (facet) |    3.902 µs |       38 |      16 |   23.04 KB |
| cedar (prost)   |    33.61 µs |      141 |      24 |   108.6 KB |
| duramen (prost) |    1.773 µs |       20 |       8 |   8.273 KB |

#### [example_2b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    31.42 µs |      125 |      24 |   104.4 KB |
| duramen         |    684.6 ns |        2 |       8 |   4.608 KB |
| cedar (serde)   |    36.43 µs |      182 |      24 |   115.6 KB |
| duramen (serde) |    3.082 µs |       67 |       8 |   15.64 KB |
| duramen (facet) |     3.86 µs |       38 |      16 |   23.02 KB |
| cedar (prost)   |    33.15 µs |      141 |      24 |   108.5 KB |
| duramen (prost) |    1.773 µs |       20 |       8 |   8.237 KB |

#### [example_2c](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2c.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |     39.1 µs |      148 |      27 |   123.7 KB |
| duramen         |    820.1 ns |        2 |       8 |   4.608 KB |
| cedar (serde)   |    47.37 µs |      236 |      27 |   142.2 KB |
| duramen (serde) |    3.934 µs |       88 |       8 |    17.7 KB |
| duramen (facet) |    4.764 µs |       50 |      16 |   26.49 KB |
| cedar (prost)   |    40.95 µs |      170 |      27 |   128.1 KB |
| duramen (prost) |    2.293 µs |       30 |       8 |   9.851 KB |

#### [example_3a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    29.21 µs |      120 |      24 |   102.4 KB |
| duramen         |    591.5 ns |        2 |       7 |   4.096 KB |
| cedar (serde)   |    33.28 µs |      167 |      24 |   111.6 KB |
| duramen (serde) |    2.658 µs |       57 |       7 |   14.68 KB |
| duramen (facet) |     3.41 µs |       33 |      14 |    21.1 KB |
| cedar (prost)   |    30.82 µs |      134 |      24 |   106.5 KB |
| duramen (prost) |    1.413 µs |       16 |       7 |   7.569 KB |

#### [example_3b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    37.85 µs |      148 |      27 |   123.7 KB |
| duramen         |    820.1 ns |        2 |       8 |   4.608 KB |
| cedar (serde)   |    46.66 µs |      236 |      27 |   142.2 KB |
| duramen (serde) |    3.892 µs |       88 |       8 |    17.7 KB |
| duramen (facet) |    4.681 µs |       50 |      16 |   26.49 KB |
| cedar (prost)   |    40.11 µs |      170 |      27 |   128.1 KB |
| duramen (prost) |    2.334 µs |       30 |       8 |   9.856 KB |

#### [example_3c](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3c.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    28.96 µs |      120 |      24 |   102.4 KB |
| duramen         |    591.5 ns |        2 |       7 |   4.096 KB |
| cedar (serde)   |    33.37 µs |      167 |      24 |   111.6 KB |
| duramen (serde) |    2.678 µs |       57 |       7 |   14.68 KB |
| duramen (facet) |     3.41 µs |       33 |      14 |    21.1 KB |
| cedar (prost)   |    30.74 µs |      134 |      24 |   106.5 KB |
| duramen (prost) |    1.424 µs |       16 |       7 |   7.569 KB |

#### [example_4a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    48.18 µs |      181 |      28 |   126.6 KB |
| duramen         |    1.174 µs |        6 |       9 |    8.32 KB |
| cedar (serde)   |    58.85 µs |      327 |      28 |     151 KB |
| duramen (serde) |    6.408 µs |      146 |       9 |   24.22 KB |
| duramen (facet) |    7.633 µs |       89 |      19 |   45.24 KB |
| cedar (prost)   |    51.02 µs |      217 |      28 |   131.9 KB |
| duramen (prost) |    3.233 µs |       44 |       9 |   14.25 KB |

#### [example_4d](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4d.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    34.61 µs |      136 |      28 |   116.9 KB |
| duramen         |    905.3 ns |        5 |       8 |   4.704 KB |
| cedar (serde)   |    40.87 µs |      235 |      28 |   131.3 KB |
| duramen (serde) |     4.78 µs |      113 |       8 |   18.91 KB |
| duramen (facet) |     6.03 µs |       71 |      17 |   36.45 KB |
| cedar (prost)   |     36.5 µs |      161 |      28 |   121.6 KB |
| duramen (prost) |    2.002 µs |       28 |       8 |   9.691 KB |

#### [example_4e](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4e.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    36.52 µs |      138 |      28 |   117.1 KB |
| duramen         |    999.3 ns |        4 |       9 |   8.256 KB |
| cedar (serde)   |    43.11 µs |      241 |      28 |   131.9 KB |
| duramen (serde) |    5.065 µs |      117 |       9 |   22.64 KB |
| duramen (facet) |    6.363 µs |       71 |      18 |   39.95 KB |
| cedar (prost)   |     38.7 µs |      165 |      28 |   121.9 KB |
| duramen (prost) |    2.292 µs |       30 |       9 |   13.29 KB |

#### [example_4f](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4f.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    42.34 µs |      156 |      29 |   123.3 KB |
| duramen         |    1.255 µs |        8 |       9 |   8.384 KB |
| cedar (serde)   |    51.42 µs |      298 |      29 |   146.1 KB |
| duramen (serde) |     6.73 µs |      154 |       9 |   24.49 KB |
| duramen (facet) |    8.207 µs |       98 |      19 |   49.62 KB |
| cedar (prost)   |    44.59 µs |      190 |      29 |   128.6 KB |
| duramen (prost) |    2.905 µs |       40 |       9 |   13.87 KB |

#### [example_5b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_5b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    62.92 µs |      223 |      36 |     166 KB |
| duramen         |    2.181 µs |       16 |      11 |   16.83 KB |
| cedar (serde)   |    84.12 µs |      521 |      36 |   213.6 KB |
| duramen (serde) |    13.25 µs |      308 |      11 |   42.33 KB |
| duramen (facet) |    15.92 µs |      195 |      22 |   92.68 KB |
| cedar (prost)   |    68.42 µs |      294 |      36 |   173.2 KB |
| duramen (prost) |    6.511 µs |       91 |      11 |   26.47 KB |

#### [ip_1](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_1.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    36.85 µs |      149 |      27 |   120.1 KB |
| duramen         |    873.9 ns |        5 |       8 |   4.704 KB |
| cedar (serde)   |    44.24 µs |      244 |      27 |   136.6 KB |
| duramen (serde) |    4.371 µs |      106 |       8 |   19.01 KB |
| duramen (facet) |     5.58 µs |       64 |      17 |   31.62 KB |
| cedar (prost)   |    39.07 µs |      172 |      27 |   124.7 KB |
| duramen (prost) |    2.291 µs |       31 |       8 |   9.782 KB |

#### [ip_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    42.04 µs |      162 |      29 |     124 KB |
| duramen         |    1.203 µs |        8 |       9 |   8.384 KB |
| cedar (serde)   |    51.31 µs |      287 |      29 |   146.6 KB |
| duramen (serde) |    6.165 µs |      139 |       9 |   24.69 KB |
| duramen (facet) |     7.22 µs |       86 |      18 |   42.61 KB |
| cedar (prost)   |    44.29 µs |      193 |      29 |   129.1 KB |
| duramen (prost) |     3.11 µs |       42 |       9 |   13.88 KB |

#### [ip_3](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_3.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    38.97 µs |      153 |      28 |   121.8 KB |
| duramen         |    910.4 ns |        5 |       8 |   4.704 KB |
| cedar (serde)   |    46.61 µs |      247 |      28 |   139.7 KB |
| duramen (serde) |    4.372 µs |      104 |       9 |   19.58 KB |
| duramen (facet) |    5.456 µs |       62 |      18 |   30.21 KB |
| cedar (prost)   |    40.86 µs |      175 |      28 |   126.4 KB |
| duramen (prost) |    2.478 µs |       31 |       9 |      10 KB |

#### [multi_1](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_1.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    44.85 µs |      177 |      29 |   132.6 KB |
| duramen         |    1.261 µs |        2 |      10 |   9.216 KB |
| cedar (serde)   |    56.37 µs |      308 |      29 |   158.3 KB |
| duramen (serde) |    6.294 µs |      135 |      10 |   24.42 KB |
| duramen (facet) |    7.318 µs |       73 |      20 |   39.07 KB |
| cedar (prost)   |    47.44 µs |      207 |      29 |   137.6 KB |
| duramen (prost) |    3.529 µs |       40 |      10 |   14.81 KB |

#### [multi_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    40.04 µs |      161 |      28 |   126.2 KB |
| duramen         |    1.136 µs |        2 |      10 |   9.216 KB |
| cedar (serde)   |    48.51 µs |      262 |      28 |   145.1 KB |
| duramen (serde) |    5.402 µs |      114 |      10 |   23.41 KB |
| duramen (facet) |    6.457 µs |       62 |      19 |   36.16 KB |
| cedar (prost)   |    42.47 µs |      185 |      28 |     131 KB |
| duramen (prost) |    2.873 µs |       31 |      10 |   14.23 KB |

#### [multi_3](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_3.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |     52.4 µs |      204 |      32 |     132 KB |
| duramen         |    1.527 µs |        6 |      10 |   9.344 KB |
| cedar (serde)   |    65.49 µs |      380 |      32 |   159.7 KB |
| duramen (serde) |    8.562 µs |      192 |      10 |   29.29 KB |
| duramen (facet) |    10.14 µs |      113 |      21 |   56.81 KB |
| cedar (prost)   |     55.6 µs |      247 |      32 |   137.9 KB |
| duramen (prost) |    4.098 µs |       52 |      10 |   17.33 KB |

#### [multi_4](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_4.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    69.94 µs |      279 |      38 |   149.8 KB |
| duramen         |    2.521 µs |       10 |      12 |   18.68 KB |
| cedar (serde)   |    91.96 µs |      582 |      38 |   198.7 KB |
| duramen (serde) |    15.03 µs |      326 |      12 |   48.95 KB |
| duramen (facet) |    17.55 µs |      189 |      26 |   97.43 KB |
| cedar (prost)   |    75.49 µs |      344 |      38 |   157.7 KB |
| duramen (prost) |    6.652 µs |       77 |      12 |   31.63 KB |

#### [multi_5](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_5.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    56.17 µs |      226 |      34 |   134.8 KB |
| duramen         |    1.817 µs |        7 |      11 |   16.54 KB |
| cedar (serde)   |    70.15 µs |      418 |      34 |   164.3 KB |
| duramen (serde) |    9.788 µs |      212 |      11 |    41.4 KB |
| duramen (facet) |    11.75 µs |      123 |      23 |   72.11 KB |
| cedar (prost)   |    60.08 µs |      268 |      34 |   141.1 KB |
| duramen (prost) |    4.394 µs |       48 |      11 |   28.27 KB |

#### [parser_testfile](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/testfiles/policies.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    740.6 µs |     3534 |     516 |    1.08 MB |
| duramen         |    41.39 µs |      149 |      21 |   301.4 KB |
| cedar (serde)   |    738.1 µs |     3534 |     516 |    1.08 MB |
| duramen (serde) |    41.27 µs |      149 |      21 |   301.4 KB |
| duramen (facet) |    41.39 µs |      149 |      21 |   301.4 KB |
| cedar (prost)   |    740.6 µs |     3534 |     516 |    1.08 MB |
| duramen (prost) |    41.35 µs |      149 |      21 |   301.4 KB |

#### corpus_502da

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    71.75 µs |      224 |      45 |   190.8 KB |
| duramen         |    5.347 µs |       18 |      10 |   9.728 KB |
| cedar (serde)   |    101.2 µs |      441 |      61 |   249.5 KB |
| duramen (serde) |    18.42 µs |      206 |      10 |   36.83 KB |
| duramen (facet) |    21.28 µs |      139 |      22 |   68.79 KB |
| cedar (prost)   |    76.11 µs |      267 |      45 |   201.1 KB |
| duramen (prost) |    14.11 µs |       73 |      10 |   25.64 KB |

#### corpus_c7e64

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    90.29 µs |      239 |      86 |     181 KB |
| duramen         |    4.933 µs |       14 |      13 |   33.15 KB |
| cedar (serde)   |    120.9 µs |      538 |     134 |   237.9 KB |
| duramen (serde) |    16.05 µs |      254 |      31 |   63.37 KB |
| duramen (facet) |    19.49 µs |      162 |      43 |     100 KB |
| cedar (prost)   |    96.83 µs |      378 |      86 |   195.6 KB |
| duramen (prost) |    15.63 µs |      179 |      49 |   53.63 KB |

#### corpus_f4174

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    93.66 µs |      238 |      87 |   182.5 KB |
| duramen         |    5.349 µs |       14 |      14 |   37.24 KB |
| cedar (serde)   |    126.4 µs |      537 |     135 |   240.7 KB |
| duramen (serde) |    17.26 µs |      254 |      32 |   68.27 KB |
| duramen (facet) |    20.61 µs |      162 |      44 |   104.5 KB |
| cedar (prost)   |    100.5 µs |      377 |      87 |     198 KB |
| duramen (prost) |    16.34 µs |      179 |      50 |   58.43 KB |

### Schema

#### [sandbox_a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_a/schema.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    395.9 µs |     2232 |     987 |   1.244 MB |
| duramen         |    4.071 µs |        2 |      14 |   36.86 KB |
| cedar (serde)   |    458.2 µs |     2941 |    1054 |   1.479 MB |
| duramen (serde) |    25.35 µs |      543 |      14 |   99.67 KB |
| duramen (facet) |    29.27 µs |      323 |      24 |     137 KB |

#### [sandbox_b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_b/schema.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    428.6 µs |     2454 |    1067 |   1.336 MB |
| duramen         |    4.571 µs |        2 |      14 |   36.86 KB |
| cedar (serde)   |    498.1 µs |     3259 |    1139 |   1.624 MB |
| duramen (serde) |    30.07 µs |      643 |      14 |     119 KB |
| duramen (facet) |    33.57 µs |      386 |      25 |   164.3 KB |

#### [sandbox_b_exts](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_b/schema_exts.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    486.7 µs |     2783 |    1216 |   1.515 MB |
| duramen         |    5.154 µs |        2 |      14 |   36.86 KB |
| cedar (serde)   |    558.6 µs |     3611 |    1291 |   1.811 MB |
| duramen (serde) |    32.08 µs |      673 |      14 |   121.4 KB |
| duramen (facet) |    35.72 µs |      406 |      25 |   168.1 KB |

#### [validator_testfile](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/validator/cedar_schema/testfiles/example.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    262.4 µs |     1465 |     624 |     805 KB |
| duramen         |    2.656 µs |        2 |      12 |   18.43 KB |
| cedar (serde)   |      291 µs |     1839 |     633 |   944.7 KB |
| duramen (serde) |    14.92 µs |      306 |      12 |   69.29 KB |
| duramen (facet) |    16.92 µs |      194 |      22 |   90.58 KB |

#### corpus_011ec

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    72.35 ms |   465123 |  216675 |   250.4 MB |
| duramen         |    327.1 µs |        2 |      26 |   2.359 MB |
| cedar (serde)   |    74.38 ms |   477968 |  219492 |   252.8 MB |
| duramen (serde) |    1.008 ms |    13317 |    2515 |   3.856 MB |
| duramen (facet) |    992.7 µs |     8222 |    2531 |   4.869 MB |

#### corpus_37250

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    16.59 ms |    97157 |   44429 |   51.33 MB |
| duramen         |    164.6 µs |        2 |      24 |   1.179 MB |
| cedar (serde)   |    18.02 ms |   116265 |   45092 |   54.94 MB |
| duramen (serde) |    1.142 ms |    25442 |     245 |   2.758 MB |
| duramen (facet) |    1.132 ms |    15634 |     261 |    4.66 MB |

#### corpus_bd2fe

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    19.49 ms |   113977 |   51565 |   60.56 MB |
| duramen         |    195.6 µs |        2 |      25 |   2.097 MB |
| cedar (serde)   |    21.01 ms |   136078 |   52263 |   64.63 MB |
| duramen (serde) |    1.368 ms |    29724 |     266 |    3.89 MB |
| duramen (facet) |    1.345 ms |    18223 |     282 |    6.11 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
