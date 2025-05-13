---
date: "2025-05-12"
datetime: "2025-05-12 20:36:18"
book: 
page: 
tags: 
references: 
aliases:
---
## Q1
You are designing an analytical to store structured data from your e-commerce platform and unstructured data from website traffic and app store, how would you approach where you store this data?
#### A1
Data lakehouse can store structured and unstructured data and can enforce schema

## Q2
What is the best way to describe a data lakehouse compared to a data warehouse?
#### A2
- A data lakehouse enables both batch and streaming analytics.
- [[Databricks Certified Data Engineer Associate Test 1#LakeHouse의 특징]]

## Q3
Which of the following is a correct statement on how the data is organized in the storage when managing a DELTA table?
#### A3
- All of the data is broken down into one or many parquet files, log files are broken down into one or many JSON files, and each transaction creates a new data file(s) and log file.

#### Delta Table의 파일 구조
- `_delta_log/000000.json`과 같이 여러개의 json 파일 구성
- `_delta_log/0000010.checkpoint.parquet` checkpoint 파일 생성
- Partition directory별 parquet 파일 생성
## Q4
As a Data Engineer, you were asked to create a delta table to store below transaction data?

| transactionId | transactionDate        | unitsSold |
| ------------- | ---------------------- | --------- |
| 1             | 01-01-2021 09:10:24 AM | 100       |
#### A4
```sql
CREATE TABLE transactions (
transactionId int,
transactionDate timestamp,
unitsSold int)
```

## Q5
How does a Delta Lake differ from a traditional data lake?
#### A5
- Delta lake is an open storage format like parquet with additional capabilities that can provide reliability, security, and performance
- [[Databricks Certified Data Engineer Associate Test 1#Deltalake]]
- [[Databricks Certified Data Engineer Associate Test 1#Deltalake is not]]

## Q6
Newly joined data analyst requested read-only access to tables, assuming you are owner/admin which section of Databricks platform is going to facilitate granting select access to the user
#### A6
- Data explorer
	- explore and manage permissions on dbs and tables
	- user: view schema details, preview sample data, see table details, properties
	- admin: [view and change owner](https://docs.databricks.com/aws/en/catalog-explorer#manage-data-object-ownership)
	- admins and data object owner can grant and revoke permissions

## Q7
You noticed that colleague is manually copying the notebook with `_bkp` to store the previous versions, which of the following feature would you recommend instead.
#### A7
- Databricks notebooks support change tracking and versioning
- 우측 상단에 히스토리 버튼을 눌러서, 변경 이력 확인 가능

## Q8
Which of the following describes how Databricks Repos can help facilitate CI/CD workflows on the Databricks Lakehouse Platform?
#### A8
- Databricks Repos can commit or push code changes to trigger a CI/CD process
- Git provider의 기능: Git automation 기능 및 PR, merge main, call databricks api
	- git automation이 git action으로 보임
- 나머지 기능은 모두 databricks에서 작업 가능(user workflow 등)

## Q9
You are currently working on a production job failure with a job set up in job clusters due to a data issue, what cluster do you need to start to investigate and analyze the data?
#### A9
- All-purpose cluster/ interactive cluster is the recommended way to run commands and view the data.
- Job Cluster에서는 interactive 기능을 제공하지 않음
	- Interactive clustser에서 관련 기능 제공
	- display data, view visualization write/edit quries

## Q10
What is the purpose of the bronze layer in a Multi-hop architecture?
#### A10
- Provides efficient storage and querying of full unprocessed history of data
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q11
A dataset has been defined using Delta Live Tables and includes an expectations clause: `CONSTRAINT valid_timestamp EXPECT (timestamp > '2020-01-01') ON VIOLATION FAIL`
What is the expected behavior when a batch of data containing data that violates these constraints is processed?
#### A11
- Records that violate the expectation cause the job to fail
- [[Databricks Certified Data Engineer Associate Test 1#Fail on invalid records]]

## Q12
What is the purpose of a silver layer in Multi hop architecture?
#### A12
- A schema is enforced, with data quality checks.
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q13
What is the purpose of a gold layer in Multi-hop architecture?
#### A13
- Powers ML applications, reporting, dashboards and adhoc reports.
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q14
You are currently asked to work on building a data pipeline, you have noticed that you are currently working on a very large scale ETL many data dependencies, which of the following tools can be used to address this problem?
#### A14
- DELTA LIVE TABLES
- DLT는 data dependencies를 DAG-base로 관리 가능
```sql
create or replace live view customers 
select * from customers;
 
create or replace live view sales_orders_raw
select * from sales_orders;
 
create or replace live view sales_orders_cleaned 
as
select sales.* from 
live.sales_orders_raw s
 join live.customers c 
on c.customer_id = s.customer_id
where c.city = 'LA';
 
create or replace live table sales_orders_in_la
select * from sales_orders_cleaned;
```

#### Delta Live table로 해결 가능한 문제
- Complexities of large scale ETL
	- Hard to build and maintain dependencies
	- Difficult to switch between batch and stream
- Data quality and governance
	- Difficult to monitor and enforce data quality
	- Impossible to trace data lineage
- Difficult pipeline operations
	- Poor observability at granular data level
	- Error handling and recovery is laborious

## Q15
How do you create a delta live tables pipeline and deploy using DLT UI?
#### A15
- Within the Workspace UI, click on Workflows, select Delta Live tables and create a pipeline and select the notebook with DLT code.
- DLT는 workspace UI에서 생성

## Q16
You are noticing job cluster is taking 6 to 8 mins to start which is delaying your job to finish on time, what steps you can take to reduce the amount of time cluster startup time
#### A16
- Use cluster pools to reduce the startup time of the jobs
- Cluster pool을 사용하면 새로운 Job cluster가 생성될 때, VM을 미리 가져와 예약 가능
	- VM을 사용하기 전에는 Azure 사용 비용만 청구되며
	- Databricks run time cost는 VM이 cluster에 할당된 이후엠나 발생

## Q17
Data engineering team has a job currently setup to run a task load data into a reporting table every day at 8:00 AM takes about 20 mins, Operations teams are planning to use that data to run a second job, so they access latest complete set of data. What is the best to way to orchestrate this job setup?
#### A17
- Add Operation reporting task in the same job and set the operations reporting task to depend on Data Engineering task
- DE task -> Operation task 의존성 연결


## Q18
The data engineering team noticed that one of the job normally finishes in 15 mins but gets stuck randomly when reading remote databases due to a network packet drop, which of the following steps can be used to improve the stability of the job?
#### A18
- Modify the task, to include a timeout to kill the job if it runs more than 15 mins.
- https://learn.microsoft.com/en-us/azure/databricks/jobs/#timeout

## Q19
Which of the following programming languages can be used to build a Databricks SQL dashboard?
#### A19
- SQL

## Q20
The data analyst team had put together queries that identify items that are out of stock based on orders and replenishment but when they run all together for final output the team noticed it takes a really long time, you were asked to look at the reason why queries are running slow and identify steps to improve the performance and when you looked at it you noticed all the code queries are running sequentially and using a SQL endpoint cluster. Which of the following steps can be taken to resolve the issue?

```sql
--- Get order summary 
create or replace table orders_summary
as 
select product_id, sum(order_count) order_count
from 
 (
  select product_id,order_count from orders_instore
  union all 
  select product_id,order_count from orders_online
 )
group by product_id
-- get supply summary 
create or repalce tabe supply_summary
as 
select product_id, sum(supply_count) supply_count
from supply
group by product_id
 
-- get on hand based on orders summary and supply summary
 
with stock_cte
as (
select nvl(s.product_id,o.product_id) as product_id,
	 nvl(supply_count,0) -  nvl(order_count,0) as on_hand
from supply_summary s 
full outer join orders_summary o
        on s.product_id = o.product_id
)
select *
from 
stock_cte
where on_hand = 0 
```
#### A20
- Increase the cluster size of the SQL endpoint.
- sequencially하게 수행된다면 scale-up
	- concurrently(More user)라면 scale-out

## Q21
The operations team is interested in monitoring the recently launched product, team wants to set up an email alert when the number of units sold increases by more than 10,000 units. They want to monitor this every 5 mins.
```md
· Create ___ query that calculates total units sold

· Setup ____ with query on trigger condition Units Sold > 10,000

· Setup ____ to run every 5 mins

· Add destination ______
```
#### A21
- SQL, Alert, Refresh, email address

## Q22
The marketing team is launching a new campaign to monitor the performance of the new campaign for the first two weeks, they would like to set up a dashboard with a refresh schedule to run every 5 minutes, which of the below steps can be taken to reduce of the cost of this refresh over time?
#### A22
- Setup the dashboard refresh schedule to end in two weeks

## Q23
Which of the following tool provides Data Access control, Access Audit, Data Lineage, and Data discovery?
#### A23
- Unity Catalog

## Q24
Data engineering team is required to share the data with Data science team and both the teams are using different workspaces in the same organization which of the following techniques can be used to simplify sharing data across?
Please note the question is asking how data is shared within an organization across multiple workspaces.

#### A24
- Unity Catalog

#### Unity Catalog
- Unify governance across clouds
- Unify data and AI assets
	- share, audit, secure, manager all data types
- Unify existing catalogs
	- works in concert with data, storage, catalogs

#### Unity Catalog의 운영
- Account Console
	- Admin이 metastore를 생성
	- User metastore에 사용자들이 접근
	- Unity Catalog = Users + Metastore
- 해당 User Metastore는 Workspace에서 권한 관리에서 사용됨
	- Catalog, schemas, tables, views, ...

## Q25
A newly joined team member John Smith in the Marketing team who currently does not have any access to the data requires read access to customers table, which of the following statements can be used to grant access.
#### A25
```sql
GRANT SELECT, USAGE ON TABLE customers TO john.smith@marketing.com
```

## Q26
Grant full privileges to new marketing user Kevin Smith to table sales
#### A26
```sql
GRANT ALL PRIVILEGES ON TABLE sales TO kevin.smith@marketing.com
```
#### Grant Privileges
- SELECT: read
- CREATE: create an object(e.g. table in a schema)
- MODIFY: add, delete, modify data to or from an object
- USAGE: does not give any abilities
	- but is an additional requirement to perform any action on a schema object
- READ_METADATA: view an object and its metadata
- CREATE_NAMED_FUNCTION: create a named UDF in an existing catalog or schema
- MODIFY_CLASSPATH: add files to the spark class path
- ALL_PRIVILEGES


## Q27
Which of the following locations in the Databricks product architecture hosts the notebooks and jobs?
#### A27
- Control plane
- [[Databricks Certified Data Engineer Associate Test 1#Control Plane]]

## Q28
What could be the expected output of query `SELECT COUNT (DISTINCT *) FROM user` on this table
![[Pasted image 20250513225106.png]]
#### A28
- 2
- `DISTINCT *`는 null value가 있는 레코드 모두 제외

## Q29
You are still noticing slowness in query after performing optimize which helped you to resolve the small files problem, the column(transactionId) you are using to filter the data has high cardinality and auto incrementing number. Which delta optimization can you enable to filter data effectively based on this column?
#### A29
- Perform Optimize with Z-order on transactionId
- data가 자연적으로 정렬되면 file들이 스캔할때 필요한 내용만 spark memory에 올라가게 됨

## Q30
If you create a database sample_db with the statement `CREATE DATABASE sample_db` what will be the **default** location of the database in DBFS?
#### A30
- Default Location, `dbfs:/user/hive/warehouse`

## Q31
Which of the following results in the creation of an external table?
#### A31
```sql
CREATE TABLE transactions (id int, desc string) LOCATION '/mnt/delta/transactions'
```
- location을 사용하면 External table

## Q32
When you drop an external DELTA table using the SQL Command `DROP TABLE table_name`, how does it impact metadata(delta log, history), and data stored in the storage?
#### A32
- Drops table from metastore, but keeps metadata(delta log, history)and data in storage
- external table 제거시 Metastore에서 테이블 정의만 사라짐
	- 데이터와 메타 데이터를 포함한 모든 것이 storage에 남게 됨(trasnaction log, time travel history, delta log)
## Q33
Which of the following is a true statement about the global temporary view?
#### A33
- A global temporary view is available only on the cluster it was created, when the cluster restarts global temporary view is automatically dropped.
- [[Databricks Certified Data Engineer Associate Test 1#Temporary view의 두가지 유형]]

## Q34
You are trying to create an object by joining two tables that and it is accessible to data scientist’s team, so it does not get dropped if the cluster restarts or if the notebook is detached. What type of object are you trying to create?
#### A34
- View
	- join multiple tables
	- persist into meta stores so others can accesses it

## Q35
What is the best way to query external csv files located on DBFS Storage to inspect the data using SQL?
#### A35
```sql
SELECT * FROM CSV.'dbfs:/location/csv_files/'
```

#### Query External files
```sql
SELECT * FROM format.`/Location`
format - CSV, JSON, PARQUET, TEXT
```

## Q36
Direct query on external files limited options, create external tables for CSV files with header and pipe delimited CSV files, fill in the blanks to complete the create table statement
```sql
CREATE TABLE sales (id int, unitsSold int, price FLOAT, items STRING)
________
________
LOCATION “dbfs:/mnt/sales/*.csv”
```
#### A36
```sql
USING CSV
OPTIONS ( header =“true”, delimiter = ”|”)
```

#### External table with additional options
```sql
CREATE TABLE table_name (col_name1 col_typ1,..)
USING data_source
OPTIONS (key=’value’, key2=vla2)
LOCATION = “/location“
```

## Q37
At the end of the inventory process, a file gets uploaded to the cloud object storage, you are asked to build a process to ingest data which of the following method can be used to ingest the data incrementally, schema of the file is expected to change overtime ingestion process should be able to handle these changes automatically. Below is the auto loader to command to load the data, fill in the blanks for successful execution of below code.
```python
spark.readStream
.format("cloudfiles")
.option("_______",”csv)
.option("_______", ‘dbfs:/location/checkpoint/’)
.load(data_source)
.writeStream
.option("_______",’ dbfs:/location/checkpoint/’)
.option("_______", "true")
.table(table_name))
```
#### A37
- `cloudfiles.format, cloudfiles.schemalocation, checkpointlocation, mergeSchema`
- [Auto Loader options](https://docs.databricks.com/aws/en/ingestion/cloud-object-storage/auto-loader/options)
- cloudfiles.schemalocation
	- The location to store the inferred schema and subsequent changes
- mergeSchema
	- Infer the schema across multiple files and to merge the schema of each file. 
	- Enabled by default for Auto Loader when inferring the schema.

## Q38
You are working on a table called **orders** which contains data for 2021 and you have the second table called **orders_archive** which contains data for 2020, you need to combine the data from two tables and there could be a possibility of the same rows between both the tables, you are looking to combine the results from both the tables and eliminate the duplicate rows, which of the following SQL statements helps you accomplish this?
#### A38
```sql
SELECT * FROM orders UNION SELECT * FROM orders_archive
```
- `UNION`: 중복 제거
- `UNION ALL`: 중복 포함

## Q39
Which of the following python statement can be used to replace the schema name and table name in the query statement?
#### A39
```python
table_name = "sales"
schema_name = "bronze"
query = f"select * from { schema_name}.{table_name}"
```

## Q40
Which of the following SQL statements can replace python variables in Databricks SQL code, when the notebook is set in SQL mode?
```python
%python 
table_name = "sales"
schema_name = "bronze"
 
%sql
SELECT * FROM ____________________
```
#### A40
```sql
SELECT * FROM ${schema_name}.${table_name}
```
- `${python variable}` -> Python variables in Databricks SQL code

## Q41
A notebook accepts an input parameter that is assigned to a python variable called `department` and this is an optional parameter to the notebook, you are looking to control the flow of the code using this parameter. you have to check department variable is present then execute the code and if no department value is passed then skip the code execution. How do you achieve this using python?
#### A41
```python
if department is not None:
  #Execute code
else:
  pass
```

## Q42
Which of the following operations are not supported on a streaming dataset view?
`spark.readStream.format("delta").table("sales").createOrReplaceTempView("streaming_view")`
#### A42
```sql
SELECT * FROM streadming_view ORDER BY id
```
- streaming_view는 전체 `ORDER BY`는 지원하지 않음
	- `GROUP BY`이후 `ORDER BY`는 지원함
- `max, sum`과 같은 agg Function은 모두 지원함

## Q43
Which of the following techniques structured streaming uses to ensure recovery of failures during stream processing?
#### A43
- Checkpointing and write-ahead logging
- c.f. [[Databricks Certified Data Engineer Associate Test 1#structured steaming의 fault tolerance]]
	- fault tolerance: Checkpointing, Idempotent Sink
	- ensure recovery of failures: Checkpointing, write-ahead logging

## Q44
What is the underlying technology that makes the Auto Loader work?
#### A44
- Structured Streaming
- Auto loader는 Structured Streaming 기반에서 동작함
	- source called `cloudFiles`
	- input dir을 직접 `cloud file storage` 기입
	- 자동으로 새로운 파일을 감지

## Q45
You are currently working to ingest millions of files that get uploaded to the cloud object storage for consumption, and you are asked to build a process to ingest this data, the schema of the file is expected to change over time, and the ingestion process should be able to handle these changes automatically. Which of the following method can be used to ingest the data incrementally?
#### A45
- AUTO LOADER
- [[Databricks Certified Data Engineer Associate Test 1#Auto Loader vs Copy Into]]
