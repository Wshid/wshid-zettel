---
tags:
  - dbt
---

# dbt-expectations
- https://hub.getdbt.com/calogica/dbt_expectations/latest/
- Great Expectations package for Python

## `dbt-date`와 의존성 존재 
```yml
# dbt_project.yml
vars:
  'dbt_date:time_zone': 'America/Los_Angeles'
```
- valid timezone에 맞게 설정 필요

## Table shape

### expect_table_columns_to_contain_set
- 열의 목록이 포함되는가
- Applies to: Model, Seed, Source
```yml
models: # or seeds:
  - name: my_model
    tests:
      - dbt_expectations.expect_table_columns_to_contain_set:
          column_list: ["col_a", "col_b"]
          transform: upper # (Optional)
```

### expect_table_row_count_to_equal_other_table
- model과 another model 간의 카운트가 동일한가
- Applies to: Model, Seed, Source
```yml
models: # or seeds:
  - name: my_model
    tests:
      - dbt_expectations.expect_table_row_count_to_equal_other_table:
          compare_model: ref("other_model")
          group_by: [col1, col2] # (Optional)
          compare_group_by: [col1, col2] # (Optional)
          factor: 1 # (Optional)
          row_condition: "id is not null" # (Optional)
          compare_row_condition: "id is not null" # (Optional)
```

## Missing values, unique values, and types

### expect_column_values_to_be_in_type_list
- column내 값이 주어진 타입 리스트에 부합하는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_be_in_type_list:
      column_type_list: [date, datetime]
```

### expect_column_values_to_have_consistent_casing
- Column내의 값이 일관성 있는 케이스를 가지는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_have_consistent_casing:
      display_inconsistent_columns: false # (Optional)
```
- `display_inconsistent_columns: true`로 설정시
  - 열에 일관성 없는 값의 수가 터미널에 표시되고,
  - SQL compile시 일관성에 부합하지 않는 값 자체가 반환됨

## Sets and ranges

### expect_column_values_to_be_in_set
- column value가 주어진 목록중에 포함되는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_be_in_set:
      value_set: ['a','b','c']
      quote_values: true # (Optional. Default is 'true'.)
      row_condition: "id is not null" # (Optional)
```

### expect_column_values_to_be_between
- 주어진 값 범위에 포함되는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_be_between:
      min_value: 0  # (Optional)
      max_value: 10 # (Optional)
      row_condition: "id is not null" # (Optional)
      strictly: false # (Optional. Default is 'false'. Adds an 'or equal to' to the comparison operator for min/max)
```

## String matching

### expect_column_value_lengths_to_be_between
- 열 항목이 `min_value, max_value`값 사이의 길이를 가진 문자열을 가지는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_value_lengths_to_be_between:
      min_value: 1 # (Optional)
      max_value: 4 # (Optional)
      row_condition: "id is not null" # (Optional)
      strictly: false # (Optional. Default is 'false'. Adds an 'or equal to' to the comparison operator for min/max)
```

### expect_column_values_to_match_regex
- 열 항목이 주어진 정규식과 일치하는 문자열인가
- Optional arguments
  - is_raw: raw string and to be escaped. default: `False`
  - flags: regex에 전달되는 문자열. adapter의 성격마다 다름
    - e.g. case-insensitive -> `i` 플래그 사용
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_match_regex:
      regex: "[at]+"
      row_condition: "id is not null" # (Optional)
      is_raw: True # (Optional)
      flags: i # (Optional)
```

### expect_column_values_to_match_like_pattern
- 열 항목이 SQL `Like` 패턴과 일치하는가
- Applies to: Column
```yml
tests:
  - dbt_expectations.expect_column_values_to_match_like_pattern:
      like_pattern: "%@%"
      row_condition: "id is not null" # (Optional)
```

## Aggregate functions
- column의 distinct count 관련 붑분
- column내 값의 고유 비율에 대한 부분

## Multi-column
- A 컬럼과 B 컬럼에 대한 비교
- 복합키가 유니크 한가

## Distributional functions
- 기간의 차이가 log-normal distribution(로그 정규 분포)를 따를때 이상값이 존재하는가
- 값이 주어진 날짜 범위에 상응하는가
  - `{{ modules.datetime.date.today() }}`등을 지정하여 `test_end_date`을 동적으로 지정 가능
