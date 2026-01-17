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
| cedar           |    38.14 µs |      150 |      28 |   121.7 KB |
| duramen         |    626.1 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    46.07 µs |      244 |      28 |   139.5 KB |
| duramen (serde) |    3.965 µs |      100 |       5 |   16.93 KB |
| duramen (facet) |    5.173 µs |       58 |      14 |   27.56 KB |
| cedar (prost)   |    105.4 µs |      172 |      28 |   126.3 KB |
| duramen (prost) |    2.049 µs |       27 |       5 |   7.354 KB |

#### [decimal_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/decimal/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    45.19 µs |      174 |      29 |   128.5 KB |
| duramen         |    880.6 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |     56.4 µs |      319 |      29 |   155.5 KB |
| duramen (serde) |    5.954 µs |      144 |       7 |   22.38 KB |
| duramen (facet) |    7.265 µs |       85 |      17 |   38.46 KB |
| cedar (prost)   |    124.1 µs |      207 |      29 |   133.8 KB |
| duramen (prost) |    3.197 µs |       41 |       7 |   10.41 KB |

#### [example_1a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_1a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    29.63 µs |      120 |      22 |   70.38 KB |
| duramen         |      475 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    33.78 µs |      175 |      22 |   81.43 KB |
| duramen (serde) |    2.796 µs |       66 |       4 |   13.09 KB |
| duramen (facet) |    3.574 µs |       37 |      12 |   20.47 KB |
| cedar (prost)   |    78.49 µs |      136 |      22 |    74.5 KB |
| duramen (prost) |    1.485 µs |       19 |       4 |   5.689 KB |

#### [example_2a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    31.92 µs |      125 |      24 |   104.4 KB |
| duramen         |    485.4 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    37.48 µs |      182 |      24 |   115.7 KB |
| duramen (serde) |      2.9 µs |       66 |       4 |   13.11 KB |
| duramen (facet) |    3.616 µs |       37 |      12 |   20.48 KB |
| cedar (prost)   |    86.82 µs |      141 |      24 |   108.6 KB |
| duramen (prost) |    1.495 µs |       19 |       4 |   5.713 KB |

#### [example_2b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    31.38 µs |      125 |      24 |   104.4 KB |
| duramen         |      475 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |     36.4 µs |      182 |      24 |   115.6 KB |
| duramen (serde) |    2.921 µs |       66 |       4 |   13.08 KB |
| duramen (facet) |    3.657 µs |       37 |      12 |   20.46 KB |
| cedar (prost)   |    85.27 µs |      141 |      24 |   108.5 KB |
| duramen (prost) |    1.495 µs |       19 |       4 |   5.677 KB |

#### [example_2c](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_2c.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    39.18 µs |      148 |      27 |   123.7 KB |
| duramen         |    584.3 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    47.41 µs |      236 |      27 |   142.2 KB |
| duramen (serde) |    3.563 µs |       87 |       4 |   15.14 KB |
| duramen (facet) |    4.476 µs |       49 |      12 |   23.93 KB |
| cedar (prost)   |    105.4 µs |      170 |      27 |   128.1 KB |
| duramen (prost) |    2.005 µs |       29 |       4 |   7.291 KB |

#### [example_3a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    29.09 µs |      120 |      24 |   102.4 KB |
| duramen         |    428.1 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    33.49 µs |      167 |      24 |   111.6 KB |
| duramen (serde) |    2.432 µs |       56 |       4 |   12.63 KB |
| duramen (facet) |    3.205 µs |       32 |      11 |   19.06 KB |
| cedar (prost)   |     80.2 µs |      134 |      24 |   106.5 KB |
| duramen (prost) |    1.167 µs |       15 |       4 |   5.521 KB |

#### [example_3b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    38.14 µs |      148 |      27 |   123.7 KB |
| duramen         |    584.3 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    46.29 µs |      236 |      27 |   142.2 KB |
| duramen (serde) |    3.646 µs |       87 |       4 |   15.14 KB |
| duramen (facet) |    4.476 µs |       49 |      12 |   23.93 KB |
| cedar (prost)   |    103.7 µs |      170 |      27 |   128.1 KB |
| duramen (prost) |    2.026 µs |       29 |       4 |   7.296 KB |

#### [example_3c](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_3c.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |     28.8 µs |      120 |      24 |   102.4 KB |
| duramen         |    425.5 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    33.45 µs |      167 |      24 |   111.6 KB |
| duramen (serde) |    2.474 µs |       56 |       4 |   12.63 KB |
| duramen (facet) |    3.226 µs |       32 |      11 |   19.06 KB |
| cedar (prost)   |    79.41 µs |      134 |      24 |   106.5 KB |
| duramen (prost) |    1.188 µs |       15 |       4 |   5.521 KB |

#### [example_4a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4a.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    48.56 µs |      181 |      28 |   126.6 KB |
| duramen         |    812.9 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |     59.1 µs |      327 |      28 |     151 KB |
| duramen (serde) |    5.833 µs |      141 |       5 |      20 KB |
| duramen (facet) |    7.226 µs |       84 |      15 |   41.02 KB |
| cedar (prost)   |    52.56 µs |      217 |      28 |   131.9 KB |
| duramen (prost) |    2.784 µs |       39 |       5 |   10.03 KB |

#### [example_4d](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4d.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    34.65 µs |      136 |      28 |   116.9 KB |
| duramen         |    641.7 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    40.66 µs |      235 |      28 |   131.3 KB |
| duramen (serde) |    4.287 µs |      109 |       4 |   16.25 KB |
| duramen (facet) |    5.746 µs |       67 |      13 |   33.79 KB |
| cedar (prost)   |    36.62 µs |      161 |      28 |   121.6 KB |
| duramen (prost) |    1.635 µs |       24 |       4 |   7.035 KB |

#### [example_4e](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4e.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    36.61 µs |      138 |      28 |   117.1 KB |
| duramen         |    729.6 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |    42.95 µs |      241 |      28 |   131.9 KB |
| duramen (serde) |    4.615 µs |      114 |       5 |   18.48 KB |
| duramen (facet) |    5.954 µs |       68 |      14 |   35.79 KB |
| cedar (prost)   |    100.2 µs |      165 |      28 |   121.9 KB |
| duramen (prost) |    1.923 µs |       27 |       5 |   9.135 KB |

#### [example_4f](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_4f.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    42.34 µs |      156 |      29 |   123.3 KB |
| duramen         |    870.2 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |    51.34 µs |      298 |      29 |   146.1 KB |
| duramen (serde) |    5.994 µs |      147 |       5 |   20.21 KB |
| duramen (facet) |    7.676 µs |       91 |      15 |   45.33 KB |
| cedar (prost)   |    116.3 µs |      190 |      29 |   128.6 KB |
| duramen (prost) |    2.375 µs |       33 |       5 |   9.591 KB |

#### [example_5b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/example_use_cases/policies_5b.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    64.01 µs |      223 |      36 |     166 KB |
| duramen         |    1.432 µs |        1 |       6 |   8.192 KB |
| cedar (serde)   |    84.16 µs |      521 |      36 |   213.6 KB |
| duramen (serde) |    12.49 µs |      293 |       6 |   33.69 KB |
| duramen (facet) |    15.15 µs |      180 |      17 |   84.04 KB |
| cedar (prost)   |    181.6 µs |      294 |      36 |   173.2 KB |
| duramen (prost) |    5.574 µs |       76 |       6 |   17.83 KB |

#### [ip_1](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_1.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    36.89 µs |      149 |      27 |   120.1 KB |
| duramen         |    594.7 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    43.86 µs |      244 |      27 |   136.6 KB |
| duramen (serde) |    4.045 µs |      102 |       4 |   16.35 KB |
| duramen (facet) |    5.171 µs |       60 |      13 |   28.96 KB |
| cedar (prost)   |    101.5 µs |      172 |      27 |   124.7 KB |
| duramen (prost) |    1.965 µs |       27 |       4 |   7.126 KB |

#### [ip_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    41.75 µs |      162 |      29 |     124 KB |
| duramen         |    807.7 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |    51.35 µs |      287 |      29 |   146.6 KB |
| duramen (serde) |    5.344 µs |      132 |       5 |    20.4 KB |
| duramen (facet) |    6.649 µs |       79 |      14 |   38.32 KB |
| cedar (prost)   |    116.4 µs |      193 |      29 |   129.1 KB |
| duramen (prost) |    2.664 µs |       35 |       5 |   9.596 KB |

#### [ip_3](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/ip/policies_3.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    38.63 µs |      153 |      28 |   121.8 KB |
| duramen         |      626 ns |        1 |       4 |   2.048 KB |
| cedar (serde)   |    46.23 µs |      247 |      28 |   139.7 KB |
| duramen (serde) |    3.965 µs |      100 |       5 |   16.92 KB |
| duramen (facet) |    5.131 µs |       58 |      14 |   27.56 KB |
| cedar (prost)   |    105.3 µs |      175 |      28 |   126.4 KB |
| duramen (prost) |     2.09 µs |       27 |       5 |    7.35 KB |

#### [multi_1](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_1.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    45.15 µs |      177 |      29 |   132.6 KB |
| duramen         |    896.2 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |    56.66 µs |      308 |      29 |   158.3 KB |
| duramen (serde) |    5.842 µs |      134 |       5 |    19.3 KB |
| duramen (facet) |    6.948 µs |       72 |      15 |   33.95 KB |
| cedar (prost)   |    122.4 µs |      207 |      29 |   137.6 KB |
| duramen (prost) |    3.034 µs |       39 |       5 |   9.698 KB |

#### [multi_2](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_2.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    40.12 µs |      161 |      28 |   126.2 KB |
| duramen         |    807.7 ns |        1 |       5 |   4.096 KB |
| cedar (serde)   |    48.13 µs |      262 |      28 |   145.1 KB |
| duramen (serde) |    4.866 µs |      113 |       5 |   18.29 KB |
| duramen (facet) |    6.045 µs |       61 |      14 |   31.04 KB |
| cedar (prost)   |    109.2 µs |      185 |      28 |     131 KB |
| duramen (prost) |    2.503 µs |       30 |       5 |   9.112 KB |

#### [multi_3](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_3.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    52.36 µs |      204 |      32 |     132 KB |
| duramen         |    1.078 µs |        1 |       5 |   4.096 KB |
| cedar (serde)   |    65.08 µs |      380 |      32 |   159.7 KB |
| duramen (serde) |    7.821 µs |      187 |       5 |   24.04 KB |
| duramen (facet) |    9.614 µs |      108 |      16 |   51.56 KB |
| cedar (prost)   |    146.1 µs |      247 |      32 |   137.9 KB |
| duramen (prost) |    3.442 µs |       47 |       5 |   12.08 KB |

#### [multi_4](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_4.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    70.15 µs |      279 |      38 |   149.8 KB |
| duramen         |    1.754 µs |        1 |       6 |   8.192 KB |
| cedar (serde)   |    92.09 µs |      582 |      38 |   198.7 KB |
| duramen (serde) |    13.75 µs |      317 |       6 |   38.45 KB |
| duramen (facet) |    16.52 µs |      180 |      20 |   86.93 KB |
| cedar (prost)   |    196.4 µs |      344 |      38 |   157.7 KB |
| duramen (prost) |    5.583 µs |       68 |       6 |   21.13 KB |

#### [multi_5](https://github.com/cedar-policy/cedar-integration-tests/blob/main/tests/multi/policies_5.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    56.54 µs |      226 |      34 |   134.8 KB |
| duramen         |    1.234 µs |        1 |       6 |   8.192 KB |
| cedar (serde)   |    70.12 µs |      418 |      34 |   164.3 KB |
| duramen (serde) |    8.675 µs |      206 |       6 |   33.05 KB |
| duramen (facet) |    10.93 µs |      117 |      18 |   63.75 KB |
| cedar (prost)   |    156.2 µs |      268 |      34 |   141.1 KB |
| duramen (prost) |    3.572 µs |       42 |       6 |   19.92 KB |

#### [parser_testfile](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/parser/testfiles/policies.cedar)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |      740 µs |     3534 |     516 |    1.08 MB |
| duramen         |    29.98 µs |        9 |      11 |   133.1 KB |
| cedar (serde)   |    737.8 µs |     3534 |     516 |    1.08 MB |
| duramen (serde) |    29.52 µs |        9 |      11 |   133.1 KB |
| duramen (facet) |    29.56 µs |        9 |      11 |   133.1 KB |
| cedar (prost)   |    1.821 ms |     3534 |     516 |    1.08 MB |
| duramen (prost) |    29.52 µs |        9 |      11 |   133.1 KB |

#### corpus_502da

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    71.37 µs |      224 |      45 |   190.8 KB |
| duramen         |     4.62 µs |        1 |       5 |   4.096 KB |
| cedar (serde)   |    247.5 µs |      441 |      61 |   249.5 KB |
| duramen (serde) |    17.27 µs |      189 |       5 |   31.19 KB |
| duramen (facet) |     20.3 µs |      122 |      17 |   63.16 KB |
| cedar (prost)   |    76.91 µs |      267 |      45 |   201.1 KB |
| duramen (prost) |    13.18 µs |       56 |       5 |   20.01 KB |

#### corpus_c7e64

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |     89.2 µs |      239 |      86 |     181 KB |
| duramen         |    4.201 µs |        1 |       7 |   16.38 KB |
| cedar (serde)   |    121.8 µs |      538 |     134 |   237.9 KB |
| duramen (serde) |     15.2 µs |      241 |      25 |    46.6 KB |
| duramen (facet) |     18.3 µs |      149 |      37 |   83.27 KB |
| cedar (prost)   |    96.84 µs |      378 |      86 |   195.6 KB |
| duramen (prost) |    14.44 µs |      166 |      43 |   36.86 KB |

#### corpus_f4174

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    93.71 µs |      238 |      87 |   182.5 KB |
| duramen         |    4.534 µs |        1 |       7 |   16.38 KB |
| cedar (serde)   |    125.9 µs |      537 |     135 |   240.7 KB |
| duramen (serde) |     15.7 µs |      241 |      25 |   47.41 KB |
| duramen (facet) |    19.17 µs |      149 |      37 |   83.67 KB |
| cedar (prost)   |      253 µs |      377 |      87 |     198 KB |
| duramen (prost) |    15.11 µs |      166 |      43 |   37.57 KB |

### Schema

#### [sandbox_a](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_a/schema.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    398.6 µs |     2232 |     987 |   1.244 MB |
| duramen         |    3.035 µs |        1 |       7 |   16.38 KB |
| cedar (serde)   |    459.7 µs |     2941 |    1054 |   1.479 MB |
| duramen (serde) |    23.82 µs |      542 |       7 |   79.19 KB |
| duramen (facet) |    28.32 µs |      322 |      17 |   116.5 KB |

#### [sandbox_b](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_b/schema.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |      433 µs |     2454 |    1067 |   1.336 MB |
| duramen         |    3.451 µs |        1 |       7 |   16.38 KB |
| cedar (serde)   |    498.9 µs |     3259 |    1139 |   1.624 MB |
| duramen (serde) |    29.88 µs |      642 |       7 |   98.57 KB |
| duramen (facet) |       32 µs |      385 |      18 |   143.8 KB |

#### [sandbox_b_exts](https://github.com/cedar-policy/cedar-integration-tests/blob/main/sample-data/sandbox_b/schema_exts.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    492.5 µs |     2783 |    1216 |   1.515 MB |
| duramen         |    3.993 µs |        1 |       7 |   16.38 KB |
| cedar (serde)   |      558 µs |     3611 |    1291 |   1.811 MB |
| duramen (serde) |    30.97 µs |      672 |       7 |   100.9 KB |
| duramen (facet) |    33.81 µs |      405 |      18 |   147.6 KB |

#### [validator_testfile](https://github.com/cedar-policy/cedar/blob/v4.8.2/cedar-policy-core/src/validator/cedar_schema/testfiles/example.cedarschema)

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |      270 µs |     1465 |     624 |     805 KB |
| duramen         |    1.952 µs |        1 |       6 |   8.192 KB |
| cedar (serde)   |    291.2 µs |     1839 |     633 |   944.7 KB |
| duramen (serde) |    14.31 µs |      305 |       6 |   59.05 KB |
| duramen (facet) |    15.93 µs |      193 |      16 |   80.34 KB |

#### corpus_011ec

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    72.69 ms |   465123 |  216675 |   250.4 MB |
| duramen         |    298.4 µs |        1 |      13 |   1.048 MB |
| cedar (serde)   |    74.22 ms |   477968 |  219492 |   252.8 MB |
| duramen (serde) |    959.5 µs |    13316 |    2502 |   2.545 MB |
| duramen (facet) |    953.4 µs |     8221 |    2518 |   3.558 MB |

#### corpus_37250

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    16.71 ms |    97157 |   44429 |   51.33 MB |
| duramen         |    139.5 µs |        1 |      12 |   524.2 KB |
| cedar (serde)   |    18.04 ms |   116265 |   45092 |   54.94 MB |
| duramen (serde) |    1.097 ms |    25441 |     233 |   2.103 MB |
| duramen (facet) |    1.106 ms |    15633 |     249 |   4.005 MB |

#### corpus_bd2fe

| Implementation  |        Time |   Allocs |   Grows |     Memory |
|-----------------|-------------|----------|---------|------------|
| cedar           |    19.65 ms |   113977 |   51565 |   60.56 MB |
| duramen         |    171.6 µs |        1 |      13 |   1.048 MB |
| cedar (serde)   |    21.05 ms |   136078 |   52263 |   64.63 MB |
| duramen (serde) |    1.308 ms |    29723 |     254 |   2.841 MB |
| duramen (facet) |    1.315 ms |    18222 |     270 |   5.062 MB |

## License

`duramen` is licensed under the terms of both the [MIT License](LICENSE-MIT) and the [Apache License (Version 2.0)](LICENSE-APACHE).
