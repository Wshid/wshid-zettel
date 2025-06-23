---
date: "2025-05-30"
datetime: "2025-05-30 21:48:56"
book: 
page: 
tags: 
references: 
aliases:
---
## Q1
One of the foundational technologies provided by the Databricks Lakehouse Platform is an open-source, file-based storage format that brings reliability to data lakes.

Which of the following technologies is being described in the above statement?
#### A1
Delta Lake
- Delta Lake is an open source technology that extends Parquet data files with a file-based transaction log for ACID transactions that brings reliability to data lakes.

## Q2
Which of the following commands can a data engineer use to purge stale data files of a Delta table?
#### A2
VACUUM
- deletes the unused data files
- older than a specificed data retention period

## Q3
In Databricks Repos (Git folders), which of the following operations a data engineer can use to save local changes of a repo to its remote repository?
#### A3
Commit & Push

## Q4
In Delta Lake tables, which of the following is the primary format for the transaction log files?
#### A4  
JSON
- trasnaction logs: JSON
- data files: Parquet

## Q5
Which of the following functionalities can be performed in Databricks Repos (Git folders)?
#### A5
Pull from a remote Git repository
- Databricks Repo에서는 git pull 작업을 지원함

## Q6
Which of the following locations completely hosts the customer data ?
#### A6
Customer's cloud account
- Data를 hosting하는 storage 계정은 databricks customers' cloud account에 저장

## Q7
If ​​the default notebook language is Python, which of the following options a data engineer can use to run SQL commands in this Python Notebook ?
#### A7
They can add `%sql` at the start of a cell.
- magic command로 다음과 같은 언어 지원
	- `%python`
	- `%sql`
	- `%scala`
	- `%r`

## Q8
A junior data engineer uses the built-in Databricks Notebooks versioning for source control. A senior data engineer recommended using Databricks Repos (Git folders) instead.

Which of the following could explain why Databricks Repos is recommended instead of Databricks Notebooks versioning?
#### A8
Databricks Repos supports creating and managing branches for development work.
- Databricks Repo: Databricks Git folders
	- https://docs.databricks.com/aws/en/repos

## Q9
Which of the following services provides a data warehousing experience to its users?
#### A9
Databricks SQL
- Databricks SQL은 Databricks Lakehouse Platform이라는 [[data_warehouse]]
- SQL 및 BI app 수행 가능

## Q10
A data engineer noticed that there are unused data files in the directory of a Delta table. They executed the VACUUM command on this table; however, only some of those unused data files have been deleted.

Which of the following could explain why only some of the unused data files have been deleted after running the VACUUM command ?
#### A10
The deleted data files were older than the default retention threshold. While the remaining files are newer than the default retention threshold and can not be deleted.
- Vaccum을 수행하더라도 threshold보다 최신 파일이라면 제거되지 않음

## Q11
The data engineering team has a Delta table called **products** that contains products’ details including the net price.

Which of the following code blocks will apply a 50% discount on all the products where the price is greater than 1000 and save the new price to the table?
#### A11
```sql
UPDATE products SET price = price * 0.5 WHERE price > 1000;
```

## Q12
A data engineer wants to create a relational object by pulling data from two tables. The relational object will only be used in the current session. In order to save on storage costs, the date engineer wants to avoid copying and storing physical data.

Which of the following relational objects should the data engineer create?
#### A12
Temporary view
- 물리적 복제를 피하려면 view를 사용해야 하며
- 현재 세션에서만 활용하기 때문에 temporary view

## Q13
A data engineer has a database named **db_hr**, and they want to know where this database was created in the underlying storage.

Which of the following commands can the data engineer use to complete this task?
#### A13
```sql
DESCRIBE DATABASE db_hr
```
- `DESCRIBE DATABASE [EXTENDED] database_name` 문법
	- 기본적으로 db명, Comment, file system 정보 반환
	- EXTENDED를 사용하면 `properties`도 같이 반환
- c.f. `DESCRIBE`만 사용하면 `DESCRIBE TABLE`을 의미함

## Q14
Which of the following commands a data engineer can use to register the table **orders** from an existing SQLite database ?
#### A14
```sql
CREATE TABLE orders
  USING org.apache.spark.sql.jdbc
  OPTIONS (
    url "jdbc:sqlite:/bookstore.db",
    dbtable "orders"
  )
```
- `USING`을 통해 `sql.jdbc` library를 로드하고
- `OPTIONS`내부에서 상세 설정을 지정함

## Q15
When dropping a Delta table, which of the following explains why both the table's metadata and the data files will be deleted ?
#### A15
The table is managed

## Q16
Given the following commands:
```sql
CREATE DATABASE db_hr;
 
USE db_hr;
CREATE TABLE employees;
```
In which of the following locations will the employees table be located?
#### A16
```sql
dbfs:/user/hive/warehouse/db_hr.db
```
- 그래도 기본 디렉터리에서 `db_hr.db`와 같이 이름 붙은 DB가 만들어짐

## Q17
Which of the following code blocks can a data engineer use to create a Python function to multiply two integers and return the result?
#### A17
```python
def multiply_numbers(num1, num2):
    return num1 * num2
```

## Q18
Given the following 2 tables:
Fill in the blank to make the following query returns the below result:
```sql
SELECT students.name, students.age, enrollments.course_id
FROM students
_____________ enrollments
ON students.student_id = enrollments.student_id
```
#### A18
LEFT JOIN
- left에 매치되는 모든 데이터 노출
- left의 모든 값과 오른쪽에 일치된 값 반환
	- 일치되지 않을 경우 NULL(OUTER)

## Q19
Which of the following SQL keywords can be used to rotate rows of a table by turning row values into multiple columns ?
#### A19
`PIVOT`
- 지정된 열 목록의 값들을 column으로 회전시켜 행을 반환함
- 테이블을 긴 형식에서 넓은 형식으로 변화(long format -> wide format)
- https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-qry-select-pivot

## Q20
Fill in the below blank to get the number of courses incremented by 1 for each student in array column **students**.
```sql
SELECT
  faculty_id,
  students,
  ___________ AS new_totals
FROM faculties
```
#### A20
```sql
TRANSFORM (students, i -> i.total_courses + 1)
```
- transform 함수는 array -> array

## Q21
Fill in the below blank to successfully create a table using data from CSV files located at **/path/input**
```sql
CREATE TABLE my_table
(col1 STRING, col2 STRING)
____________
OPTIONS (header = "true",
        delimiter = ";")
LOCATION = "/path/input"
```
#### A21
```bash
USING CSV
```
- https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-syntax-ddl-create-table-using
- USING `datasource`

## Q22
Which of the following statements best describes the usage of `CREATE SCHEMA` command ?
#### A22
It’s used to create a database
- `CREATE SCHEMA`는 `CREATE DATABASE`와 동치어

## Q23
Which of the following statements is **Not** true about CTAS statements ?
#### A23
CTAS statements support manual schema declaration
- CTAS는 infer schema로 처리함. 별도 스키마를 지정할 수 없음


## Q24
Which of the following SQL commands will append this new row to the existing Delta table **users**?
#### A24
```sql
INSERT INTO users VALUES (“0015”, “Adam”, 23)
```

## Q25
Given the following Structured Streaming query:
```sql
(spark.table("orders")
        .withColumn("total_after_tax", col("total")+col("tax"))
    .writeStream
        .option("checkpointLocation", checkpointPath)
        .outputMode("append")
        .___________
        .table("new_orders") )
```
Fill in the blank to make the query executes multiple micro-batches to process all available data, then stops the trigger.
#### A25
```python
trigger(availableNow=True)
```
- 사용 가능한 모든 데이터를 여러개의 마이크로 배치로 처리하는 방법
- 사용가능한 데이터 처리가 완료되면 자체적으로 종료

## Q26
Which of the following techniques allows Auto Loader to track the ingestion progress and store metadata of the discovered files ?
#### A26
Checkpointing
- keeps track of discovered files using checkpointing
- AUTO LOADER가 exactly-once를 보증함

## Q27
A data engineer has defined the following data quality constraint in a Delta Live Tables pipeline:
```sql
CONSTRAINT valid_id EXPECT (id IS NOT NULL) _____________

```
#### A27
```
ON VIOLATION FAIL UPDATE
```
- [[Databricks Certified Data Engineer Associate Test 1#Fail on invalid records]]

## Q28
In multi-hop architecture, which of the following statements best describes the Silver layer tables?
#### A28
They provide a more refined view of raw data, where it’s filtered, cleaned, and enriched.

## Q29
The data engineer team has a DLT pipeline that updates all the tables at defined intervals until manually stopped. The compute resources of the pipeline continue running to allow for quick testing.

Which of the following best describes the execution modes of this DLT pipeline ?
#### A29
The DLT pipeline executes in Continuous Pipeline mode under Development mode.
- Continous Pipeline: 주기적으로 데이터를 업데이트 함
- [[Databricks Certified Data Engineer Associate - Practice Exam 1#DLT Pipeline - Continuous Pipeline]]
- [[Databricks Certified Data Engineer Associate - Practice Exam 1#DLT Pipeline - Triggered Pipeline]]

## Q30
Given the following Structured Streaming query:
```python
(spark.readStream
        .table("cleanedOrders")
        .groupBy("productCategory")
        .agg(sum("totalWithTax"))
    .writeStream
        .option("checkpointLocation", checkpointPath)
        .outputMode("complete")
        .table("aggregatedOrders")
)
```
Which of the following best describe the purpose of this query in a multi-hop architecture?
#### A30
The query is performing a hop from Silver layer to a Gold table

## Q31
Given the following Structured Streaming query:
```python
(spark.readStream
        .table("orders")
    .writeStream
        .option("checkpointLocation", checkpointPath)
        .table("Output_Table")
)
```
Which of the following is the trigger Interval for this query ?
#### A31
Every half second
- `trigger(processingTime=”500ms")`
- trigger 기본값은 0.5s로 보임

## Q32
A data engineer has the following query in a Delta Live Tables pipeline
```sql
CREATE STREAMING TABLE sales_silver
AS
  SELECT store_id, total + tax AS total_after_tax
  FROM LIVE.sales_bronze
```
The pipeline is failing to start due to an error in this query.

Which of the following changes should be made to this query to successfully start the DLT pipeline ?
#### A32
```sql
CREATE STREAMING TABLE sales_silver
AS
  SELECT store_id, total + tax AS total_after_tax
  FROM STREAM(LIVE.sales_bronze)
```

#### CREATE STREAMING TABLE
- `STREAM()` 함수를 사용하면 동일 파이프라인의 다른 테이블의 데이터 스트리밍 가능
- 이때, `CREATE STREAMING TABLE` 구문을 사용해야 함
- [[Databricks Certified Data Engineer Associate - Practice Exam 1#CREATE LIVE TABLE]]
- LIVE 테이블의 경우 무조건 `LIVE.`를 붙여주어야 함
- 또한 `CREATE STREAMING LIVE TABLE`은 곧 deprecated 예정이라고 함
- LIVE TABLE의 경우 `CREATE MATERIALIZED VIEW`로 변경되었다는 이야기가 있음
	- stateless 연산을 수행하며, 필요시 전체 테이블을 재계산함

## Q33
In multi-hop architecture, which of the following statements best describes the Gold layer tables?
#### A33
- They provide business-level aggregations that power analytics, machine learning, and production applications

## Q34
The data engineer team has a DLT pipeline that updates all the tables once and then stops. The compute resources of the pipeline terminate when the pipeline is stopped.

Which of the following best describes the execution modes of this DLT pipeline ?
#### A34
The DLT pipeline executes in Triggered Pipeline mode under Production mode.
- [[Databricks Certified Data Engineer Associate - Practice Exam 1#DLT Pipeline - Production Mode]]
- 파이프라인 종료시 모든 리소스가 반환됨(클러스터 종료)

## Q35
A data engineer needs to determine whether to use Auto Loader or COPY INTO command in order to load input data files incrementally.
In which of the following scenarios should the data engineer use Auto Loader over COPY INTO command ?
#### A35
If they are going to ingest files in the order of millions or more over time
- 수천개 단위로 파일 수집: COPY INTO 사용
- 수백만개 이상의 파일 수집: AUTO LOADER 사용
- schema primitive 및 evolution 관점에서도 AUTO LOADER가 우위

## Q36
From which of the following locations can a data engineer set a schedule to automatically refresh a Databricks SQL query ?
#### A36
From the query's page in Databricks SQL
- Query 페이지에서 Refresh Schedule을 설정할 수 있음

## Q37
Databricks provides a declarative ETL framework for building reliable and maintainable data processing pipelines, while maintaining table dependencies and data quality.

Which of the following technologies is being described above?
#### A37
Delta Live Tables
- DLT는 reliable, maintainable, testable한 pipeline
- 사용자가 Transformations를 정의하면 task orchestration, cluster management, monitoring, data quality, error handling을 지원함

## Q38
Which of the following services can a data engineer use for orchestration purposes in Databricks platform ?
#### A38
Databricks Jobs
- Databricks에서 Orchestration 목적으로 사용하려면 Job을 사용해야 함

## Q39
A data engineer has a Job with multiple tasks that takes more than 2 hours to complete. In the last run, the final task unexpectedly failed.

Which of the following actions can the data engineer perform to complete this Job Run while minimizing the execution time ?
#### A39
They can repair this Job Run so only the failed tasks will be re-executed
- task와 그 하위 작업만 재수행하면 됨
- 전체 task를 재수행 할 필요는 없음

## Q40
A data engineering team has a multi-tasks Job in production. The team members need to be notified in the case of job failure.

Which of the following approaches can be used to send emails to the team members in the case of job failure ?
#### A40
They can configure email notifications settings in the job page