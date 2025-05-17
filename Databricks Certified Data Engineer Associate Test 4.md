---
date: "2025-05-16"
datetime: "2025-05-16 20:21:45"
book: 
page: 
tags: 
references: 
aliases:
---
## Q16
You are asked to setup two tasks in a databricks job, the first task runs a notebook to download the data from a remote system, and the second task is a DLT pipeline that can process this data, how do you plan to configure this in Jobs UI
#### A16
- Single job can be used to setup both notebook and DLT pipeline, use two different tasks with linear dependency.
- Single Job을 사용해 notebook, DLT pipline 모두 설정 가능
	- 선형 종속성을 가진 두가지 작업 사용 가능
	- notebook task -> DLT task

## Q17
You are asked to set up an alert to notify in an email every time a KPI indicater increases beyond a threshold value, team also asked you to include the actual value in the alert email notification.
#### A17
- Setup an alert but use the custom template to notify the message in email’s subject
- Custom template을 사용하여 변수 사용이 가능함

## Q18
Operations team is using a centralized data quality monitoring system, a user can publish data quality metrics through a webhook, you were asked to develop a process to send messages using a webhook if there is atleast one duplicate record, which of the following approaches can be taken to integrate an alert with current data quality monitoring system
#### A18
- Setup an alert with custom Webhook destination
- Alert Destination에 webhook이 존재함

## Q19
You are currently working with the application team to setup a SQL Endpoint point, once the team started consuming the SQL Endpoint you noticed that during peak hours as the number of concurrent users increases you are seeing degradation in the query performance and the same queries are taking longer to run, which of the following steps can be taken to resolve the issue?
#### A19
- They can increase the maximum bound of the SQL endpoint’s scaling range.

## Q20
The data engineering team is using a bunch of SQL queries to review data quality and monitor the ETL job every day, which of the following approaches can be used to set up a schedule and automate this process?
#### A20
- They can schedule the query to refresh every 1 day from the query’s page in Databricks SQL.
- query info 탭에서 refresh schedule 지정 가능


## Q21
In order to use Unity catalog features, which of the following steps needs to be taken on managed/external tables in the Databricks workspace?
#### A21
-   Migrate/upgrade objects in workspace managed/external tables/view to unity catalog

## Q22
- What is the top-level object in unity catalog?
#### A22
- Metastore
- Meta Store -> Catalog -> Schema -> External/Managed Table/Views

## Q23
One of the team members Steve who has the ability to create views, created a new view called regional_sales_vw on the existing table called sales which is owned by John, and the second team member Kevin who works with regional sales managers wanted to query the data in regional_sales_vw, so Steve granted the permission to Kevin using command

`GRANT VIEW, USAGE ON regional_sales_vw to kevin@company.com` but Kevin is still unable to access the view?
#### A23
- Steve is not the owner of the sales table
- 소유권은 파생 객체에 대한 권한을 다른 사용자에게 부여할 수 있는지 결정함
- sales 테이블에 대한 소유권이 Steve에게 없었으므로
	- Steve는 테이블이나 데이터에 대한 권한을 간접적으로 부여할 수 없음

## Q24
You were asked to setup a new all-purpose cluster, but the cluster is unable to start which of the following steps do you need to take to identify the root cause of the issue and the reason why the cluster was unable to start?
#### A24
- Check the cluster event logs

#### Cluster event log
- 클러스터 가용성 문제 파악시 도움
- 리소스 제한, 클라우드 제공 업체등의 문제로 클러스터가 시작되지 않을 수 있음
- VM이 subnet 한계에 도달하거나, cpu quota가 넘치는 경우 등

## Q25
When you drop a managed table using SQL syntax `DROP TABLE table_name` how does it impact metadata, history, and data stored in the table?

#### A25
Drops table from meta store, drops metadata, history, and data in storage.

## Q26
Which of the following developer operations in CI/CD flow can be implemented in Databricks Repos?
#### A26
Commit and push code

## Q27
You noticed that a team member started using an all-purpose cluster to develop a notebook and used the same all-purpose cluster to set up a job that can run every 30 mins so they can update underlying tables which are used in a dashboard. What would you recommend for reducing the overall cost of this approach?
#### A27
Change the cluster all-purpose to job cluster when scheduling the job

#### All-purpose Cluster와 비용
- 개발중에 사용은 가능함
- 노트북과 상호 작용이 필요 없을 경우, 특히 예약되는 작업일 경우
	- Job Cluster를 사용하는 것이 더 저렴함
- 다목적 클러스터 사용시 비용은 2배가 될 수 있음

## Q28
Which of the following commands can be used to run one notebook from another notebook?
#### A28
`dbutils.notebook.run("full notebook path")`

```python
run(path: String, timeout_seconds: int, arguments: Map): String

# example
dbutils.notebook.run("ful-notebook-name", 60, {"argument": "data", "argument2": "data2", ...})
```

## Q29
Which of the following SQL command can be used to insert or update or delete rows based on a condition to check if a row(s) exists?
#### A29
- `MERGE INTO table_name`

```sql
MERGE INTO target_table_name [target_alias]
   USING source_table_reference [source_alias]
   ON merge_condition
   [ WHEN MATCHED [ AND condition ] THEN matched_action ] [...]
   [ WHEN NOT MATCHED [ AND condition ]  THEN not_matched_action ] [...]
 
matched_action
 { DELETE |
   UPDATE SET * |
   UPDATE SET { column1 = value1 } [, ...] }
 
not_matched_action
 { INSERT * |
   INSERT (column1 [, ...] ) VALUES (value1 [, ...])
```

## Q30
When investigating a data issue you realized that a process accidentally updated the table,  you want to query the same table with yesterday's version of the data so you can review what the prior version looks like, what is the best way to query historical data so you can do your analysis?
#### A30
```sql
SELECT * FROM table_name TIMESTAMP AS OF date_sub(current_date(), 1)
```

#### Time Travel Query
```sql
-- TIMESTAMP
SELECT count(*) FROM my_table TIMESTAMP AS OF "2019-01-01"
SELECT count(*) FROM my_table TIMESTAMP AS OF date_sub(current_date(), 1)
SELECT count(*) FROM my_table TIMESTAMP AS OF "2019-01-01 01:30:00.000"

-- Version Number
SELECT count(*) FROM my_table VERSION AS OF 5238
SELECT count(*) FROM my_table@v5238
SELECT count(*) FROM delta.`/path/to/my/table@v5238`
```

#### Time Travel Expression
```sql
-- Time travel
time_travel_version
 { TIMESTAMP AS OF timestamp_expression |
   VERSION AS OF version }

-- timestamp_expression
- `'2018-10-18T22:15:12.013Z'`, that is, a string that can be cast to a timestamp
- `cast('2018-10-18 13:36:32 CEST' as timestamp)`
- `'2018-10-18'`, that is, a date string
- `current_timestamp() - interval 12 hours`
- `date_sub(current_date(), 1)`
```

## Q31
While investigating a data issue, you wanted to review yesterday's version of the table using below command, while querying the previous version of the table using time travel you realized that you are no longer able to view the historical data in the table and you could see it the table was updated yesterday based on the table history(DESCRIBE HISTORY table_name) command what could be the reason why you can not access this data?
```sql
SELECT * FROM table_name TIMESTAMP AS OF date_sub(current_date(), 1)
```
#### A31
A command `VACUUM table_name RETAIN 0` was ran on the table

#### VACUUM
- default: 7 days
- recursive하게 transaction log를 제거함
- `VACUUM table_name RETAIN 0`를 하게 되면 모든 historical versions of data가 제거되며
	- 현재 버전만 남게 됨

## Q32
You have accidentally deleted records from a table called transactions, what is the easiest way to restore the records deleted or the previous state of the table? Prior to deleting the version of the table is 3 and after delete the version of the table is 4.
#### A32
```sql
RESTORE TABLE transactions TO VERSION as of 3

# 문법
RESTORE [TABLE] table_name [TO] time_travel_version
```

## Q33
Create a schema called bronze using location ‘/mnt/delta/bronze’, and check if the schema exists before creating.
#### A33
```sql
CREATE SCHEMA IF NOT EXISTS bronze LOCATION '/mnt/delta/bronze'
```

## Q34
How do you check the location of an existing schema in Delta Lake? 
#### A34
- Run SQL command `DESCRIBE SCHEMA EXTENDED schema_name`

## Q35
Which of the below SQL commands create a Global temporary view?
#### A35
```sql
 CREATE OR REPLACE GLOBAL TEMPORARY VIEW view_name
    AS SELECT * FROM table_name
```

## Q36
The below spark command is looking to create a summary table based customerId and the number of times the customerId is present in the event_log delta table and write a one-time micro-batch to a summary table, fill in the blanks to complete the query.
```python
spark._________
  .format("delta")
  .table("events_log")
  .groupBy("customerId")
  .count()
  ._______
  .format("delta")
  .outputMode("complete")
  .option("checkpointLocation", "/tmp/delta/eventsByCustomer/_checkpoints/")
  .trigger(______)
  .table("target_table")
```
#### A36
- readStream, writeStream, once = True
- 한번 수행할때는 processingTime이 아닌, `once=True` 사용

## Q37
The team has decided to take advantage of table properties to identify a business owner for each table, which of the following table DDL syntax allows you to populate a table property identifying the business owner of a table
#### A37
```sql
CREATE TABLE inventory (id INT, units FLOAT)
TBLPROPERTIES (business_owner = 'supply chain')
```
#### ALTER TABLE TBLPROPERTIES
```sql
ALTER TABLE inventory SET TBLPROPERTIES(business_owner , 'operations')
```
- `ALTER`일 경우에는 `SET`을 사용해야 함

## Q38
Data science team has requested they are missing a column in the table called average price, this can be calculated using units sold and sales amt, which of the following SQL statements allow you to reload the data with additional column
#### A38
```sql
CREATE OR REPLACE TABLE sales
AS SELECT *, salesAmt/unitsSold as avgPrice FROM sales
```

#### INSERT OVERWRITE와 CRAS의 차이
- `INSERT OVERWRITE`와 `CRAS`의 차이는 **스키마 변경 유무**
- 만약 `INSERT OVERWRITE`에서 스키마 변경시에도 적용하고 싶다면
	- `spark.databricks.delta.schema.autoMerge.enabled` 옵션을 켜야함
## Q39
You are working on a process to load external CSV files into a delta table by leveraging the COPY INTO command, but after running the command for the second time no data was loaded into the table name, why is that?
```sql
COPY INTO table_name
FROM 'dbfs:/mnt/raw/*.csv'
FILEFORMAT = CSV
```
#### A39
- COPY INTO did not detect new files after the last load

#### COPY_INTO의 동작 방식
- `COPY INTO`는 테이블에 성공적으로 로드된 파일을 추적. 추후 COPY INTO 수행시 파일을 건너뜀
- `COPY_OPTIONS 'force=true'`옵셜 설정시, 모든 경로/패턴의 파일이 로드 됨


## Q40
What is the main difference between the below two commands?
```sql
INSERT OVERWRITE table_name
SELECT * FROM table

CREATE OR REPLACE TABLE table_name
AS SELECT * FROM table
```
#### A40
- `INSERT OVERWRITE` replaces data by default, `CREATE OR REPLACE` replaces data and Schema by default

## Q41
- Which of the following functions can be used to convert JSON string to Struct data type?
#### A41
```sql
FROM_JSON (json value, schema of json)
```

#### FROM_JSON
```sql
from_json(jsonStr, schema [, options])

-- example
SELECT from_json('{"a":1, "b":0.8}', 'a INT, b DOUBLE');
SELECT from_json('{"datetime":"26/08/2015"}', 'datetime Timestamp', map('timestampFormat', 'dd/MM/yyyy')); {"datetime":2015-08-26 00:00:00}
```
- `jsonStr`: A STRING expression specifying a row of CSV data.
- `schema`: A STRING literal or invocation of [schema_of_json function (Databricks SQL)](https://docs.microsoft.com/en-us/azure/databricks/sql/language-manual/functions/schema_of_json).
- `options`: An optional MAP<STRING,STRING> literal specifying directives.

## Q42
You are working on a marketing team request to identify customers with the same information between two tables CUSTOMERS_2021 and CUSTOMERS_2020 each table contains 25 columns with the same schema, You are looking to identify rows that match between two tables across all columns, which of the following can be used to perform in SQL
#### Q42
```sql
SELECT * FROM CUSTOMERS_2021 
 INTERSECT
SELECT * FROM CUSTOMERS_2020
```

#### INTERSECT
- `ALL`
- `DISDINCT`: 중복 제거(default)

## Q43
You are looking to process the data based on two variables, one to check if the department is supply chain and second to check if process flag is set to True
#### A43
```python
if department == "supply chain" and process:
```

## Q44
You were asked to create a notebook that can take department as a parameter and process the data accordingly, which is the following statements result in storing the notebook parameter into a python variable
#### A44
```python
department = dbutils.widget.get("department")
```
- databricks widget: https://docs.databricks.com/aws/en/notebooks/widgets

## Q45
Which of the following statements can successfully read the notebook widget and pass the python variable to a SQL statement in a Python notebook cell?
```python
order_date  = dbutils.widgets.get("widget_order_date")
spark.sql(f"SELECT * FROM sales WHERE orderDate = '{order_date}' ") 
```
