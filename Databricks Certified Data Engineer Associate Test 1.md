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
- Temporary view의 두가지 유형
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