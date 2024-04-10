---
tags:
  - data-quality
  - dbt
  - great-expectations
---

# Data Quality with DBT Tests and Great Expectations
- https://medium.com/@shehroz1447/data-quality-with-dbt-tests-and-great-expectations-b349634089bf

## Data Quality
- DS, DE에서 중요한 지
  - 데이터가 정확한지, 일관성 있는지, 신뢰성이 있는지
- DBT Test를 통해 데이터 품질 검사를 구현
- DQ Test를 dbt workflow에 통합하면서

## Generic Tests in DBT
- dbt core에 포함된 4가지 테스트
  - not_null
  - unique
  - accecpted_value
  - relationship
- 이외에 Great Expectation과 같은 **외부 패키지**를 포함하여
  - 일반적인 테스트도 포함 가능

### YAML 파일 예시
```yaml
version: 2
models:
  - name: dim_hosts_cleansed
    description: CleansedtablefortheAirbnbhosts
    columns:
      - name: host_id
        description: Theidofthehost.Thisistheprimarykey.
        tests:
          - not_null
          - unique      
      - name: host_name
        description: Thenameofthehost
        tests:
          - not_null
      - name: is_superhost
        description: Defineswhetherthehostsisasuperhost.
        tests:
          - accepted_values:
            values: ['t', 'f'] # 특정값만 올수 있도록 정의됨
```
- dim_hosts_cleaned 모델의 일반적인 테스트
  - 모델을 테이블로 생각할 수 있음
- 테스트 구동 명령어
  ```bash
  dbt test --select dim_hosts_cleansed
  ```

## Importing more Generic Tests with Great Expectations
- DBT에서 사용자 정의 테스트를 만들 수 있음
  - 하지만 그 전에, 외부 open source에서 먼저 찾아보기를 권장
- 테스트 예시
  - dbt-expectations: https://github.com/calogica/dbt-expectations
  - Great Expectation을 위한 python open source
  - 여러 개의 큰 GX Test dbt에 가져오는데 도움이 될 수 있음

### Bind dbt-expectations
```yaml
# packages.yml
packages:
  - packages: calogica/dbt_expectations
    version: [">=0.8.-", "<0.9.0"]
```
```bash
dbt deps
```

### Use dbt-expectations
```yml
version: 2
models:
  - name: dim_listings_w_hosts
    tests:
      - dbt_expectations.expect_table_row_count_to_equal_other_table:
          compare_model: source('airbnb', 'listings')
    columns:
      - name: price
        tests:
          - dbt_expectations.expect_column_values_to_be_of_type:
              column_type: number
          - dbt_expectations.expect_column_quantile_values_to_be_between:
              quantile: .99
              min_value: 50
              max_value: 500
          - dbt_expectations.expect_column_max_to_be_between:
              max_value: 10000
              config:
                severity: warn
```
```bash
dbt test --select dim_listings_w_hosts
```
- GX에 대한 상세한 예시는 아래 repository에서 추가 확인 가능
  - Great Expections official repo: https://github.com/great-expectations/great_expectations

## Singular Tests in DBT
- Singular Test
  - 특정 모델을 참조하는 sql 정의
  - 테스트를 통과하면 **empty result set**를 반환함
- 상세 비즈니스 로직에 대한 테스트를 자유롭게 작성할 수 있음

### Singular Test Example
- 검토 날짜가 해당 목록의 날짜 이후에 추가되도록 작성
- 검토 날짜는 **해당 목록 작성일 이후**에 추가되어야 함
```sql
SELECT * 
FROM {{ ref('dim_listings_cleansed') }} l
    INNER JOIN {{ ref('fct_reviews') }} r
USING (listing_id)
WHERE l.created_at >= r.review_date
```
- `consistent_created_at`이라는 singular test의 예시
  - 테스트 sql은 테스트가 통과된다면 empty set을 리턴해야 함

### Singular Test Exmaple - DBT macro
- macro를 사용하여, singular test를 generic test에 활용할 수 있음
- macro는 python/java에서 function과 동일함
- jinja template을 사용한 macro 예시
  ```sql
  {% macro no_nulls_in_columns(model) %}
      SELECT * FROM {{ model }} WHERE
      {% for col in adapter.get_columns_in_relation(model) -%}
          {{ col.column }} IS NULL OR
      {% endfor %}
      FALSE
  {% endmacro %}
  ```
  - `not_nulls_in_columns` macro
  - macro는 기본 프로젝트 폴더 내 `macro/` 하위 sql파일로 지정됨
  - `yaml`에서 이 테스트를 참조하고, 일반 테스트 처럼 구현 가능

## Conclusion
- DQ는 강조해도 지나치지 않음
- DBT Test를 Data transformation process에 포함시키면, DQ 및 reliability 확보에 도움이 될 수 있음
