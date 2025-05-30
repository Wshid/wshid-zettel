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
