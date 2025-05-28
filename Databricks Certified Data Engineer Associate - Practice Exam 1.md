---
date: "2025-05-27"
datetime: "2025-05-27 21:45:04"
book: 
page: 
tags: 
references: 
aliases:
---
## Q1
Which of the following commands can a data engineer use to compact small data files of a Delta table into larger ones ?
#### A1
OPTIMIZE

## Q2
A data engineer is trying to use Delta time travel to rollback a table to a previous version, but the data engineer received an error that the data files are no longer present.

  
Which of the following commands was run on the table that caused deleting the data files?
#### A2
VACUUM

## Q3
In Delta Lake tables, which of the following is the primary format for the data files?
#### A3
Parquet
- JSON 파일 형태의 Transaction log
- Parquet 형식의 데이터 파일

## Q4
Which of the following locations hosts the Databricks web application?
#### A4
Control plane
- https://docs.databricks.com/aws/en/getting-started/overview
- [[Databricks Certified Data Engineer Associate Test 1#Control Plane]]

## Q5
In Databricks Repos (Git folders), which of the following operations a data engineer can use to update the local version of a repo from its remote Git repository ?
#### A5
Pull

## Q6
According to the Databricks Lakehouse architecture, which of the following is located in the customer's cloud account?
#### A6
Cluster virtual machines
- Spark cluster등을 설정할 경우, cluster VM들은 Data plane에 배포(in the customer's cloud account)

## Q7
Which of the following best describes Databricks Lakehouse?
#### A7
Single, flexible, high-performance system that supports data, analytics, and machine learning workloads.

## Q8
If ​​the default notebook language is SQL, which of the following options a data engineer can use to run a Python code in this SQL Notebook ?
#### A8
They can add `%python` at the start of a cell.

## Q9
Which of the following tasks is not supported by Databricks Repos (Git folders), and must be performed in your Git provider ?
#### A9
Delete branches
#### GitProvider에서만 제공하는 기능
- Create a pull Request
- Delete branches
- Merge and rebase branches
	- 현재 곧 제공 가능함
	- 당시 시험 버전에서는 불가함

## Q10
Which of the following statements is **Not** true about Delta Lake ?
#### A10
Delta Lake builds upon standard data formats: Parquet + XML
- Parquet + JSON

## Q11
How long is the default retention period of the VACUUM command ?
#### A11
- 7 days

## Q12
The data engineering team has a Delta table called **employees** that contains the employees personal information including their gross salaries.

Which of the following code blocks will keep in the table only the employees having a salary greater than 3000 ?
#### A12
```sql
DELETE FROM employees WHERE salary <= 3000;
```

## Q13
A data engineer wants to create a relational object by pulling data from two tables. The relational object must be used by other data engineers in other sessions on the same cluster only. In order to save on storage costs, the date engineer wants to avoid copying and storing physical data.

Which of the following relational objects should the data engineer create?
#### A13
Global Temporary view
- 실제 데이터 복사 및 저장 X -> View 사용
- View: 물리적 데이터가 없는 가상 테이블
	- 실제 테이블에 대한 SQL
- same cluster, other session -> Global temporary view

## Q14
A data engineer has developed a code block to completely reprocess data based on the following if-condition in Python:
```python
if process_mode = "init" and not is_table_exist:
   print("Start processing ...")
```

This if-condition is returning an invalid syntax error.
Which of the following changes should be made to the code block to fix this error ?
#### A14
```python
if process_mode == "init" and not is_table_exist:
    print("Start processing ...")
```

## Q15
Fill in the below blank to successfully create a table in Databricks using data from an existing PostgreSQL database:

```sql
CREATE TABLE employees
  USING ____________
  OPTIONS (
    url "jdbc:postgresql:dbserver",
    dbtable "employees"
  )
```
#### A15
- `org.apache.spark.sql.jdbc`

## Q16
Which of the following commands can a data engineer use to create a new table along with a comment ?
#### A16
```sql
CREATE TABLE payments
COMMENT "This table contains sensitive information"
AS SELECT * FROM bank_transactions
```
- COMMENT는 CTAS시에 CT이후 바로 사용 필요

## Q17
A junior data engineer usually uses `INSERT INTO` command to write data into a Delta table. A senior data engineer suggested using another command that avoids writing of duplicate records.

Which of the following commands is the one suggested by the senior data engineer ?
#### A17
`MERGE INTO`
- 소스 테이블을 기반으로 update, insert, delete를 병합할 수 있음
- 델타 테이블에 쓸 때 중복 레코드 삽입 방지 가능

## Q18
A data engineer is designing a Delta Live Tables pipeline. The source system generates files containing changes captured in the source data. Each change event has metadata indicating whether the specified record was inserted, updated, or deleted. In addition to a timestamp column indicating the order in which the changes happened. The data engineer needs to update a target table based on these change events.

Which of the following commands can the data engineer use to best solve this problem?
#### A18
- `APPLY CHANGES INTO`

#### APPLY CHANGES INTO
- CDC(Change Data Capture)
- 메타 데이터 정보와 함께 레코드의 데이터를 모두 포함하는 이벤트로 소스에 기록
	- Operating column: 지정된 레코드가 삽입, 업데이트, 삭제되었는지 여부를 나타냄
	- Sequence column: 변경이 발생한 순서를 나타내는 timestamp
- Delta Table의 CDC를 사용하기 위해서 `APPLY CHANGES INTO` 구문 사용 가능

## Q19
In PySpark, which of the following commands can you use to query the Delta table **employees** created in Spark SQL?
#### A19
- `spark.table("employees")`

## Q20
Which of the following code blocks can a data engineer use to create a user defined function (UDF) ?
#### A20
```sql
CREATE FUNCTION plus_one(value INTEGER)
RETURNS INTEGER
RETURN value +1;
```
- UDF시 키워드는 `CREATE FUNCTION`
- 파라미터 및 리턴 타입, 리턴 인자 필요
	- RETURNS, RETURN

## Q21
When dropping a Delta table, which of the following explains why only the table's metadata will be deleted, while the data files will be kept in the storage ?
#### A21
The table is external
- 메타 데이터는 지워지지만 데이터는 남음

## Q22
Given the two tables **students_course_1** and **students_course_2**. Which of the following commands can a data engineer use to get all the students from the above two tables without duplicate records ?
#### A22
```sql
SELECT * FROM students_course_1
UNION
SELECT * FROM students_course_2
```

## Q23
Given the following command:

`CREATE DATABASE IF NOT EXISTS hr_db ;`

In which of the following locations will the **hr_db** database be located?
#### A23
`dbfs:/user/hive/warehouse`
- LOCATION이 지정되지 않으면 기본적으로 `warehouse`하위에 위치

## Q24
Given the following table **faculties**

Fill in the below blank to get the students enrolled in less than 3 courses from the array column **students**
```sql
SELECT
  faculty_id,
  students,
  ___________ AS few_courses_students
FROM faculties
```
#### A24
```sql
FILTER (students, i -> i.total_courses < 3)
```

## Q25
Given the following Structured Streaming query:
```python
(
	spark.table("orders")
    .withColumn("total_after_tax", col("total")+col("tax"))
    .writeStream
        .option("checkpointLocation", checkpointPath)
        .outputMode("append")
         .______________ 
        .table("new_orders")
)

```
Fill in the blank to make the query executes a micro-batch to process data every 2 minutes

#### A25
```python
trigger(processingTime=”2 minutes")
```

## Q26
Which of the following is used by Auto Loader to load data incrementally?
#### A26
Spark Structured Streaming
- Auto Loader는 Spark Streaming source로 `cloudFiles`를 사용

## Q27
Which of the following statements best describes Auto Loader ?
#### A27
Auto loader monitors a source location, in which files accumulate, to identify and ingest only new arriving files with each command run. While the files that have already been ingested in previous runs are skipped.

## Q28
A data engineer has defined the following data quality constraint in a Delta Live Tables pipeline:

`CONSTRAINT valid_id EXPECT (id IS NOT NULL) _____________`

Fill in the above blank so records violating this constraint will be added to the target table, and reported in metrics
#### A28
There is no need to add ON VIOLATION clause. By default, records violating the constraint will be kept, and reported as invalid in the event log
- **ON VIOLATION DROP ROW**
- **ON VILOATION FAIL UPDATE** 
- [[Databricks Certified Data Engineer Associate Test 1#Drop invalid records]]
- [[Databricks Certified Data Engineer Associate Test 1#Fail on invalid records]]

## Q29
The data engineer team has a DLT pipeline that updates all the tables once and then stops. The compute resources of the pipeline continue running to allow for quick testing.

Which of the following best describes the execution modes of this DLT pipeline ?
#### A29
The DLT pipeline executes in Triggered Pipeline mode under Development mode.

#### DLT Pipeline - Triggered Pipeline
- 현재 사용 가능한 모든 데이터로 각 테이블 업데이트 및 종료
#### DLT Pipeline - Development Mode
- 클러스터 재사용을 통한 재시작 오버헤드 회피
- 개발모드 활성화시 클러스터 2시간 동안 수행
- 파이프라인 재시도를 비활성화 -> 오류 즉시 감지 및 수정 가능

## Q30
Which of the following will utilize Gold tables as their source?
#### A30
Dashboards


## Q31
Which of the following code blocks can a data engineer use to query the existing streaming table **events** ?
#### A31
```python
spark.readStream
.table("events")
```
- Spark Structured Streaming에서는 `readStream`을 통해 테이블 참조

## Q32
In multi-hop architecture, which of the following statements best describes the Bronze layer ?
#### A32
It maintains raw data ingested from various sources

## Q33
Given the following Structured Streaming query
```python
(spark.readStream
        .format("cloudFiles")
        .option("cloudFiles.format", "json")
        .load(ordersLocation)
     .writeStream
        .option("checkpointLocation", checkpointPath)
        .table("uncleanedOrders")
)
```
Which of the following best describe the purpose of this query in a multi-hop architecture?
#### A33
The query is performing raw data ingestion into a Bronze table
- cloudFiles로 보아 AUTO LOADER를 사용하는 기능
- ordersLocation이라는 Raw json 데이터 수집

## Q34
A data engineer has the following query in a Delta Live Tables pipeline:
```sql
CREATE LIVE TABLE aggregated_sales
AS
  SELECT store_id, sum(total)
  FROM cleaned_sales
  GROUP BY store_id
```
The pipeline is failing to start due to an error in this query

Which of the following changes should be made to this query to successfully start the DLT pipeline ?
#### A34
```sql
CREATE LIVE TABLE aggregated_sales
AS
  SELECT store_id, sum(total)
  FROM LIVE.cleaned_sales
  GROUP BY store_id
```
- `CREATE LIVE TABLE` 키워드를 사용하고
	- 다른 Live table을 참조하기 위해 `LIVE.`을 사용해야 함

## Q35
A data engineer has defined the following data quality constraint in a Delta Live Tables pipeline:

`CONSTRAINT valid_id EXPECT (id IS NOT NULL) _____________`

Fill in the above blank so records violating this constraint will be dropped, and reported in metrics
#### A35
```sql
ON VIOLATION DROP ROW
```

## Q36
Which of the following compute resources is available in Databricks SQL ?
#### A36

#### SQL warehouses
- Compute resource: 클라우드에서 처리기능을 제공하는 인프라 리소스
- SQL warehouse: Databricks SQL내에서 데이터 개체에 대해 SQL 명령을 수행하는 Compute resource
- SQL endpoint의 다른 이름!

## Q37
Which of the following is the benefit of using the Auto Stop feature of Databricks SQL warehouses ?
#### A37
Minimizes the total running time of the warehouse
- Auto Stop: 지정된 시간동안 idle할 경우 warehouse를 중지하는 기능

## Q38
Which of the following alert destinations is **Not** supported in Databricks SQL ?
#### A38
SMS

#### Databricks SQL destinations
- email, webhook, slack, Microsoft teams

## Q39
A data engineering team has a long-running multi-tasks Job. The team members need to be notified when the run of this job completes.

Which of the following approaches can be used to send emails to the team members when the job completes ?
#### A39
They can configure email notifications settings in the job page
- Job 시작, 성공, 실패시 Email notification In Job page
- 여러 이메일 주소 입력 가능
- Job Owner가 아니어도 설정 가능

## Q40
A data engineer wants to increase the cluster size of an existing Databricks SQL warehouse.

Which of the following is the benefit of increasing the cluster size of Databricks SQL warehouses ?
#### A40
Improves the latency of the queries execution
- Cluster Size의 증가
	- cluster worker 수의 증가
	- compute 리소스 향상(쿼리 및 대시보드에서 활용)
	- query latency를 줄임

## Q41
Which of the following describes Cron syntax in Databricks Jobs ?
#### A41
It’s an expression to represent complex job schedule that can be defined programmatically
- Job Schedule을 Cron으로 구성할 수 있음

## Q42
The data engineer team has a DLT pipeline that updates all the tables at defined intervals until manually stopped. The compute resources terminate when the pipeline is stopped.

Which of the following best describes the execution modes of this DLT pipeline ?
#### A42
The DLT pipeline executes in Continuous Pipeline mode under Production mode.


#### DLT Pipeline - Continuous Pipeline
- 입력 데이터가 변경되면 테이블 지속 업데이트
- 업데이트가 시작되면 파이프라인 종료시까지 진행

#### DLT Pipeline - Production Mode
- pipeline이 중지되면 클러스터 즉시 종료
- 복구 가능한 오류가 있는 경우 클러스터 다시 시작
	- memory leak, leak or stale credentials
- 특정 오류가 발생한 경우 retry
	- e.g. failure to start a cluster

## Q43
Which part of the Databricks Platform can a data engineer use to grant permissions on tables to users ?
#### A43
Data Explorer
- [[Databricks Certified Data Engineer Associate Test 2#Data explorer]]

## Q44
Which of the following commands can a data engineer use to grant full permissions to the HR team on the table **employees** ?
#### A44
```sql
GRANT ALL PRIVILEGES ON TABLE employees TO hr_team
```

#### ALL_PRIVILEGES
- SELECT, CREATE, MODIFY, USAGE, READ_METADATA

## Q45
A data engineer uses the following SQL query:

`GRANT MODIFY ON TABLE employees TO hr_team`

Which of the following describes the ability given by the `MODIFY` privilege ?
#### A45
All the above abilities are given by the `MODIFY` privilege
- MODIFY는 add, delete, modify 권한을 모두 부여