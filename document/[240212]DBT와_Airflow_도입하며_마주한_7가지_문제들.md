---
tags:
  - dbt
  - airflow
---

# DBT와 Airflow 도입하며 마주한 7가지 문제들
- DBT + Airflow로 DE 역량이 없더라도 DE를 수행할 수 있도록
## 1. 어떤 구조로 DBT 프로젝트를 구성할 것인가
- raw_data, base, dimension, fact
- raw_data: base = 1:1
  - DBT docs의 staging과 동일 역할

### Base 계층의 설계
- `{source}_{data name}`
- base
  ```yaml
  # 예시 base.yml
  - name: db_something_some_table
    description: no description
    config:
      tags: ["kr", "database"]
  ```
  - tag: 모델의 사용 국가
- sql
  ```sql
  -- 예시 db_something_some_table.sql
  SELECT
      CAST(id AS STRING) AS user_id,
      CAST(is_leader AS BOOLEAN) AS is_leader,
      (CASE status_code
          WHEN 1 THEN "NEW"
          WHEN 2 THEN "REACTIVATED"
          WHEN 3 THEN "ACTIVATED"
          ELSE "DEFAULT"
          END) AS user_status,
      CAST(created_at AS TIMESTAMP) AS created_at,
  FROM {{ source('db_something', 'some_table') }}
  ```

### Dimension
- 특정 객체의 속성값
  - e.g. 사용자의 상태, 생성일자
- Dimension 계층은 Base 계층의 모델 참조

### Fact
- 사용자의 정보를 `X가 Y했다`라는 형태로 표현
- Dimension과 동일하게 Base 계층을 참조
  - 간단한 사용자 정보: `사용자가 메세지를 보냈다`
  - 리텐션 지표: `사용자가 처음으로 화면에 진입했다`
  - 비즈니스 로직: `사용자가 액티브하게 피드를 사용했다`
- 조건
  - owner
  - schema, description
  - tag: 모델의 주기, 국가. Airflow에서 참조
  - created_ymdt, updated_ymdt
  - 각 컬럼에 대해 필요시 test
- Fact나 Dimension은 정의에 따라, 같은 계층의 모델 참조 가능
  - 단, DAG 사용. 순환 참조는 일어나지 x

#### 예시
```yaml
# 예시 fact.yml
- name: sampleUser_sent_message
  description: sampleUser가 메시지를 보냈다
  meta:
    owner: henry@daangn.com
  config:
    tags: ["daily-kr"]
    cluster_by: ["..."]
  columns:
    - name: event_id
      description: 이벤트 ID
      tests:
        - unique:
          config:
            where: "DATE(event_timestamp) = DATE('{{ var('target_date') }}')"
      - not_null:
          config:
            where: "DATE(event_timestamp) = DATE('{{ var('target_date') }}')"
    - name: user_id
      description: 유저 ID
    - name: message_category
      description: 보낸 메시지의 카테고리
    - <<: *meta_columns
```
```sql
-- 예시 sampleUser_sent_message.sql
SELECT
    event_id,
    user_id,
    message_category
    ...
FROM {{ ref('db_something_some_table') }} AS something
	JOIN another_thing USING (user_id)
WHERE DATE(event_timestamp) = DATE('{{ var("target_date") }}')
	
UNION ALL

...
```

## 2. DBT 모델을 어떻게 저장할 것인가?
- 모든 모델을 다 저장하면
  - 데이터 스토리지 비용이 너무 높음
  - 모델을 다시 구할때마다 비용 발생
- 각 계층의 역할, 사용성을 고려 `materialized`방식을 다르게 지정

### base
- 필요한 데이터 추출, casting, aliasing만 진행
- 원천 데이터가 변경되는 경우 유연 대응이 가능하도록 `view`로 구성
```yaml
# dbt_project.yml
base:
  +enabled: true
  +materialized: view
  +tags: ["base"]
   ...
```

### Dimension
- `JOIN, WHERE` 포함
- 객체의 속성값을 표현하므로 많은 컬럼을 지닐 수는 잇으나
  - 개체의 수만큼 row수 -> row 수는 비교적 적을 수 있음
- `table`로 `materialize`
```yaml
# dbt_project.yml
dimension:
  +enabled: true
  +materialized: table
  +tags: ["dimension"]
  +persist_docs:
    columns: true
  ...
```

### Fact
- 복잡한 연산
- 로그성 데이터
- 시간 단위로 데이터 분할 가능성 존재
- `incremental`로 `materialize`
- 데이터를 날짜 단위로 파티셔닝
- `insert_overwrite`

#### Incremental 모델
- sql에서 `is_incremental()` macro 사용
- 모델을 처음 돌릴때 `full refresh`로 수행
  - 최초 모델 실행시 많은 비용
- `vars`를 활용하여 **파티션 지정**
```yaml
# dbt_project.yml
fact:
  +enabled: true
  +materialized: incremental
  +incremental_strategy: insert_overwrite
  +on_schema_change: append_new_columns
  +partition_by:
    field: DATE(event_timestamp)
    data_type: date
    granularity: day
    time_ingestion_partitioning: false
    copy_partitions: true
  +require_partition_filter: true
  +persist_docs:
    columns: true
  +tags: ["fact"]
  ...
```
```sql
-- 예시 sampleUser_sent_message.sql
SELECT
    ...
FROM {{ ref('something') }}
WHERE DATE(event_timestamp) = DATE('{{ var("target_date") }}')

-- DBT에서 incremental materialize할때 사용되는 방법이지만, 
-- 1) Airflow로 DBT 모델 실행시 정확한 날짜를 전달 할 수 있고
-- 2) 최초 모델 실행할때 full refresh하기에 당근은 데이터가 너무 많기에
-- vars로 갖고올 데이터의 파티션을 명시해서 갖고 오도록 했어요.
-- 밑의 구문은 DBT에서 incremental materialization시 자주 사용되는 패턴
-- {% if is_incremental() %}
-- WHERE event_timestamp > (select max(event_timestamp) from {{ this }})
-- {% endif %}
```

#### 모델 설정시 추가 옵션
- `clustering`
  - 효율적으로 모델들이 활용하기 위해
  - 쿼리 성능을 높히고 비용 저하 가능
- `persist_docs`
  - BigQuery에서도 `description`이 보이게 설정
- `on_schema_change`
  - `append_new_columns`로 설정
  - 스키마가 추가되는 경우, 컬럼이 `append`되도록
  - DBT 기본 설정은 `ignore`
    - `incremental` 모델에서 새로운 컬럼이 추가되어도 실제로는 생성 불가

## 3. DBT와 Airflow를 어떻게 Integration 할 것인가?
- 사용자에 대한 최신 정보
  - DBT 모델을 주기적으로 수행
- 모델간 의존성 Airflow에서 확인
- 모델별로 어느 시간 간격으로 주기적으로 수행 가능
- 모델 수행 가능

### `astronomer-cosmos`
- https://github.com/astronomer/astronomer-cosmos
- DBT 모델을 Airflow task로 쉽게 변환
- DBT 모델을 파싱하면서 의존성 파악. Airflow task로 그림
- `dbt run`, `dbt test` 명령어를 하나의 `task group`내에서 수행 가능
- `select`, `exclude` 파라미터
  - `DBT tag`나 `model path`지정시, 파이프라인에서 원하는 모델만 filtering 가능
  - Airflow DAG당 필요한 부분만 필터 후 활용
```python
# 예시
sample_task = DbtTaskGroup(
    group_id='sample_task',
    dbt_project_name='sample',
    dbt_root_path='sample',
    conn_id='sample',
    select={
        'paths': ['models/sample'...],
        'configs': ['tags:daily-kr'...],
    },
    exclude={
        'paths': [...],
        'configs': [...],
    },
    dbt_args={
        ...
        'vars': {
            'target_date': '{{ data_interval_start }}',
            ...
        },
    },
)
```
- 단, astronomer-cosmos는 릴리즈된지 얼마되지 않음
  - 빠르게 변화중

## 4. DBT 모델을 어떤 프로세스로 개발하고, 테스트할 것인가?
- 도메인 지식이 있는 사내 구성원도 쉽게 DBT 모델을 개발하기
- PR을 열면, 테스팅 환경이 PR별 배포
- DBT 모델에 대한 정의, SQL 작성, Pull Request를 열면 모델 테스트 진행
- PR별로 독립된 테스트 환경으로 Airflow, Compile된 DBT 모델의 `dbt docs`가 배포됨
- 이 과정에서 DBT 모델 정의, 의존성을 잘못 설정하면 CI 혹은 dbt docs에서 인지 가능
- DBT pipeline에 접속해서 테스트하고 싶은 모델 정보를 `Trigger w/ config`로 추가
- `DBT 모델`이 컴파일 및 `dry run`으로 모델 쿼리 수행
  - dry run, BigQuery 지원 기능
    - https://cloud.google.com/bigquery/docs/samples/bigquery-query-dry-run?hl=ko
  - 모델 쿼리가 잘 동작하는가
  - 컴파일된 코드가 얼마나 많은 byte를 스캔하는가
  - 쿼리의 데이터 스캔량이 `1TB`를 넘으면 실패
    - 단, 강제로 수행하는 `flag` 추가
- `dbt run`, `dbt test` 수행
  - `dbt run`의 결과물을 별도의 `_test` suffix가 있는 데이터셋에서 확인 가능
  - `dbt test`가 통과했는지 확인 가능
- 별도의 DBT Test 환경을 구축해서
  - `dbt`에 대한 설정을 구성원이 별도로 할 필요 x
  - `test`시 production data overwrite나, 비용 문제 발생 x

## 5. 다른 파이프라인과 DBT 모델간의 의존성은 어떻게 챙길 것인가?
- `DBT + astronomer-cosmos`: DBT 모델간 의존성 확보
  - but, source와의 의존성 x
- DBT 모델에서 바라보는 원천 데이터가 잘 적재 되고 모델 수행이 되어야 함
- 네이밍 컨벤션을 통해 source에 대한 참조 확인 가능
- `Airflow`의 `Sensor` 사용
  - `ExternalTaskSensor` 활용하여 원천 소스를 적재하는 파이프라인 완료되도록

### ExternalTaskSensor 구현시 유의점
- DBT 모델이, 자신이 바라보는 원천 소스가 적재되면 바로 수행
- `upstream`으로 원천 소스를 알아내야 함
```sql
-- 예시
-- user_actively_used_feed fact
SELECT * FROM {{ ref('user_scrolled') }}
UNION ALL
SELECT * FROM {{ ref('user_created_item') }}

-- user_scrolled fact
SELECT * FROM {{ ref('event_impressed') }} WHERE ...

-- user_created_item
SELECT * FROM {{ ref('db_article') }} WHERE ...

--
[db_article, event_impressed] 
 >> [user_scrolled, user_created_item] 
 >> user_actively_used_feed 
-- 해당 fact의 원천 소스는 db_article와 event_impressed
-- event_impressed, db_article이 적재되고 나서 DBT 모델들이 실행되어야 함
```
- 원천 소스별로 Airflow 파이프라인을 바라보는 `ExternalTaskSensor`를 구성
  - DBT모델에서 해당하는 sensor에 의존성 구현
```python
for (
    task
) in sample_dbt_task_group.children.values():
    if not isinstance(task, CosmosTaskGroup):
        continue
		
    # DBT모델이 Airflow내의 다른 DAG를 sensing할 수 있도록
    # source이름을 확인해서 적절한 파이프라인을 찾고
    # ExternalTaskSensor를 생성해서 모델에 의존성 추가
    sensor_task_ids = get_sensor_task_ids(...)
    task_sensor_deps = [
        sensor_tasks.get(task_id) for task_id in sensor_task_ids
    ]
    
    # Airflow내 다른 파이프라인에 의존성이 있다면 sensing하고, 없다면 skip
    if len(task_sensor_deps) > 0:
        task_sensor_deps >> task
```

## 6. 국가별로 같은 모델에 대한 정의를 어떻게 할 것인가?
- 파이프라인에 따라 국가, 도시의 타임존 고려
- DBT 모델 국가별 구현시
  - 중복이 많아짐
  - 특정 국가는 모델 추가 x
- DRY(Don't Repeat Yourself)
  - 하나의 프로젝트로 모든 모델 관리
  - 각 국가에 맞는 로직이 커스텀하게 실행될 수 있도록
  - `dbt vars`와 `dbt tag`, `jinja`를 최대한 활용

### 같은 DBT 모델이나, 국가별로 다른 타임존에 맞춰서 수행되는 경우
- `tags`를 사용하여 구분
- `tags`에 국가 코드를 작성
  - 자동으로 `Airflow`에서 국가 타임존에 맞춰서 수행되는 파이프라인
```yaml
- name: sampleUser_sent_message
  description: sampleUser가 메세지를 보냈다
  meta:
    owner: henry@daangn.com
  config:
    tags: ["daily-kr", "daily-..."]
    cluster_by: ["..."]
```

### 같은 모델이나, 국가마다 다른 비즈니스 로직 반영
- `vars`와 `jinja`를 활용하여 `SQL`에서 분기
- 기본적으로 복잡한 비즈니스 로직이 필요한 `core model`은 데이터팀에서 만듦
  - 필요시 여러 sql에서 반복될경우 `macros`를 활용하여 `SQL` 단순화
```sql
-- 예시 sampleUser_sent_message.sql
SELECT
    event_id,
    user_id,		
    message_category
    {% if var('country_code') is 한국 %}
    // 한국 관련 로직
    {% else %}
    // 다른 국가 관련 로직
    {% endif %}
FROM {{ ref('db_something_some_table') }}
...
```

## 7. DBT 모델의 backfill은 어떻게 할 것인가?
- 일반적으로 DBT에서는
  - `full refresh`로 전체 데이터 백필
  - 모든 모델에 대해 `full refresh`하는건 비용대비 효용이 떨어짐
- **원하는 날짜에 대해서만 데이터를 `backfill`할 수 있도록 `backfill pipeline 구성`

### Backfill의 조건
- 입력 값
  - 모델명
  - 날짜 범위
- 반자동으로 Backfill
- 모델마다 `backfill`해야하는 날짜를 다르게 지정하는 경우도 존재하기 때문에,
  - `Airflow dynamic task`를 활용하여 `DBT model`마다 다른 날짜 범위에 대한 데이터가 backfill
- Dynamic task를 사용하면
  - 하나의 task내에 여러 `mapped task`가 만들어져서, `task`별로 **실행**, **실패**기록 확인 가능
- `2023-11-01, 2023-11-30`설정시,
  - 30개의 mapped instance가 구성됨

## TODO
- DE 지식이 없어도, 데이터간 의존성, 스케줄링, DQ Test, Observability이 갖춰진 data pipeline 구성 가능
- DBT 모델의 확장성 문제
- 다른 Data Product와의 유기적 연동
- DBT 모델 개발, 테스트, 백필 환경 개선
- Base, Dimension, Fact이외에 필요한 형태 추가 개발
- 사용자 정보를 편리하게 찾을 수 있도록 `Discovery` 고도화

## References
- https://medium.com/daangn/dbt%EC%99%80-airflow-%EB%8F%84%EC%9E%85%ED%95%98%EB%A9%B0-%EB%A7%88%EC%A3%BC%ED%95%9C-7%EA%B0%80%EC%A7%80-%EB%AC%B8%EC%A0%9C%EB%93%A4-61250a9904ab