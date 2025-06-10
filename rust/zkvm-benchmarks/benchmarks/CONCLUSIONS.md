| Category    | Test Name                     | Actual Cycles |
| ----------- | ----------------------------- | ------------: |
| url_pattern | exact_match                   |       2424905 |
| url_pattern | exact_match_long_url          |       3401453 |
| url_pattern | fragment                      |       2434925 |
| url_pattern | protocol_alternative          |       2886234 |
| url_pattern | regex_pathname                |       3062655 |
| url_pattern | regex_for_query_params        |       2522214 |
| url_pattern | wildcard_path_and_query_regex |       3771545 |
| json        | get_string_100b               |         42590 |
| json        | get_string_1kb                |        256361 |
| json        | get_string_10kb               |       3151214 |
| json        | get_string_100kb              |      37502419 |
| json        | get_string_1_lvl_10k          |       3153201 |
| json        | get_string_10_lvl_10k         |       3150984 |
| json        | get_string_100_lvl_10k        |       3134178 |
| json        | get_int_10kb                  |       3151081 |
| json        | get_bool_10kb                 |       3151317 |
| regex       | is_match_simple_1kb           |        482390 |
| regex       | is_match_simple_10kb          |        736983 |
| regex       | is_match_simple_100kb         |       3283176 |
| regex       | is_match_complex_1kb          |       2629048 |
| regex       | is_match_complex_10kb         |       2883466 |
| regex       | is_match_complex_100kb        |       5429474 |
| regex       | capture_simple_1kb            |        485235 |
| regex       | capture_simple_10kb           |        753686 |
| regex       | capture_simple_100kb          |       3437784 |
| regex       | capture_complex_1kb           |       2631796 |
| regex       | capture_complex_10kb          |       2900213 |
| regex       | capture_complex_100kb         |       5584547 |

## Key Findings

### Category Comparison

- **URL Pattern** operations range from ~2.4M to ~3.8M cycles.
- **JSON** operations have the widest range (42K to 37.5M cycles).
- **Regex** operations range from ~480K to ~5.6M cycles.

### Size Impact

- For **JSON** operations, cycles scale linearly with input size  
  (100b: 42K cycles → 100kb: `37.5M` cycles`).
- For **Regex**, complexity matters more than size:
  - Simple regex: ~500K cycles for 1kb → ~3.4M for 100kb.
  - Complex regex: ~2.6M cycles for 1kb → ~5.6M for 100kb.

### URL Pattern Observations

- The most expensive is `wildcard_path_and_query_regex` at ~3.77M cycles.
- Simple exact matches are cheapest at ~2.4M cycles.
- Regex-based patterns are consistently more expensive than exact matches.

### JSON Observations

- Depth of nesting (1 vs 10 vs 100 levels) has minimal impact (~3.13M–3.15M cycles).
- Data type (string vs int vs bool) has negligible difference at the same size.

---

## Regex Benchmark Patterns: Simple vs Complex

In these benchmarks, the distinction between "simple" and "complex" regex is based on the pattern used:

- **Simple regex**: The pattern is `^.*needle.*$`. This matches any line containing the literal substring `needle` anywhere. It uses basic regex features: start/end anchors, wildcard (`.`), and greedy quantifiers (`*`).

- **Complex regex**: The pattern is `^.*\d{3}-\d{2}-\d{4}.*$`. This matches any line containing a sequence like a US Social Security Number (three digits, a dash, two digits, a dash, four digits) anywhere in the text. It uses character classes (`\d`), quantifiers (`{n}`), and literal dashes, making it more computationally intensive for the regex engine to process, especially on large inputs.

Both patterns are anchored and use wildcards, but the complex pattern requires the engine to evaluate digit classes and specific numeric formats, which increases processing time compared to the simple literal substring match.

---

**Summary:**

- Input size has a dramatic impact on JSON operations.
- Regex complexity matters more than input size.
- URL pattern matching is consistently expensive, with regex patterns being most costly.
