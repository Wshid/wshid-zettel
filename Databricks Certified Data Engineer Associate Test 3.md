---
date: "2025-05-14"
datetime: "2025-05-14 20:48:48"
book: 
page: 
tags: 
references: 
aliases:
---
## Q1
Which of the statements are **incorrect** when choosing between lakehouse and Datawarehouse?
#### A1
- Lakehouse cannot serve low query latency with high reliability for BI workloads, only suitable for batch workloads.
- [Spark Cache vs Delta Cache](https://docs.databricks.com/aws/en/optimizations/disk-cache)
- [[Databricks Certified Data Engineer Associate Test 1#LakeHouse의 특징]]

## Q2
Which of the statements are correct about lakehouse?

#### A2
- Lakehouse supports schema enforcement and evolution

#### Dataframe writer API의 3가지 모드
- Default
	- 적용만 가능. 변경은 허용되지 않음
	- schema drift/evolution은 실패함
- Merge
	- enforcement and evolution을 모두 지원
	- 새로운 컬럼 추가
	- nested columns
	- evolving data types, e.g. Byte, Short, Integer
```python
DF.write.format("delta").option("mergeSchema", "true").saveAsTable("table_name")
                         or 
spark.databricks.delta.schema.autoMerge = True  ## Spark session
```
- Overwrite
	- no enforcement
	- Dropping columns
	- Change string to integer
	- Rename columns
```python
DF.write.format("delta").option("overwriteSchema", "True").saveAsTable("table_name")
```

## Q3
Which of the following are stored in the control pane of Databricks Architecture?
#### A3
- Databricks Web Application
- [[Databricks Certified Data Engineer Associate Test 1#Control Plane]]
	- Databricks Web application
	- Customer Notebooks
	- Jobs and Queries
	- Cluster Management
- Data Plane
	- Data processing with Apache Spark cluster
	- DBFS(Databricks File System)

## Q4
You have written a notebook to generate a summary data set for reporting, Notebook was scheduled using the job cluster, but you realized it takes 8 minutes to start the cluster, what feature can be used to start the cluster in a timely fashion so your job can run immediatley?
#### A4
- Use the Databricks cluster pools feature to reduce the startup time
- [[Databricks Certified Data Engineer Associate Test 2#Cluster Pool의 사용]]

## Q5
Which of the following developer operations in the CI/CD can only be implemented through a GIT provider when using Databricks Repos.
#### A5
- Pull request and review process
- [[Databricks Certified Data Engineer Associate Test 1#CI/CD Databricks Repo]]

## Q6
You have noticed the Data scientist team is using the notebook versioning feature with git integration, you have recommended them to switch to using Databricks Repos, which of the below reasons could be the reason the why the team needs to switch to Databricks Repos.
#### A6
- Databricks Repos allow you to add comments and select the changes you want to commit.

## Q7
Data science team members are using a single cluster to perform data analysis, although cluster size was chosen to handle multiple users and auto-scaling was enabled, the team realized queries are still running slow, what would be the suggested fix for this?
#### A7
- Use High concurrency mode instead of the standard mode
- High Concurrency clusters are ideal for groups of users who need to share resources or run ad-hoc jobs.  Databricks recommends enabling autoscaling for High Concurrency clusters.

## Q8
Which of the following SQL commands are used to append rows to an existing delta table?
#### A8
```sql
INSERT INTO table_name
```


## Q9
How are Delta tables stored?
#### A9
- A Directory where parquet data files are stored, a sub directory `_delta_` log where meta data, and the transaction log is stored as JSON files.
- [[Databricks Certified Data Engineer Associate Test 2#Delta Table의 파일 구조]]

## Q10
While investigating a data issue in a Delta table, you wanted to review logs to see when and who updated the table, what is the best way to review this data?
#### A10
- Run SQL command `DESCRIBE HISTORY table_name`

## Q11
While investigating a performance issue, you realized that you have too many small files for a given table, which command are you going to run to fix this issue
#### A11
- `OPTIMIZE table_name`

#### OPTIMIZE 구문
- small parquet files to bigger file
- Optimze 명령 시점의 테이블 크기에 따라 파일 크기가 결정됨
- Databricks는 자동으로 테이블의 크기를 조절함

## Q12
Create a sales database using the DBFS location `'dbfs:/mnt/delta/databases/sales.db/'`
#### A12
```sql
CREATE DATABASE sales LOCATION 'dbfs:/mnt/delta/databases/sales.db/'
```
- Unity catalog에서는 3단계의 이름으로 구분하며
	- `SCHEMA, DATABASE`는 상호 동일한 의미를 가짐

## Q13
What is the type of table created when you issue SQL DDL command `CREATE TABLE sales (id int, units int)`
#### A13
- Managed Delta table

## Q14
How to determine if a table is a managed table vs external table?
#### A14
- Run SQL command `DESCRIBE EXTENDED table_name` and check type

## Q15
Which of the below SQL commands creates a session scoped temporary view?
#### A15
```sql
CREATE OR REPLACE TEMPORARY VIEW view_name
AS SELECT * FROM table_name
```

## Q16
Drop the customers database and associated tables and data, all of the tables inside the database are managed tables. Which of the following SQL commands will help you accomplish this?
#### A16
```sql
DROP DATABASE customers CASCADE
```
- `CASCADE`를 사용하면 DB내 모든 table도 drop
- 테이블들이 모두 MANAGED TABLE 이었기 때문에 데이터 정리를 위해 추가 작업은 필요 없음

## Q17
Define an external SQL table by connecting to a local instance of an SQLite database using JDBC
#### A17
```sql

CREATE TABLE users_jdbc
USING org.apache.spark.sql.jdbc
OPTIONS (
    url = "jdbc:sqlite:/sqmple_db",
    dbtable = "users"
)
```

#### JDBC를 활용한 외부 테이블 생성
```sql
CREATE TABLE <jdbcTable>
USING org.apache.spark.sql.jdbc or JDBC
OPTIONS (
    url = "jdbc:<databaseServerType>://<jdbcHostname>:<jdbcPort>",
    dbtable " = <jdbcDatabase>.atable",
    user = "<jdbcUsername>",
    password = "<jdbcPassword>"
)
```

## Q18
When defining external tables using formats CSV, JSON, TEXT, BINARY any query on the external tables caches the data and location for performance reasons, so within a given spark session any new files that may have arrived will not be available after the initial query. How can we address this limitation?
#### A18
```sql
REFRESH TABLE table_name
```
- 외부 테이블 정의시, 관련 내용이 캐시되어 새 파일이 보이지 않는다는 의미
- `REFRESH`를 통해 새로운 파일을 불러올 수 있음

## Q19
Which of the following table constraints that can be enforced on Delta lake tables are supported?
#### A19
- Not Null, Check Constraints
- [Delta Contraints](https://learn.microsoft.com/en-us/azure/databricks/tables/constraints)
- [ADD CONSTRAINT clauses](https://learn.microsoft.com/en-us/azure/databricks/sql/language-manual/sql-ref-syntax-ddl-alter-table-add-constraint)

```sql
ALTER TABLE events CHANGE COLUMN id SET NOT NULL;
ALTER TABLE events ADD CONSTRAINT dateWithinRange CHECK (date > '1900-01-01');
```
- DBR 11.1의 경우 Unity Catalog가 존재할 경우
	- Primary key, Foreign Key 지원을 추가했지만, 실제 적용되지는 x

## Q20
The data engineering team is looking to add a new column to the table, but the QA team would like to test the change before implementing in production, which of the below options allow you to quickly copy the table from Prod to the QA environment, modify and run the tests?
#### A20
- `SHALLOW CLONE`

#### SHALLOW CLONE
- 현재 테이블을 수정하지 않으면서 test를 위한 복제를 빠르게 수행할 때 사용
- Delta transaction logs만 복제하며, 데이터를 이동시키지 않음

#### DEEP CLONE
- data, metadata를 실제 복제 진행
- copy는 점진적으로 일어나며 이 명령을 재실행하면 소스에서 대상 위치로 변경 사항 동기화 가능
- 모든 data, transaction log를 복제하므로 테이블 크기에 따라 오래 걸릴 수 있음

## Q21
Sales team is looking to get a report on a measure number of units sold by date, below is the schema. Fill in the blank with the appropriate array function.
- Table **orders**: `orderDate DATE, orderIds ARRAY<INT>`
- Table **orderDetail**: `orderId INT, unitsSold INT, salesAmt DOUBLE`
```sql
SELECT orderDate, SUM(unitsSold)
      FROM orderDetail od
JOIN (select orderDate, ___________(orderIds) as orderId FROM orders) o
    ON o.orderId = od.orderId
GROUP BY orderDate
```
#### A21
-   EXPLODE

## Q22
You are asked to write a python function that can read data from a delta table and return the DataFrame, which of the following is correct?
#### A22
- Python function can return a DataFrame
```python
get_source_dataframe(tablename):
	df = spark.read.table(tablename)
	return df
```

## Q23
What is the output of the below function when executed with input parameters 1, 3  :
```python
def check_input(x,y):
    if x < y:
        x= x+1
        if x<y:
            x= x+1
            if x <y:
                x = x+1
     return x
```
#### A23
- 3

## Q24
Which of the following SQL statements can replace a python variable, when the notebook is set in SQL mode
#### A24
```sql
spark.sql(f"SELECT * FROM {schema_name}.{table_name}")
```

## Q25
When writing streaming data, Spark’s structured stream supports the below write modes
#### A25
- Append, Complete, Update

#### Spark structured stream write mode
- Append(default)
	- 마지막 트리거 이후, 결과 테이블에 추가된 새 행만 싱크에 출력
	- 결과 테이블에 추가된 행이 변경되지 않는 쿼리만 지원함
	- 각 행이 한번만 출력되도록 보장(assuming fault-tolerant sink)
	- e.g. `select, where, map, flatMap, filter, join` 지원
- Complete
	- 전체 결과 테이블은 매 트리거 후에 sink 출력
		- 매번 현재의 전체 테이블 출력(overwrite)
	- 집계 쿼리에 대해 지원
- Update
	- Spark 2.1.1 부터 지원
	- 마지막 트리거 후, 업데이트 된 결과 테이블의 행만 Sink에 출력

## Q26
When using the complete mode to write stream data, how does it impact the target table?
#### A26
-   Target table is overwritten for each batch

## Q27
At the end of the inventory process a file gets uploaded to the cloud object storage, you are asked to build a process to ingest data which of the following method can be used to ingest the data incrementally, the schema of the file is expected to change overtime ingestion process should be able to handle these changes automatically. Below is the auto loader command to load the data, fill in the blanks for successful execution of the below code.
```sql
spark.readStream
.format("cloudfiles")
.option("cloudfiles.format",”csv)
.option("_______", ‘dbfs:/location/checkpoint/’)
.load(data_source)
.writeStream
.option("_______",’ dbfs:/location/checkpoint/’)
.option("mergeSchema", "true")
.table(table_name))
```
#### A27
  - `cloudfiles.schemalocation, checkpointlocation`
	  - read시, 들어오는 데이터의 추론된 스키마를 저장하는데 사용
	  - Write시, 장애 복구시 사용. 가장 최근에 처리된 바이트 오프셋 저장
## Q28
When working with AUTO LOADER you noticed that most of the columns that were inferred as part of loading are string data types including columns that were supposed to be integers, how can we fix this?
#### A28
- Provide schema hints

#### Schema Hints
```python
spark.readStream \
  .format("cloudFiles") \
  .option("cloudFiles.format", "csv") \
  .option("header", "true") \
  .option("cloudFiles.schemaLocation", schema_location) \
  .option("cloudFiles.schemaHints", "id int, description string")
  .load(raw_data_location)
  .writeStream \
  .option("checkpointLocation", checkpoint_location) \
  .start(target_delta_table_location)
```
- `cloudFiles.schemaLocation`이 지정된 경우, 스키마 힌트를 사용하여 이미 알려진 열에 대한 데이터 유형 적용 가능

## Q29
You have configured AUTO LOADER to process incoming IOT data from cloud object storage every 15 mins, recently a change was made to the notebook code to update the processing logic but the team later realized that the notebook was failing for the last 24 hours, what steps team needs to take to reprocess the data that was not loaded after the notebook was corrected?
#### A29
- Autoloader automatically re-processes data that was not loaded
- checkpoint를 사용하여 로드되지 않은 데이터 자동 로드

## Q30
Which of the following Structured Streaming queries is performing a hop from a bronze table to a Silver table?
#### A30
```python
(spark.table("sales")
.withColumn("avgPrice", col("sales") / col("units"))
.writeStream
.option("checkpointLocation", checkpointPath)
.outputMode("append") 
.table("cleanedSales"))
```
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q31
Which of the following Structured Streaming queries successfully performs a hop from a Silver to Gold table?
#### A31
```python
(spark.table("sales") 
.groupBy("store") 
.agg(sum("sales")) 
.writeStream 
.option("checkpointLocation", checkpointPath) 
.outputMode("complete") 
.table("aggregatedSales") ) 
```
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q32
Which of the following Auto loader structured streaming commands successfully performs a hop from the landing area into Bronze?
#### A32
```python
spark\
.readStream\
.format("cloudFiles")\
.option("cloudFiles.format","csv")\
.option("cloudFiles.schemaLocation", checkpoint_directory)\
.load("landing")\
.writeStream.option("checkpointLocation", checkpoint_directory)\
.table(raw)
```
- readStream을 사용해야 함
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q33
A DELTA LIVE TABLE pipelines can be scheduled to run in two different modes, what are these two different modes?
#### A33
-   Triggered, Continuous

#### DELTA LIVE TABLE mode
- **Triggered**
	- 현재 사용 가능한 모든 데이터로 각 테이블을 업데이트
	- 다음 파이프라인을 실행하는 클러스터 중지
	- 테이블 간 종속성을 자동으로 분석. 외부 소스에서 읽는 테이블을 계산하는 것으로 시작
	- 파이프라인 내의 테이블 종속 데이터 소스가 업데이트된 후에 실행
- **Continuous**
	- 입력 데이터가 변경되면 테이블을 지속적으로 업데이트
	- 업데이트가 시작되면 수동으로 중지할때까지 계속 실행
	- 항상 실행되는 클러스터가 필요하나, 다운 스트림 소비자에게 최신 데이터 제공

## Q34
Your team member is trying to set up a delta pipeline and build a second gold table to the same pipeline with aggregated metrics based on an existing Delta Live table called sales_orders_cleaned but he is facing a problem in starting the pipeline, the pipeline is failing to state it cannot find the table sales_orders_cleaned, you are asked to identify and fix the problem.
```sql
CREATE LIVE TABLE sales_order_in_chicago
AS
SELECT order_date, city, sum(price) as sales,
FROM sales_orders_cleaned
WHERE city = 'Chicago')
GROUP BY order_date, city
```
#### A34
- sales_orders_cleaned table is missing schema name LIVE
- `LIVE`가 빠졌음
	- 모든 Delta live table에는 `LIVE` 스키마가 잇어야 함
```sql
CREATE LIVE TABLE sales_order_in_chicago
AS
SELECT order_date, city, sum(price) as sales,
FROM LIVE.sales_orders_cleaned
WHERE city = 'Chicago')
GROUP BY order_date, city
```

## Q35
Which of the following type of tasks cannot setup through a job?
#### A35
- Databricks SQL Dashboard refresh

## Q36
Which of the following approaches can the data engineer use to obtain a version-controllable configuration of the Job’s schedule and configuration?
#### A36
- They can download the JSON equivalent of the job from the Job’s page.

## Q37
What steps need to be taken to set up a DELTA LIVE PIPELINE as a job using the workspace UI?
#### A37
- Select Workflows UI and Delta live tables tab, under task type select Delta live tables pipeline and select the notebook

## Q38
Data engineering team has provided 10 queries and asked Data Analyst team to build a dashboard and refresh the data every day at 8 AM, identify the best approach to set up data refresh for this dashaboard?
#### A38
- The entire dashboard with 10 queries can be refreshed at once, single schedule needs to be set up to refresh at 8 AM.

## Q39
The data engineering team is using a SQL query to review data completeness every day to monitor the ETL job, and query output is being used in multiple dashboards which of the following approaches can be used to set up a schedule and automate this process?
#### A39
- They can schedule the query to refresh every day from the query’s page in Databricks SQL
- 예약된 쿼리 실행을 사용하여 대시보드 지속 업데이트 혹은 정기적 알림 설정 가능
- Databricks SQL workspace는 개별 쿼리를 추가, 편집, 실행 예약도 가능함

## Q40
A data engineer is using a Databricks SQL query to monitor the performance of an ELT job. The ELT job is triggered by a specific number of input records being ready to process. The Databricks SQL query returns the number of minutes since the job’s most recent runtime. Which of the following approaches can enable the data engineering team to be notified if the ELT job has not been run in an hour?
#### A40
- They can set up an Alert for the query to notify them if the returned value is greater than 60.
- 대시보드가 아닌 쿼리에서 설정할 수 있음
	- 쿼리는 알림을 트리거할 수 있는 경우에 사용되는 값 반환 가능

## Q41
Which of the following is true, when building a Databricks SQL dashboard?
#### A41
-   More than one visualization can be developed using a single query result

## Q42
A newly joined team member John Smith in the Marketing team currently has access read access to sales tables but does not have access to update the table, which of the following commands help you accomplish this?
#### A42
```sql
GRANT MODIFY ON TABLE table_name TO john.smith@marketing.com
```
- https://learn.microsoft.com/en-us/azure/databricks/data-governance/table-acls/object-privileges#privileges

## Q43
A new user who currently does not have access to the catalog or schema is requesting access to the customer table in sales schema, but the customer table contains sensitive information, so you have decided to create view on the table excluding columns that are sensitive and granted access to the view GRANT SELECT ON view_name to user@company.com but when the user tries to query the view, gets the error view does not exist. What is the issue preventing user to access the view and how to fix it?
#### A43
- User requires USAGE privilege on Sales schema

## Q44
How do you access or use tables in the unity catalog?
#### A44
- `catalog_name.schema_name.table_name`

## Q45
How do you upgrade an existing workspace managed table to a unity catalog table?
#### A45
```sql
Create table catalog_name.schema_name.table_name
as select * from hive_metastore.old_schema.old_table
```