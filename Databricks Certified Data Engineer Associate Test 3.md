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

