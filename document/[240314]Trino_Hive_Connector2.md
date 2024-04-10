---
tags:
  - trino
---

# Trino w/ Hive Connector 2

## Fault-tolerant execution support
- Read/Write는 모든 non-transaction table의 Retry Policy에 적용됨
- Transaction table에서는 read에 대해 retry policy가 적용됨
  - **CTAS, Write에서는 적용되지 x**

## Table Statistics
- Hive Connector는쿼리 성능 향상을 위해 테이블 통계 지원
- 데이터를 write할 때, HiveConnector는 항상 기본 통계를 작성함
  - 기본 통계: `numFiles, numRows, rawDataSize, totalSize`
- 열 수준 통계도 기본적으로 제공

### 테이블 통계가 가능한 Column
- TINYINT, SMALLINT, INTEGER, DATE, TIMESTAMP
  - Null수, distinct한 수, min/max
- VARCHAR, CHAR
  - null의 수, distinct 수

### Updaing Table and Partition Statistics
- query가 complex하거나 large data set을 포함한 경우
  - **TABLE, PARTITION에서 `ANALYZE` 명령을 수행하면 쿼리 성능 향상이 가능**
- PARTITIONS 지정 방법
  ```sql
  ANALYZE table_name WITH (
      partitions = ARRAY[
          ARRAY['p1_value1', 'p1_value2'],
          ARRAY['p2_value1', 'p2_value2']])
  ```
  - 키가 있는 두 파티션에 대한 통계
- 전체 테이블에 대한 모든 열에 대한 통계를 내는 것은 비효율적
  - 특정 열에 대한 통계를 내는 방법
  ```sql
  ANALYZE table_name WITH (
      partitions = ARRAY[ARRAY['p2_value1', 'p2_value2']],
      columns = ARRAY['col_1', 'col_2'])
  ```
  - col_1, col_2애 대한 통계 수집
  - p2_value1, p2_value2 키를 사용
- 부분집합(subset)에 대한 통계를 구하기 전, 기존 통계 제거 필요
  ```sql
  CALL system.drop_stats('schema_name', 'table_name')
  ```
- 선택한 파티션에 대한 통계 삭제 가능
  ```sql
  CALL system.drop_stats(
      schema_name => 'schema',
      table_name => 'table',
      partition_values => ARRAY[ARRAY['p2_value1', 'p2_value2']])
  ```

## Dynamic Filtering
- Dynamic Partition Pruning
  - 모든 파일 형식 지원
  - Broadcast, Partition join에 사용
- Dynamic Bucket Pruning
  - 모든 파일 형식 지원
  - Broadcast시 적용
- ORC, Parquet 파일 형식으로 저장된 테이블 일경우 Dynamic Filter도 포함
- ORC, Parquet 파일내 Sort Join 기준은 row-group pruning의 효과를 증가 시킴
  - 유사한 데이터를 동일한 stripe, row group내에 그룹화 하기 때문

## Table Redirection
- 다양한 데이터 형식을 가진 테이블을 보유
- `EXPLAIN`을 통해 실제 데이터 접근을 확인할 수 있음