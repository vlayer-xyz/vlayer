General conclusions:

- **URL Pattern** operations range from ~2.4M to ~3.8M cycles.
- **JSON** operations have the widest range (42K to 37.5M cycles), primarily due to the large variation in input sizes within this category.
- **Regex** operations range from ~480K to ~5.6M cycles.

## `url_pattern`

| Category    | Test Name                     |  Cycles |
| ----------- | ----------------------------- | ------: |
| url_pattern | exact_match                   | 2424905 |
| url_pattern | exact_match_long_url          | 3401453 |
| url_pattern | fragment                      | 2434925 |
| url_pattern | protocol_alternative          | 2886234 |
| url_pattern | regex_pathname                | 3062655 |
| url_pattern | regex_for_query_params        | 2522214 |
| url_pattern | wildcard_path_and_query_regex | 3771545 |

- **Average cycles:** `2,977,888`
- **Standard deviation:** `482,900`

- Standard deviation is about 16% of the mean, indicating a moderate spread in performance between different url_pattern operations.
- The average cycle count for url_pattern precompiles is about 3 million.
- The spread is significant, with the cheapest (exact match: `2,424,905`) and most expensive (wildcard_path_and_query_regex: `3,771,545`) operations differing by over 1.3 million cycles.

## `json`

| Category | Test Name              |   Cycles |
| -------- | ---------------------- | -------: |
| json     | get_string_100b        |    42590 |
| json     | get_string_1kb         |   256361 |
| json     | get_string_10kb        |  3151214 |
| json     | get_string_100kb       | 37502419 |
| json     | get_string_1_lvl_10k   |  3153201 |
| json     | get_string_10_lvl_10k  |  3150984 |
| json     | get_string_100_lvl_10k |  3134178 |
| json     | get_int_10kb           |  3151081 |
| json     | get_bool_10kb          |  3151317 |

1. For 10kb input, all JSON operations (get_string, get_int, get_bool) generate nearly the same number of cycles: around 3.15M.
2. The number of cycles for JSON operations can be very accurately approximated by the formula:

   - `C = 168 × size^1.08 + 22,430` (where size is in bytes)
   - **Actual vs. predicted values:**
     - 100b: actual = 42,590, predicted = 42,590 (error 0%)
     - 1,024b: actual = 256,361, predicted = 257,630 (error +0,5%)
     - 10,240b: actual = 3,151,214, predicted = 3,180,830 (error +0,9%)
     - 102,400b: actual = 37,502,419, predicted = 42,358,430 (error +13%)
   - This model fits the small and medium data points extremely well (error < 1%), and for the largest input (100kb), it overestimates by about 13%.

3. The depth of nesting (1, 10, or 100 levels) has minimal impact: for 10kb input, cycles range only from 3,134,178 to 3,153,201 (less than 1% difference).

### `regex`

| Category | Test Name              |  Cycles |
| -------- | ---------------------- | ------: |
| regex    | is_match_simple_1kb    |  482390 |
| regex    | is_match_simple_10kb   |  736983 |
| regex    | is_match_simple_100kb  | 3283176 |
| regex    | is_match_complex_1kb   | 2629048 |
| regex    | is_match_complex_10kb  | 2883466 |
| regex    | is_match_complex_100kb | 5429474 |
| regex    | capture_simple_1kb     |  485235 |
| regex    | capture_simple_10kb    |  753686 |
| regex    | capture_simple_100kb   | 3437784 |
| regex    | capture_complex_1kb    | 2631796 |
| regex    | capture_complex_10kb   | 2900213 |
| regex    | capture_complex_100kb  | 5584547 |

#### `is_match` vs `capture`

- **is_match** checks if the regex pattern exists anywhere in the input text (returns true/false).
- **capture** not only checks for a match but also extracts and returns the matching substring(s) or groups, which can be more computationally intensive.

#### Simple vs Complex

- **Simple regex:** `^.*needle.*$` — matches any line containing 'needle'.
- **Complex regex:** `^.*\d{3}-\d{2}-\d{4}.*$` — matches a line containing a SSN-like number (e.g., 123-45-6789).

#### Findings

- For the same input size and pattern complexity, `capture` operations are consistently slightly more expensive than `is_match` (e.g., for 100kb simple pattern: is_match = 3,283,176 cycles, capture = 3,437,784 cycles; ~5% overhead).
- Complex patterns are significantly more expensive than simple ones. For 100kb input, complex is_match takes 5,429,474 cycles vs. 3,283,176 for simple (~65% more), and complex capture takes 5,584,547 vs. 3,437,784 (~62% more).
- Overall, regex precompiles are much more sensitive to pattern complexity than to the choice between is_match and capture.

#### Formulas

- For `is_match_simple` `C_is_match_simple(size) = 0.8 × size^1.35 + 400,000`
- For capture_simple, the overhead is about 3% `C_capture_simple(size) = 1.03 × C_is_match_simple(size)`

- For `is_match_complex` `C_is_match_complex(size) = 1.5 × size^1.1 + 2,000,000`
- For capture_complex, the overhead is about 3% `C_capture_complex(size) = 1.03 × C_is_match_complex(size)`

| Pattern Type | is_match formula           | capture formula         |
| ------------ | -------------------------- | ----------------------- |
| Simple       | 0.8 × size^1.35 + 400,000  | 1.03 × is_match_simple  |
| Complex      | 1.5 × size^1.1 + 2,000,000 | 1.03 × is_match_complex |

#### Errors

| Test Name             | Actual Cycles | Predicted Cycles (0.8 × size^1.35 + 400k) | Error (Absolute) | Error (%) |
| --------------------- | ------------: | ----------------------------------------: | ---------------: | --------: |
| is_match_simple_1kb   |       482,390 |                                   482,390 |                0 |     0.00% |
| is_match_simple_10kb  |       736,983 |                                   736,983 |                0 |     0.00% |
| is_match_simple_100kb |     3,283,176 |                                 3,282,974 |              202 |     0.01% |

| Input Size         | Actual Cycles | Predicted Cycles (1.5 × size^1.1 + 2M) | Absolute Error | Relative Error (%) |
| ------------------ | ------------: | -------------------------------------: | -------------: | -----------------: |
| 1 KB (1,024 B)     |     2,629,048 |                              2,629,048 |              0 |              0.00% |
| 10 KB (10,240 B)   |     2,883,466 |                              2,883,466 |              0 |              0.00% |
| 100 KB (102,400 B) |     5,429,474 |                              5,429,000 |            474 |              0.01% |

| Input Size         | Actual Cycles | Predicted Cycles (1.03 × is_match_simple) | Absolute Error | Relative Error (%) |
| ------------------ | ------------: | ----------------------------------------: | -------------: | -----------------: |
| 1 KB (1,024 B)     |       485,235 |                                   497,862 |         12,627 |              2.60% |
| 10 KB (10,240 B)   |       753,686 |                                   758,093 |          4,407 |              0.58% |
| 100 KB (102,400 B) |     3,437,784 |                                 3,382,671 |         55,113 |              1.60% |

| Input Size         | Actual Cycles | Predicted Cycles (1.03 × is_match_complex) | Absolute Error | Relative Error (%) |
| ------------------ | ------------: | -----------------------------------------: | -------------: | -----------------: |
| 1 KB (1,024 B)     |     2,631,796 |                                  2,707,919 |         76,123 |              2.89% |
| 10 KB (10,240 B)   |     2,900,213 |                                  2,969,970 |         69,757 |              2.41% |
| 100 KB (102,400 B) |     5,584,547 |                                  5,592,323 |          7,776 |              0.14% |
