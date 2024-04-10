# DuckDB vs MotherDuck
- https://kestra.io/blogs/2023-07-28-duckdb-vs-motherduck

## DuckDB
- Analytics용 SQLite
  - DB 서버를 설치할 필요가 없음
- OLAP Database
- DuckDB 설치시, CLI/SQL만 사용하여 빠른 OLAP Engine 사용 가능
- 단점
  - High Concurrency X
  - User Management X
  - scale horizontally X

## MotherDuck
- DuckDB의 Cloud Serveless 버전
  - DuckDB보다 장점이 많음
  - Managed storage, data sharing, interactive SQL IDE
- Cloud에 저장된 데이터(MotherDuck에 저장 or 원격 클라우드 스토리지 bucket에 저장된 데이터)
- 로컬에 저장된 데이터 활용 가능
- 주어진 쿼리를 수행하는 가장 적합한 장솔를 선택
- MotherDuck Table + Notebook(in Local) 사이에 Join 가능

## DuckDB -> MotherDuck
- 아직 MotherDuck은 Beta 상태
- GA(General Availability)까지 대기하는 것을 추천

## DuckDB 시작하기
```bash
brew install duckdb
```
```sql
duckdb orders.db

INSTALL httpfs;
LOAD httpfs;

-- create table from a remote CSV file
CREATE TABLE orders AS
SELECT * FROM read_csv_auto('https://raw.githubusercontent.com/kestra-io/datasets/main/csv/orders.csv');

-- create another table
CREATE TABLE bestsellers as
SELECT product_id, round(sum(total),2) as total
FROM orders
GROUP BY 1
ORDER BY 2 DESC;
```
- 이후 CSV 파일이나, S3 등으로 내보낼 수 있음(`COPY` 사용)

## Using MotherDuck and DuckDB in ETL Pipelines
- Kestra, Apache Airflow 등의 Orchestration tool과 같이 사용
- Kestra
  - docker compose file: https://github.com/kestra-io/kestra/blob/develop/docker-compose.yml
  - `docker compose up -d`
  - 이후 DuckDB에 대한 기본 예제 확인 가능
  - DuckDB의 경우 `hash()`, `md5()`등을 사용하여
    - **민감한 열**을 마스킹 할 수 있음
- Git workflow 사용 가능(w/ dbt)
  - https://us.kestra.cloud/ui/login?from=/ui/demo/dashboard
- Email Report
  - S3에서 데이터 적재, DuckDB 쿼리로 집계
  - 최종 파일은 CSV로 추출하여 Email 전달
- Send KPIs via Slack
  - threshold를 지정하여, 해당 값 범주에 따라 slack 메세지 전달
  - DuckDB 쿼리를 사용하여 유효값 판단
- Anomaly Detection
  - 신규 파일이 S3에 도착할때마다, DuckDB가 데이터의 이상 유무 판단
  - 이상치 탐지시, 경고 전송
