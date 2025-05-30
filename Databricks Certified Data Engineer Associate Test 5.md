---
date: "2025-05-19"
datetime: "2025-05-19 20:14:46"
book: 
page: 
tags: 
references: 
aliases:
---
## Q4
You were asked to write python code to stop all running streams, which of the following command can be used to get a list of all active streams currently running so we can stop them, fill in the blank.
```python
for s in _______________:
  s.stop()
```
#### A4
```python
spark.streams.active
```

## Q5
At the end of the inventory process a file gets uploaded to the cloud object storage, you are asked to build a process to ingest data which of the following method can be used to ingest the data incrementally, schema of the file is expected to change overtime ingestion process should be able to handle these changes automatically. Below is the auto loader to command to load the data, fill in the blanks for successful execution of below code.
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
#### A5
- `cloudfiles.format, cloudfiles.schemalocation, checkpointlocation, mergeSchema`

## Q6
Which of the following scenarios is the best fit for AUTO LOADER?
#### A6
Efficiently process new data incrementally from cloud object storage

## Q7
You are asked to setup an AUTO LOADER to process the incoming data, this data arrives in JSON format and get dropped into cloud object storage and you are required to process the data as soon as it arrives in cloud storage, which of the following statements is correct
#### A7
AUTO LOADER can support file notification method so it can process data as it arrives
- File Notification의 후속 처리로 external storage의 내용을 정리하면 됨

## Q8
What is the main difference between the bronze layer and silver layer in a medallion architecture?
#### A8
Bronze is raw copy of ingested data, silver contains data with production schema and optimized for ELT/ETL throughput

## Q9
What is the main difference between the silver layer and the gold layer in medalion architecture?
#### A9
Gold may contain aggregated data

## Q10
What is the main difference between the silver layer and gold layer in medallion architecture?

#### A10
Silver optimized to perform ETL, Gold is optimized query performance
- Gold에서는 query 성능 튜닝
- Silver에서는 ETL 성능 튜닝
- [[Databricks Certified Data Engineer Associate Test 1#Medallian Architecture]]

## Q11
A dataset has been defined using Delta Live Tables and includes an expectations clause: `CONSTRAINT valid_timestamp EXPECT (timestamp > '2020-01-01')`

What is the expected behavior when a batch of data containing data that violates these constraints is processed?
#### A11
Records that violate the expectation are added to the target dataset and recorded as invalid in the event log.
- target dataset에 추가되면서 event log에 추가됨
- [[Databricks Certified Data Engineer Associate Test 1#Retain invalid records]]

## Q12
A dataset has been defined using Delta Live Tables and includes an expectations clause: `CONSTRAINT valid_timestamp EXPECT (timestamp > '2020-01-01') ON VIOLATION DROP ROW`

What is the expected behavior when a batch of data containing data that violates these constraints is processed?
#### A12
Records that violate the expectation are dropped from the target dataset and recorded as invalid in the event log.
- Drop은 하되, event log에는 기록이 됨

## Q13
What is the output of below function when executed with input parameters 1, 3 :
```python
def check_input(x,y):
    if x < y:
        x= x+1
        if x>y:
            x= x+1
            if x <y:
            x = x+1
    return x
```
#### A13
2

## Q14
Your colleague was walking you through how a job was setup, but you noticed a warning message that said, “Jobs running on all-purpose cluster are considered all purpose compute", the colleague was not sure why he was getting the warning message, how do you best explain this warning message?
#### A14
All-purpose clusters are more expensive than the job clusters
- All-purpose는 job cluster보다 비싸다

## Q15
Your team has hundreds of jobs running but it is difficult to track cost of each job run, you are asked to provide a recommendation on how to monitor and track cost across various workloads
#### A15
Use Tags, during job creation so cost can be easily tracked
- job 관련 내용은 tag 태깅으로 구분은 가능함

## Q16
The sales team has asked the Data engineering team to develop a dashboard that shows sales performance for all stores, but the sales team would like to use the dashboard but would like to select individual store location, which of the following approaches Data Engineering team can use to build this functionality into the dashboard.
#### A16
Use query Parameters which then allow user to choose any location
```sql
-- Multi
SELECT *  FROM sales WHERE field IN ( {{ Multi Select Parameter }} )

-- Single
SELECT *  FROM sales WHERE field =  {{ Single Select Parameter }}
```
## Q17
You are working on a dashboard that takes a long time to load in the browser, due to the fact that each visualization contains a lot of data to populate, which of the following approaches can be taken to address this issue?
#### A17
Use Databricks SQL Query filter to limit the amount of data in each visualization
- query filter의 사용
```sql
SELECT action AS `action::filter`, COUNT(0) AS "actions count"
FROM events
GROUP BY action
```
- 이미 쿼리는 수행 한뒤, 브라우저에서 보여줄 때 필터링 하는 조건
- 소규모 데이터셋에 적합함

## Q18
One of the queries in the Databricks SQL Dashboard takes a long time to refresh, which of the below steps can be taken to identify the root cause of this issue?
#### A18
Use Query History, to view queries and select query, and check query profile to time spent in each step


## Q19
A SQL Dashboard was built for the supply chain team to monitor the inventory and product orders, but all of the timestamps displayed on the dashboards are showing in UTC format, so they requested to change the time zone to the location of New York. How would you approach resolving this issue?
#### A19
Under SQL Admin Console, set the SQL configuration parameter time zone to America/New_York
- 어드민 콘솔에서 Timezone 변경이 가능함

## Q20
Which of the following technique can be used to implement fine-grained access control to rows and columns of the Delta table based on the user's access?
#### A20
Use dynamic view functions
```sql
-- 관리자 그룹 속해있는지 여부에 따라 행에 대한 액세스 제한하기

CREATE VIEW sales_redacted AS 
SELECT user_id, country, product, total 
FROM sales_raw 
WHERE CASE WHEN is_member('managers') THEN TRUE ELSE total <= 1000000 END;
```

```sql
-- 사용자 access에 따라 열 데이터를 숨기는 쿼리
CREATE VIEW sales_redacted AS 
SELECT user_id,
       CASE WHEN is_member('auditors') THEN email ELSE 'REDACTED' END AS email,
       country, 
       product, 
       total 
FROM sales_raw
```

#### Dynamic View Functions
- http://learn.microsoft.com/en-us/azure/databricks/data-governance/table-acls/object-privileges#dynamic-view-functions
- `is_member`
- `is_group_member`

## Q21
Unity catalog helps you manage the below resources in Databricks at account level
#### A21
All of the above
- Unity Catalog는 Governance Solution
- file, table, ML model, dashboard 등에 대한 관리 가능

## Q22
John Smith is a newly joined team member in the Marketing team who currently has access read access to sales tables but does not have access to delete rows from the table, which of the following commands help you accomplish this?
#### A22
```sql
GRANT MODIFY ON TABLE table_name TO john.smith@marketing.com
```
- `INSERT, UPDATE, DELETE`는 `MODIFY`로 통합 관리
- [[Databricks Certified Data Engineer Associate Test 2#Grant Privileges]]

## Q23
Kevin is the owner of both the sales table and regional_sales_vw view which uses the sales table as the underlying source for the data, and Kevin is looking to grant select privilege on the view regional_sales_vw to one of newly joined team members Steven. Which of the following is a true statement?
#### A23
Kevin can grant access to the view, because he is the owner of the view and the underlying table
- Ownership 자체는 파생 객체에 대한 권한을 다른 사용자에게 부여할 수 있는지 판단
- 스키마, 테이블, 뷰, 함수를 만드는 사용자가 소유자

## Q24
Identify one of the below statements that can query a delta table in PySpark Dataframe API
#### A24
```python
Spark.read.table("table_name")
```
- delta table을 읽을 때 단순하게 접근

## Q25
You are currently working on storing data you received from different customer surveys, this data is highly unstructured and changes over time,  why Lakehouse is a better choice compared to a Data warehouse?
#### A25
Lakehouse supports schema enforcement and evolution, traditional data warehouses lack schema evolution.

## Q26
Which of the following locations hosts the driver and worker nodes of a Databricks-managed cluster?
#### A26
- Data plane
- 컴퓨팅(all-purpose, job cluster, DLT) -> customer cloud account
- 한가지 예외 존재
	- SQL Warehouses compute는 classic, pro, serverless 3가지 존재
	- classic, pro compute: customer cloud account에 저장
	- serverless: databricks cloud account에 저장

## Q27
You have written a notebook to generate a summary data set for reporting, Notebook was scheduled using the job cluster, but you realized it takes an average of 8 minutes to start the cluster, what feature can be used to start the cluster in a timely fashion?
#### A27
Use the Databricks cluster pools feature to reduce the startup time
- [[Databricks Certified Data Engineer Associate Test 2#Cluster Pool의 사용]]

## Q28
Which of the following statement is true about Databricks repos?
#### A28
Databricks repos allow you to comment and commit code changes and push them to a remote branch

## Q29
Which of the statement is correct about the cluster pools?
#### A29
Cluster pools allow you to save time when starting a new cluster

## Q30
Once a cluster is deleted, below additional actions need to performed by the administrator
#### A30
No action needs to be performed. All resources are automatically removed.

## Q31
How does a Delta Lake differ from a traditional data lake?
#### A31
Delta lake is an open storage format like parquet with additional capabilities that can provide reliability, security, and performance

## Q32
How VACCUM and OPTIMIZE commands can be used to manage the DELTA lake?
#### A32
OPTIMIZE command can be used to compact small parquet files, and the VACCUM command can be used to delete parquet files that are marked for deletion/unused.
- `VACUUM`의 경우 `thresold`보다 오래된 파일의 제거 가능
	- 자동으로 수행되지는 않음
	- default: 7d

## Q33
Which of the below commands can be used to drop a DELTA table?
#### A33
```sql
DROP TABLE table_name
```

## Q34
Delete records from the transactions Delta table where transactionDate is greater than current timestamp?
#### A34
```sql
DELETE FROM transactions where transactionDate > current_timestamp()
```

## Q35
How does Lakehouse replace the dependency on using Data lakes and Data warehouses in a Data and Analytics solution?
#### A35
All the above
- Lakehouse = Data Lake + Data Warehouse
- [[Databricks Certified Data Engineer Associate Test 1#LakeHouse의 특징]]

## Q36
The default threshold of VACUUM is 7 days, internal audit team asked to certain tables to maintain at least 365 days as part of compliance requirement, which of the below setting is needed to implement.
#### A36
```sql
ALTER TABLE table_name set TBLPROPERTIES (delta.deletedFileRetentionDuration= ‘interval 365 days’)
```

## Q37
Which of the following commands can be used to query a delta table?
#### A37
Both A & B

```python
%python
spark.sql("select * from table_name")
```

```sql
%sql 
Select * from table_name 
```

## Q38
Below table **temp_data** has one column called **raw** contains JSON data that records temperature for every four hours in the day for the city of **Chicago**, you are asked to calculate the **maximum** temperature that was ever recorded for **12:00 PM** hour across all the days.  Parse the JSON data and use the necessary array function to calculate the max temp.

Table: temp_date
Column: raw
Datatype: string

#### A38
- https://docs.databricks.com/aws/en/semi-structured/json
```sql
select array_max(from_json(raw:chicago[*].temp[3],'array<int>')) from temp_data
```

- `raw:chicago[*]`: chicago에 대한 내용을 가져옴
- `raw:chicago[*].temp[3]`: chicago의 4번째 element를 문자열 형태로 가져옴
- `from_json(raw:chicago[*].temp[3], 'array<int>')`: str to array
- `array_max...`: array 중에 최대값 가져오기
- 문자열 컬럼은 `:`로 참조하기 시작하며, 이후 `.`으로 속성 조회

## Q39
Which of the following SQL statements can be used to update a transactions table, to set a flag on the table from Y to N
#### A39
```sql
UPDATE transactions SET active_flag = 'N' WHERE active_flag = 'Y'
```

## A40
Below sample input data contains two columns, one cartId also known as session id, and the second column is called items, every time a customer makes a change to the cart this is stored as an array in the table, the Marketing team asked you to create a unique list of item’s that were ever added to the cart by each customer, fill in blanks by choosing the appropriate array function so the query produces below **expected** result as shown below.

Schema: `cartId INT, items Array<INT>`

```sql
SELECT cartId, ___ (___(items)) as items
FROM carts GROUP BY cartId
```

```md
Expected result:

cartId              items

1                 [1,100,200,300,250]
```

#### A40
`ARRAY_UNION, COLLECT_SET`
- `COLLECT_SET`: 모든 row의 column value를 하나의 리스트로 만듦
- `ARRAY_UNION`: 중복 제거하여 합침

## Q41
You were asked to identify number of times a temperature sensor exceed threshold temperature (100.00) by each device, each row contains 5 readings collected every 5 minutes, fill in the blank with the appropriate functions.

`Schema: deviceId INT, deviceTemp ARRAY<double>, dateTimeCollected TIMESTAMP`

```sql
SELECT deviceId, __ (__ (__(deviceTemp], i -> i > 100.00)))
FROM devices
GROUP BY deviceId
```
#### A41
`SUM, SIZE, FILTER`
- `FILTER`: array의 기반의 데이터 필터링
- `SIZE`: array의 사이즈를 가져옴
- `SUM`: 합계 계산

## Q42
You are currently looking at a table that contains data from an e-commerce platform, each row contains a list of items(Item number) that were present in the cart, when the customer makes a change to the cart the entire information is saved as a separate list and appended to an existing list for the duration of the customer session, to identify all the items customer bought you have to make a unique list of items, you were asked to create a unique item’s list that was added to the cart by the user, **fill in the blanks** of below query by choosing the appropriate higher-order function?

Note: See below sample data and expected output.

`Schema: cartId INT, items Array<INT>`

**Fill in the blanks:**

`SELECT cartId, _(_(items)) FROM carts`

#### A42
`ARRAY_DISTINCT, FLATTEN`
- `FLATTEN`: array of array -> single array
- `ARRAY_DISTINCT`: array -> distinct element array
- [[#A40]]의 `ARRAY_UNION`으로 한번에 연산 가능할 듯

## Q43
You are working on IOT data where each device has 5 reading in an array collected in Celsius, you were asked to covert each individual reading from Celsius to Fahrenheit, fill in the blank with an appropriate function that can be used in this scenario.

`Schema: deviceId INT, deviceTemp ARRAY<double>`

![](https://img-c.udemycdn.com/redactor/raw/practice_test_question/2022-07-10_07-53-50-c2a0819f981fc87dc8230857f77bf702.JPG)

```sql
SELECT deviceId, __(deviceTempC,i-> (i * 9/5) + 32) as deviceTempF
FROM sensors
```
#### A43
`TRANSFORM`
- `trasnform(expr, func)`
	- array in `expr` 에 `func`를 모두 적용함
- c.f. `aggregate/reduce`는 단일값 변환 진행

## Q44
Which of the following array functions takes input column return unique list of values in an array?
#### A44
`COLLECT_SET`
- 원소 자체가 고유하게 끔 합치는 함수
- 모든 행에서 원소 자체가 겹친다면 하나만 출력
- `COLLECT_LIST`는 모든 원소를 하나의 array로
c.f. `ARRAY_UNION` 같은 경우,  `FLATTEN + ARRAY_DISTINCT`
## Q45
You are looking to process the data based on two variables, one to check if the department is supply chain or check if process flag is set to True
#### A45
- `if department == “supply chain” or process:`