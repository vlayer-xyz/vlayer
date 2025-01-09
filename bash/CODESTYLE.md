# Bash codestyle

## Variable names

| **Type**    | **Scope** | **Convention** |
|-------------|-----------|----------------|
| Environment | Global    | `MY_VARIABLE`  |
| Global      | Global    | `_MY_VARIABLE` |
| Local       | Function  | `my_variable`  |

## Variables in strings
If it's a `${VARIABLE}` concatenated with string, use curly braces as it makes it easier to read.

In case it's a `"$LONELY_VARIABLE"` there's no need for that, as it will help you realize faster if it's "lonely" or not.