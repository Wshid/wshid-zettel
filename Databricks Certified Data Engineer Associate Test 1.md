---
date: 2025-05-08
datetime: 2025-05-08 21:09:45
book: 
page: 
tags: 
references:
  - https://www.udemy.com/course/databricks-certified-data-engineer-associate-practice-tests/learn/quiz/5596958/test?components=add_to_cart%2Cavailable_coupons%2Cbase_purchase_section%2Cbuy_button%2Cbuy_for_team%2Ccacheable_buy_button%2Ccacheable_deal_badge%2Ccacheable_discount_expiration%2Ccacheable_price_text%2Ccacheable_purchase_text%2Ccurated_for_ufb_notice_context%2Ccurriculum_context%2Cdeal_badge%2Cdiscount_expiration%2Cgift_this_course%2Cincentives%2Cinstructor_links%2Clifetime_access_context%2Cmoney_back_guarantee%2Cprice_text%2Cpurchase_tabs_context%2Cpurchase%2Crecommendation%2Credeem_coupon%2Csidebar_container%2Cpurchase_body_container%2Cone_click_checkout&couponCode=ST6MT60525G1#content
aliases:
---
## Q1
You were asked to create a table that can store the below data, orderTime is a timestamp but the finance team when they query this data normally prefer the orderTime in date format, you would like to create a calculated column that can convert the orderTime column timestamp datatype to date and store it, fill in the blank to complete the DDL.

```sql
CREATE TABLE orders (
    orderId int,
    orderTime timestamp,
    orderdate date _____________________________________________ ,
    units int)
```
#### A1
- `GENERATED ALWAYS AS (CAST(orderTime as DATE))`
	- Generated Always as: 다른 열을 기반으로 생성함
	- 직접 값을 삽입하면 문제 발생
	- 해당 열은 파티셔닝에 활용하기도 함

## Q2
The data engineering team noticed that one of the job fails randomly as a result of using spot instances, what feature in Jobs/Tasks can be used to address this issue so the job is more stable when using spot instances?
#### A2
- `Add a retry policy to the task`
	- Job내 Task에서 retry policy를 지정할 수 있음

## Q3
What is the main difference between AUTO LOADER  and COPY INTO?
#### A3
- `AUTO LOADER Supports file notification when performing incremental loads.`
	- AUTO LOADER: directory listing, file notification 지원
	- COPY INTO: directory listing
- AUTO loader File Notification
	- Azure blob storage, S3 등의 입력 디렉터리에서 파일 이벤트를 구독하는 알림 서비스 자동 설정
	- 대규모 입력 디렉터리, 대량의 파일에 높은 성능, 확장성 제공
	- ![Image](https://github.com/user-attachments/assets/b0a8303c-5942-4229-b9ca-4a7e2e4452ae)
		1. Directory listing - List Directory and maintain the state in RocksDB, supports incremental file listing
		2. File notification - Uses a trigger+queue to store the file notification which can be later used to retrieve the file, unlike Directory listing File notification can scale up to millions of files per day.

#### Auto Loader vs Copy Into
- 새로운 데이터 파일이 클라우드 스토리지에 도착시 추가 설정 없이 점진적, 효율적으로 처리
- cloudFiles라는 새로운 구조화된 스트리밍 소스를 제공
- 클라우드 파일 스토리의 입력 디렉터리 경로가 주어지면, cloudFiles source는 새 파일이 도착하면 자동 처리
	- 해당 디렉터리에 있는 기존 파일도 처리하는 옵션 제공
- 사용해야 하는 경우
	- 수백만 개 이상의 파일이 포함된 파일 위치에서 데이터 로드시
		- COPY INTO SQL 명령보다 파일을 더 효율적으로 검색 및 파일 처리시 여러 배치로 분할 가능
	- 이전에 업로드한 하위 집합을 로드할 계획이 없을 때
		- AutoLoader 사용시 파일의 하위 집합 재처리가 어려움
		- 단, Auto Loader 스트림이 동시에 실행되는 순간, COPY INTO SQL을 사용하여 파일의 하위 집합 로드 가능
- When to use COPY INTO
	- [https://docs.databricks.com/delta/delta-ingest.html#copy-into-sql-command](https://docs.databricks.com/delta/delta-ingest.html#copy-into-sql-command)
	- `COPY INTO`는 deprecate 되었으며, `CREATE STREAMING TABLE`이라는 기능으로 변경됨
- When to use Auto Loader
	- [https://docs.databricks.com/delta/delta-ingest.html#auto-loader](https://docs.databricks.com/delta/delta-ingest.html#auto-loader)

## Q4
Why does AUTO LOADER require schema location?
#### A4
- Schema location is used to store schema inferred by AUTO LOADER
- Schema location은 Auto Loader가 추론한 스키마를 저장하는 데 사용함
	- 다음에 Auto Loader가 실행될 때 알려진 스키마를 사용하여
	- 매번 스키마를 추론할 필요가 없으므로 Auto Loader가 더 빠르게 수행됨
- Auto Loader는 처음 발견한 50G, 1000개의 파일 중 먼저 초과하는 한도를 sampling
	- 스트림 시작시마다 이러한 inference cost가 발생하지 않도록 하고
	- 스트림이 재시작될 때마다 안정적인 스키마를 제공하려면 `cloudFiles.schemaLocation` 옵션 설정 필요
	- AutoLoader는 이 숨겨진 디렉터리 `_schemas`를 생성하여 시간 경과에 따른 입력 데이터의 스키마 변경사항 추적
- [Auto Loader options](https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/options)

## Q5
Which of the following statements are incorrect about the lakehouse
#### A5
- Storage is coupled with Compute

#### LakeHouse의 특징
- https://www.databricks.com/blog/2020/01/30/what-is-a-data-lakehouse.html
- Transaction Support: ACID
- Schema enforcement and governance: DW schema, star/snowflake schema
- BI Support
	- BI tools, 원본 데이터에서 바로 활용 가능
- Storage is decoupled from compute
	- 스토리지와 컴퓨팅이 별도의 클러스터 사용
	- 더 많은 동시 사용자, 더 많은 데이터 크기 지원
- Openness
	- parquet등의 open format 사용
	- API 지원, 데이터 직접 조회 및 활용(python, R, ml) 가능
- Support for diverse data types ranging from unstructured to structured data
- Support for diverse workloads
	- 데이터 과학, 머신러닝, SQL 분석 등을 동일한 데이터 레포지토리 에서 가능
- End-to-end streaming
	- 실시간 보고서 제공
## Q6
You are designing a data model that works for both machine learning using images and Batch ETL/ELT workloads. Which of the following features of data lakehouse can help you meet the needs of both workloads?
#### A6  
Data lakehouse can store unstructured data and support ACID transactions.


## Q7
Which of the following locations in Databricks product architecture hosts jobs/pipelines and queries?

#### A7
- Control Plane
- Databricks는 대부분의 서비스를 Control Plane, Data Plane에서 운영
	- SQL endpoint, DLT 컴퓨팅과 같은 serverless 기능은 Control Plane에서 shared compute 사용

#### Control Plane
- Stored in Databricks Cloud Account
- Databricks가 자체 Azure 계정에서 관리하는 백엔드 서비스 포함
- notebook commands 및 기타 많은 workspace configurations는 해당 제어 영역에 저장
- 미사용시 암호화
- Web Application
- Compute orchestration
- Unity Catalog
- Queries and Code

#### Data Plane
- Stored in Customer Cloud Account
- Azure 계정에서 관리하며 데이터가 상주하는 곳
- 데이터가 처리되는 곳
- Azure Databricks Connector를 사용하여
	- 클러스터가 Azure 계정 외부의 데이터 소스에 연결하여 데이터 수집, 저장 가능
- ![Image](https://github.com/user-attachments/assets/3bdb928d-fe07-4559-b224-203f09bd4572)

## Q8
You are currently working on a notebook that will populate a reporting table for downstream process consumption, this process needs to run on a schedule every hour. what type of cluster are you going to use to set up this job?
#### A8
- The job cluster is best suited for this purpose.
- 예약된 작업의 경우 실행 중에 노트북과 상호작용할 필요가 없으므로
	- job cluster가 합리적
- all-purpose cluster를 사용하게 되면 job cluster보다 2배더 비싼 비용 발생
- 작업이 완료되면 새 클러스터를 만드는 옵션이 있는 job scheduler를 실행하면 클러스터가 종료됨
	- job cluster는 재시작 불가

## Q9
Which of the following developer operations in CI/CD flow can be implemented in Databricks Repos?

#### A9
- Trigger Databricks Repos API to pull the latest version of code into production folder

#### CI/CD Databricks Repo
- 아래 그림에서 노란색의 강조 단계는 Databricks Repo에서 수행 가능
- 회색으로 표시된 단계는 Github 또는 Azure DataOps와 같은 Git 공급자에서 수행 가능
- ![[Pasted image 20250508214455.png]]

## Q10
You are currently working with the second team and both teams are looking to modify the same notebook, you noticed that the second member is copying the notebooks to the personal folder to edit and replace the collaboration notebook, which notebook feature do you recommend to make the process easier to collaborate.

#### A10
- Databricks Notebooks support real-time coauthoring on a single notebook

## Q11
You are currently working on a project that requires the use of SQL and Python in a given notebook, what would be your approach

#### A11
- A single notebook can support multiple languages, use the magic command to switch between the two.
- `%sql`, `%python`과 같은 magic commands 사용 가능

## Q12
Which of the following statements are correct on how Delta Lake implements a lake house?
#### A12
- Delta lake uses open source, open format, optimized cloud storage and scalable meta data

#### Deltalake
- Open source
- standard data format
- Optimized for cloud object storage
- scalable metadata handling
- Data Lake 위에 구축되어 데이터의 안정성, 보안성, 성능을 향상시키도록 설계된 오픈소스 스토리지 계층
- 파일 기반 트랜잭션 로그를 추가하여
	- ACID Transaction, 확장 가능한 메타 데이터 처리, 스트리밍 및 배치 데이터 처리 통합 등의 기능 제공
- Azure Databricks에서는 별도의 설정 없이 모든 테이블이 Delta Table로 생성됨
- 클라우드 데이터 객체 스토리지내 Parquet 파일 + 트랜잭션 로그(`_delta_log/` 하위 json으로 저장)하여 사용
	- 트랜잭션, 버전관리 지원

#### Deltalake is not
- Proprietary technology
- Storage format
- Storage medium
- Database service or data warehouse

## Q13
You were asked to create or overwrite an existing delta table to store the below transaction data.

#### A13
```sql
CREATE OR REPLACE TABLE transactions (
transactionId int,
transactionDate timestamp,
unitsSold int)
```
- databricks에서는 기본적으로 delta format을 사용함

## Q14
if you run the command `VACUUM transactions retain 0 hours`? What is the outcome of this command?
#### A14
- Command will fail, you cannot run the command with retentionDurationcheck enabled
```sql
VACUUM [db_name].table_name | path] [RETAIN num HOURS] [DRY RUN]
```
- Delta Table과 관련된 디렉터리를 재귀적으로 청소
	- 테이블에 대한 transaction log의 최신 상태가 아니며
	- retention theshold 보다 오래된 파일을 제거
	- default: 7d
- 이 검사를 활성화 하는 이유
	- Delta가 의도치 않은 기록 삭제를 방지
	- `0 hours`로 설정할 경우 데이터 손실 가능성이 있기 때문
- Documentation in VACUUM https://docs.delta.io/latest/delta-utility.html
	- https://kb.databricks.com/delta/data-missing-vacuum-parallel-write.html

## Q15
You noticed a colleague is manually copying the data to the backup folder prior to running an update command, incase if the update command did not provide the expected outcome so he can use the backup copy to replace table, which Delta Lake feature would you recommend simplifying the process?
#### A15
- Use time travel feature to refer old data instead of manually copying
- [https://databricks.com/blog/2019/02/04/introducing-delta-time-travel-for-large-scale-data-lakes.html](https://databricks.com/blog/2019/02/04/introducing-delta-time-travel-for-large-scale-data-lakes.html)
```sql
SELECT count(*) FROM my_table TIMESTAMP AS OF "2019-01-01"
SELECT count(*) FROM my_table TIMESTAMP AS OF date_sub(current_date(), 1)
SELECT count(*) FROM my_table TIMESTAMP AS OF "2019-01-01 01:30:00.000"
```

## Q16
Which one of the following is not a Databricks lakehouse object?
#### A16
- Stored Procedures
- object
	- Tables
	- Views
	- Database/Schemas
	- Catalog
	- Functions

## Q17
What type of table is created when you create delta table with below command?
```sql
CREATE TABLE transactions USING DELTA LOCATION "DBFS:/mnt/bronze/transactions"
```
#### A17
- External Table
	- 테이블 생성시 `LOCATION` 키워드를 사용하면 external 테이블로 생성됨
- LOCATION을 지정하지 않으면 `MANAGED`로 생성됨

## Q18
Which of the following command can be used to drop a managed delta table and the underlying files in the storage?
#### A18
```sql
DROP TABLE table_name
```
- MANAGED 테이블의 경우 DROP 되면 하기 정보도 같이 제거됨
	- data, metadata, history

## Q19
Which of the following is the correct statement for a session scoped temporary view?
#### A19
- Temporary views are lost once the notebook is detached and re-attached

#### Temporary view의 두가지 유형
- session scoped
	- Spark session에서만 사용 가능
	- 같은 클러스터의 다른 노트북은 액세스 불가
	- 노트북이 분리/재연결시 local temp view lost
- Global
	- 클러스터의 모든 노트북에서 global temp view 보기 가능
	- 단, cluster 재시작시 temp view는 사라짐

## Q20
Which of the following is correct for the global temporary view?
#### A20
- global temporary views **can be** still accessed even if the notebook is detached and attached

## Q21
You are currently working on reloading customer_sales tables using the below query  
```sql
INSERT OVERWRITE customer_sales
SELECT * FROM customers c
INNER JOIN sales_monthly s on s.customer_id = c.customer_id

```
After you ran the above command, the Marketing team quickly wanted to review the old data that was in the table. How does INSERT OVERWRITE impact the data in the `customer_sales` table if you want to see the previous version of the data prior to running the above statement?

#### A21
- Overwrites the data in the table but preserves all historical versions of the data, you can time travel to previous versions
- OVERWRITE를 하더라도, 이전 버전으로 Time traveling 가능
	- any DML/DDL 단, DROP TABLE은 허용하지 않음
	- 모든 기록을 보존함
- SQL Syntax for Time travel
```sql
SELECT * FROM table_name as of [version number]

SELECT * FROM customer_sales as of 1 -- previous version
SELECT * FROM customer_sales as of 2 -- current version

DESCRIBE HISTORY table_name
```


#### INSERT OVERWRITE와 `CREATE OR REPLACE TABLE`(CRAS)의 차이
- CRAS의 경우 테이블의 스키마 수정이 가능함
	- 신규열 추가 및 기존 열의 데이터 유형 변경 등
- `INSERT OVERWRITE`의 경우 데이터를 덮어쓰기만 함
- `spark.databricks.delta.schema.autoMerge.enable=True` 옵션을 사용하지 않고 `INSERT OVERWRITE`를 할때
	- 스키마 변경시 실패 발생

## Q22
Which of the following SQL statement can be used to query a table by eliminating duplicate rows from the query results?
#### A22
```sql
SELECT DISTINCT * FROM table_name
```

## Q23
Which of the below SQL Statements can be used to create a SQL UDF to convert Celsius to Fahrenheit and vice versa, you need to pass two parameters to this function one, actual temperature, and the second that identifies if its needs to be converted to Fahrenheit or Celcius with a one-word letter F or C?
Example:
```sql
select udf_convert(60,'C')  will result in 15.5
select udf_convert(10,'F')  will result in 50
```
#### A23
```sql
CREATE FUNCTION udf_convert(temp DOUBLE, measure STRING)
RETURNS DOUBLE
RETURN CASE WHEN measure == 'F' then (temp * 9/5) + 32
        ELSE (temp – 33 ) * 5/9
       END
```
- UDF를 만들때 `CREATE FUNCTION` 사용
- 파라미터 타입 제시
- RETURNS의 타입 정의
- RETURN + 연산 + END로 이루어짐

## Q24
You are trying to calculate total sales made by all the employees by parsing a complex struct data type that stores employee and sales data, how would you approach this in SQL

Table definition,
```sql
batchId INT, 
performance ARRAY<STRUCT<employeeId: BIGINT, sales: INT>>, 
insertDate TIMESTAMP
```

Sample data of `performance` column
```json
[
{ "employeeId":1234
"sales" : 10000},
 
{ "employeeId":3232
"sales" : 30000}
]
```
Calculate total sales made by all the employees?

Sample data with create table syntax for the data: 
```sql
create or replace table sales as 
select 1 as batchId ,
	from_json('[{ "employeeId":1234,"sales" : 10000 },{ "employeeId":3232,"sales" : 30000 }]',
         'ARRAY<STRUCT<employeeId: BIGINT, sales: INT>>') as performance,
  current_timestamp() as insertDate
union all 
select 2 as batchId ,
  from_json('[{ "employeeId":1235,"sales" : 10500 },{ "employeeId":3233,"sales" : 32000 }]',
                'ARRAY<STRUCT<employeeId: BIGINT, sales: INT>>') as performance,
                current_timestamp() as insertDate
```

#### A24
```sql
select aggregate(flatten(collect_list(performance.sales)), 0, (x, y) -> x + y) 
as  total_sales from sales  
```
- nested structure에는 `.` notation을 사용함
	- `performance.sales`
- `:`의 경우 json 데이터를 참조할때만 사용되나, 여기서는 구조체 유형을 다루기 때문
	- json을 다루는지, 구조체 데이터를 다루는지에 따라 다름
![[Pasted image 20250509201923.png]]

#### A24 - Other solutions
```sql
-- other solutions 1
select reduce(flatten(collect_list(performance.sales)), 0, (x, y) -> x + y) as  total_sales from sales;

-- other solutions 2
with cte as (
  select
    explode(flatten(collect_list(performance.sales))) sales from sales
)
select
  sum(sales) from cte
```
- databricks에서 `reduce`와 `aggregate`는 동의어
	- aggregate는 좀 더 일반적인 집계 연산
	- Reduce는 배열 값을 단일 값으로 축소한다는 의미 강조

## Q25
Which of the following statements can be used to test the functionality of code to test number of rows in the table equal to 10 in python?
```python
row_count = spark.sql("select count(*) from table").collect()[0][0]
```

#### A25
```python
assert row_count == 10, "Row count did not match"
```


## Q26
How do you handle failures gracefully when writing code in Pyspark,  fill in the blanks to complete the below statement

```python
_____
 
    Spark.read.table("table_name").select("column").write.mode("append").SaveAsTable("new_table_name")
 
_____
 
    print(f"query failed")
```

#### A26
```python
try: except:
```


## Q27
You are working on a process to query the table based on batch date, and batch date is an input parameter and expected to change every time the program runs, what is the best way to we can parameterize the query to run without manually changing the batch date?
#### A27
- Create a notebook parameter for batch date and assign the value to a python variable and use a spark data frame to filter the data based on the python variable
- notebook parameter에 Python 변수 지정 이후, 이를 spark dataframe에서 사용하면 되는 것으로 보임

## Q28
Which of the following commands results in the successful creation of a view on top of the delta stream(stream on delta table)?
#### A28
```python
Spark.readStream.format("delta").table("sales").createOrReplaceTempView("streaming_vw")
```
- 델타 테이블을 스트림 소스로 로드하여 스트리밍 쿼리에 사용하면
	- 모든 데이터 + 스트림 시작 이후 도착하는 새 데이터를 처리함
- path, tables를 모두 stream으로 읽을 수 있음
- delete, changes(update, merge, overwrite)를 무시할 수도 있음
- https://docs.databricks.com/aws/en/structured-streaming/delta-lake#delta-table-as-a-source

## Q29
Which of the following techniques structured streaming uses to create an end-to-end fault tolerance?
#### A29
- Checkpointing and idempotent sinks

#### structured steaming의 fault tolerance
- checkpointing
	- write-ahead logs
	- offset range of data being processed during trigger interval
- idempotent
	- multiple writes of the same data
	- do not result in duplicates being written to the sink
	- 오프셋으로 식별되는 동일한 데이터를 여러번 쓰더라도, 싱크에 중복 기록이 발생하지 않음
- 그에 따라, end-to-end, exactly-once semantics 달성

## Q30
Which of the following two options are supported in identifying the arrival of new files, and incremental data from Cloud object storage using Auto Loader?
#### A30
- Directory listing, File notification
	- Directory listing: Auto loader identifies new files by listing the input dir
	- File Notification
		- notification and queue service that subscribe to file events from the input dir
- https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/file-detection-modes

## Q31
Which of the following data workloads will utilize a Bronze table as its destination?
#### A31
- A job that ingests raw data from a streaming source into the Lakehouse
	- kafka
	- optimized and stored in silver

#### Medallian Architecture
- https://www.databricks.com/glossary/medallion-architecture
	- **Bronze: raw data**
		- Raw copy of ingested data
		- Replaces traditional data lake
		- Provides efficient storage and querying of full, unprocessed history of data
		- No schema is applied at this layer
	- **Silver: cleaned and conformed data**
		- Reduces data storage complexity, latency, and redundency
		- Optimize ETL throughput and analytic query performance
		- Preserves grain of original data(without aggregation)
		- Eliminates duplicate records
		- Production schema enforced
		- Data quality checks, quarantine corrupt data
	- **Gold: curated business - level table**
		- Power MI apps, reporting, dashboards, ad hoc analytics
		- Refined views of data, typically with aggregations
		- Reduce stain on production system
		- Optimizes query performance for business-critical data

## Q32
Which of the following data workloads will utilize a silver table as its source?
#### A32
- A job that aggregates cleaned data to create standard summary statistics
	- 원본 데이터의 결을 유지해야 함(grain of the original data)
	- silver 영역의 data source를 가지고 gold에서 aggregating/storing

## Q33
Which of the following data workloads will utilize a gold table as its source?
#### A33
- A job that queries aggregated data that already feeds into a dashboard

## Q34
You are currently asked to work on building a data pipeline, you have noticed that you are currently working with a data source that has a lot of data quality issues and you need to monitor data quality and enforce it as part of the data ingestion process, which of the following tools can be used to address this problem?
#### A34
- DELTA LIVE TABLES
	- expectations를 사용하여 bad data 식별
	- Data query metrics이 event log에 저장되며 차후 분석, 모니터링에 사용 가능
	- 3가지의 expectation type을 가짐
#### Retain invalid records
- `except` operator를 사용
- valid record와 violate records를 같이 저장함
```python
1. @dlt.expect("valid timestamp", "col(“timestamp”) > '2012-01-01'")
```

```sql
CONSTRAINT valid_timestamp EXPECT (timestamp > '2012-01-01')
```
#### Drop invalid records
- `except or drop` operator 사용
- target dataset에서 violate record가 제거됨
```python
@dlt.expect_or_drop("valid_current_page", "current_page_id IS NOT NULL AND current_page_title IS NOT NULL")
```

```sql
CONSTRAINT valid_current_page EXPECT (current_page_id IS NOT NULL and current_page_title IS NOT NULL) ON VIOLATION DROP ROW
```
#### Fail on invalid records
- `except or fail`
- 비정상 레코드 유입시 즉시 실패
- 이 operator가 table을 Update 한다면, system은 자동으로 roll back 수행
```python
1. @dlt.expect_or_fail("valid_count", "count > 0")
```

```sql
CONSTRAINT valid_count EXPECT (count > 0) ON VIOLATION FAIL UPDATE
```

## Q35
When building a DLT s pipeline you have two options to create a live tables, what is the main difference between `CREATE STREAMING LIVE TABLE` vs `CREATE LIVE TABLE`?
#### A35
- `CREATE STREAMING LIVE TABLE` is used when working with Streaming data sources and Incremental data
- [[DLT Pipeline]]

## Q36
A particular job seems to be performing slower and slower over time, the team thinks this started to happen when a recent production change was implemented, you were asked to take look at the job history and see if we can identify trends and root cause, where in the workspace UI can you perform this analysis?
#### A36
- Under jobs UI select the job you are interested, under runs we can see current active runs and last 60 days historical run
- Job별로 히스토리를 볼 수 있음

## Q37
What are the different ways you can schedule a job in Databricks workspace?
#### A37
-  Cron, On Demand runs
- https://docs.databricks.com/aws/en/jobs#run-a-job

## Q38
You have noticed that Databricks SQL queries are running slow, you are asked to look reason why queries are running slow and identify steps to improve the performance, when you looked at the issue you noticed all the queries are running in parallel and using a SQL endpoint(SQL Warehouse) with a single cluster. Which of the following steps can be taken to improve the performance/response times of the queries?
Please note Databricks recently renamed SQL endpoint to SQL warehouse.
#### A38
- They can increase the maximum bound of the SQL endpoint(SQL warehouse)’s scaling range
	- workspace를 확장하면, 클러스터의 수가 늘어남
- SQL Endpoint(warehouse)를 확장하는 방법
	- 쿼리가 순차적으로 실행되면 scale-up
		- 하나의 클러스터 내에서 worker node 확장
	- 동시에 실행되거나 사용자가 많으면 scale-out
		- 클러스터 확장
- warehouse 생성 중 또는 크기 변경시, 변경 사항 적용을 위해 Warehouse 재시작이 필요할 수 있음

|특징|스케일 아웃 (Scale-out)|스케일 업 (Scale-up)|
|---|---|---|
|**방식**|더 많은 컴퓨팅 단위(클러스터) 추가|단일 컴퓨팅 단위의 리소스(CPU, 메모리 등) 증설|
|**목표**|동시성 처리량 증가, 전체 워크로드 분산|단일 작업 또는 쿼리 성능 향상|
|**Databricks**|주로 클러스터 수의 자동 또는 수동 조절|SQL Warehouse 생성/변경 시 T-shirt size 선택|
|**유연성**|워크로드 변화에 따른 탄력적 대응 용이|최대 용량에 물리적 한계 존재|
|**관리**|분산 시스템 관리 복잡성 존재 가능|상대적으로 단순한 관리|
#### SQL Endpoint(SQL Warehouse) Overview
1. A SQL Warehouse should have at least one cluster
2. A cluster comprises one driver node and one or many worker nodes
3. No of worker nodes in a cluster is determined by the size of the cluster (2X -Small ->1 worker, X-Small ->2 workers.... up to 4X-Large -> 128 workers) this is called **Scale up**
	1. worker node의 수를 증가시키는 방법으로 보임
	2. **2X-Small -> X-Small -> Small -> Medium -> Large**
4. A single cluster irrespective of cluster size(2X-Smal.. to ...4XLarge) can only run 10 queries at any given time if a user submits 20 queries all at once to a warehouse with 3X-Large cluster size and cluster scaling (min 1, max1) while 10 queries will start running the remaining 10 queries wait in a queue for these 10 to finish.
5. Increasing the Warehouse cluster size can improve the performance of a query, for example, if a query runs for 1 minute in a 2X-Small warehouse size it may run in 30 Seconds if we change the warehouse size to X-Small. this is due to 2X-Small having 1 worker node and X-Small having 2 worker nodes so the query has more tasks and runs faster (note: this is an ideal case example, the scalability of a query performance depends on many factors, it can not always be linear)
6. A warehouse can have more than one cluster this is called **Scale out**. If a warehouse is configured with X-Small cluster size with cluster scaling(Min1, Max 2) Databricks spins up an additional cluster if it detects queries are waiting in the queue, If a warehouse is configured to run 2 clusters(Min1, Max 2), and let's say a user submits 20 queries, 10 queriers will start running and holds the remaining in the queue and databricks will automatically start the second cluster and starts redirecting the 10 queries waiting in the queue to the second cluster.
7. A single query will not span more than one cluster, once a query is submitted to a cluster it will remain in that cluster until the query execution finishes irrespective of how many clusters are available to scale.
	1. 단일 쿼리는 두 개 이상의 클러스터에 거렻 있지 않음
	2. 쿼리가 제출되면, 클러스터 수에 관계 없이 쿼리 실행 완료시까지 해당 클러스터에 유지

## Q39
You currently working with the marketing team to setup a dashboard for ad campaign analysis, since the team is not sure how often the dashboard should be refreshed they have decided to do a manual refresh on an as needed basis. Which of the following steps can be taken to reduce the overall cost of the compute when the team is not using the compute?
Please note that Databricks recently change the name of SQL Endpoint to SQL Warehouses.
#### A39
- They can turn on the Auto Stop feature for the SQL endpoint(SQL Warehouse).
- 사용하지 않을 때 Auto stop하는 기능이 존재함

## Q40
You had worked with the Data analysts team to set up a SQL Endpoint(SQL warehouse) point so they can easily query and analyze data in the gold layer, but once they started consuming the SQL Endpoint(SQL warehouse)  you noticed that during the peak hours as the number of users increase you are seeing queries taking longer to finish, which of the following steps can be taken to resolve the issue?

Please note Databricks recently renamed SQL endpoint to SQL warehouse.
#### A40
- They can increase the maximum bound of the SQL endpoint(SQL warehouse) ’s scaling range.

## Q41
The research team has put together a funnel analysis query to monitor the customer traffic on the e-commerce platform, the query takes about 30 mins to run on a small SQL endpoint cluster with max scaling set to 1 cluster. What steps can be taken to improve the performance of the query?
#### A41
- They can increase the cluster size anywhere from X small to 3XL to review the performance and select the size that meets the required SLA.
- 단일 쿼리를 위한 추가 메모리 사용
	- 추가 Worker node는 클러스터에서 더 많은 작업 수행 및 쿼리 성능 향상 가능

## Q42
Unity catalog simplifies managing multiple workspaces, by storing and managing permissions and ACL at _______ level
#### A42
- Account
- classic access control list(tables, workspace, cluster) -> workspace level
- Unity catalog는 account level에 존재하며, account의 모든 workspace 관리 가능

## Q43
Which of the following section in the UI can be used to manage permissions and grants to tables?
#### A43
- Data Explorer
- https://docs.databricks.com/aws/en/catalog-explorer
## Q44
Which of the following is not a privilege in the Unity catalog?
#### A44
- DELETE
- MODIFY를 통해 update, delete permission을 제어해야 함
- Table ACL 권한과 Unity catalog 권한 유형은 다름
	- [Unity Catalog](https://learn.microsoft.com/en-us/azure/databricks/spark/latest/spark-sql/language-manual/sql-ref-privileges#privilege-types)
	- [Table ACL privileges](https://learn.microsoft.com/en-us/azure/databricks/security/access-control/table-acls/object-privileges#privileges)

| 특징        | Table ACLs                       | Unity Catalog 권한                                    |
| --------- | -------------------------------- | --------------------------------------------------- |
| **범위**    | Workspace 단위                     | Account 단위 (중앙 집중식, Workspace 간 공유 가능)              |
| **관리 주체** | Workspace Metastore (Hive 또는 외부) | Unity Catalog Metastore                             |
| **대상 객체** | 주로 Tables, Views                 | Tables, Views, Volumes, Models, Functions, 등 다양한 자산 |
| **세분성**   | 테이블/뷰, 데이터베이스 수준                 | 객체 타입별 다양한 권한, 계층적 상속                               |
| **현 상태**  | 레거시 모델 (Workspace-local 데이터용)    | 최신 권장 모델 (중앙 집중식 거버넌스)                              |

## Q45
A team member is leaving the team and he/she is currently the owner of the few tables, instead of transfering the ownership to a user you have decided to transfer the ownership to a group so in the future anyone in the group can manage the permissions rather than a single individual, which of the following commands help you accomplish this?
#### A45
```sql
ALTER TABLE table_name OWNER to 'group'
```
- [Assign owner to object](https://learn.microsoft.com/en-us/azure/databricks/data-governance/table-acls/object-privileges#assign-owner-to-object)