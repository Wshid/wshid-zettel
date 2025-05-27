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