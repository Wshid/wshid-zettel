---
tags:
- dbt
- spark
---

# dbt-spark
- https://github.com/dbt-labs/dbt-spark
- https://docs.getdbt.com/docs/core/connect-data-platform/spark-setup
- dbt는 ETL중에 T를 담당
- 분석 준비가 완료되도록
  - warehouse에 있는 원시 데이터 정리
  - 정규화 해제
  - 필터링
  - 이름 변경, 사전 집계

## dbt-spark
- dbt가 Apache Spark, Databricks와 함께 작동할 수 있도록 모든 코드가 포함됨

## local 구동 방법
- `docker-compose` 환경에서는 아래 서비스를 Hive Metastore back-end로 활용
  - Spark Thrift 서버
  - Postgres db
- `dbt-spark`는 spark 3.3.2도 지원함
- 예시 profile
  ```yaml
  spark_testing:
  target: local
  outputs:
    local:
      type: spark
      method: thrift
      host: 127.0.0.1
      port: 10000
      user: dbt
      schema: analytics
      connect_retries: 5
      connect_timeout: 60
      retry_all: true
  ```
- SQL-based testing 가능
  - localhost:10000
  - jdbc:hive2://localhost:10000 (hive jdbc 환경)
    - credential, dbt:dbt

## Installing dbt-spark
```bash
python -m pip install dbt-spark
```

## Configuring dbt-spark
- connecting to a spark cluster via
  - the generic thrift | http method -> `PyHive`
```bash
python -m pip install "dbt-spark[ODBC]"
python -m pip install "dbt-spark[PyHive]"
python -m pip install "dbt-spark[session]"
```

### 연결 방식
- odbc
  - Databricks에 연결될 때 선호하는 방식
  - SQL Endpoint
  - 범용 대화형 클러스터 연결
- thrift
  - localhost hosted, on-promise
- http
  - http endpoint
  - Databricks 대화형 클러스터에 대한 연결 포함
- session
  - local, remote system에서 실행되는 pyspark 세션 연결
