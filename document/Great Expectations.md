---
date: 2024-07-27
datetime: 2024-07-27 12:10:13
book: 
page: 
tags: 
references:
  - https://docs.greatexpectations.io/docs/oss/tutorials/quickstart/
aliases:
---
Great Expectations의 기본 구성 요소 및 개념 파악

### Data Context
- https://docs.greatexpectations.io/docs/reference/learn/terms/data_context/
- GX의 주요 entrypoint
- Data Source 생성
- Expectations 생성
- CheckPoint 관리

### Data Asset
- https://docs.greatexpectations.io/docs/reference/learn/terms/data_asset
- 데이터 자산
	- 테이블, 쿼리, 특정 파일, 데이터를 관리할 수 있는 단위
	- 논리적 개념
- 데이터 연결시, 각 데이터 소스에서 하나 이상의 데이터 자산을 정의함
- 새로운 데이터 Batch
- DataSource와 연관이 되어 있음
	- 여러개의 DataAsset을 정의할 수 있음
- DataAsset을 추가한 뒤 Splitter를 추가할 수 있음
- GX에서 Quality를 보장하는 단위

### Batch
- https://docs.greatexpectations.io/docs/reference/learn/terms/batch
- [[Great Expectations#Data Asset|Data Asset]]에서 레코드를 선택한 것
- Batch는 [[MECE]]로 설계됨
- 데이터 소스의 데이터를 슬라이스 하는 방법을 정의한 후(=하나로 슬라이스 할 수도 있음)
	- 특정 배치를 '배치'로 만드는 요인을 결정
- GX가 검증하고 메트릭을 수집할 기본 단위
- Metric은 항상 '데이터 배치'와 연결됨
- Validator
- Checkpoint, Exepectation Suite
- Batch는 Batch Request에 의해 생성됨

### Batch Request
- https://docs.greatexpectations.io/docs/reference/learn/terms/batch_request/
- data의 batch 형태
- [[#Data Asset]]의 `build_batch_requset`를 통해 생성
- 기본적으로 데이터를 쿼리하는 모든 세부 정보가 포함됨
- `Batch Request`와 [[#Batch]]로 반환되는 데이터의 관계가 보장됨

### Checkpoint
- https://docs.greatexpectations.io/docs/reference/learn/terms/checkpoint
- 운영환경에서 데이터를 검증하는 수단
- [[#Batch]]에 대한 [[#Validation]]을 [[#Expectation Suite]]와 비교하여 번들링 하기 위한 추상화
- 검증 후에 수행해야 하는 작업 제공
- [[#Expectation Suite]]와 [[#Validation Results]]와 마찬가지로 [[#Data Context]]와 같이 관리되며,
	- `yaml`파일을 유지하기 위한 자체 `store`가 존재함
- 버전 제어가 관리되며, 팀과 공유도 가능
- ![image](https://github.com/user-attachments/assets/57dd528f-f094-48a2-b079-ac98758badf2)
### Expectation Suite
- https://docs.greatexpectations.io/docs/reference/learn/terms/expectation_suite
- 여러개의 [[#Expectation]]의 집합
- `my_database.my_table`과 같이 활용
- 주어진 프로젝트에서 이름은 고유해야 함
- Validator를 사용하거나 Data Assistant를 사용하여 자동 생성됨
- [[#Checkpoint]]에서 데이터를 검증하는데 사용됨

### Validator
- https://docs.greatexpectations.io/docs/reference/learn/terms/validator/
- 데이터에 대해 [[#Expectation Suite]]를 실행하는 개체
- 추가 구성이 필요하지 않으며 [[#Expectation Suite]]와 [[#Batch Request]]가 함께 제공되면 동작
- [[#Expectation Suite]]에 대한 Expectation 생성시 대부분의 workflow에서 사용
- [[#Checkpoint]]은 [[#Batch Request]]에 대해 [[#Expectation Suite]]를 실행할때 validator를 


### Expectation
- https://docs.greatexpectations.io/docs/reference/learn/terms/expectation
- `expect_column_values_to_not_be_null`과 같이 검증 가능한 assertion
- customize 가능

## 사용 예시 정리
- trino에 대해 data_source를 연결하여 validator를 통한 검증까지 수행하는 예시

### DataSource 생성하기
```python
import great_expectations as gx

context = gx.get_context()
connection_string = "trino://account:password@host:port/hive"
datasource = context.sources_add_or_update_sql(
	name="prod-trino", connection_string=connection_string
)
```

### DataAsset 생성 및 splitter 적용
```python
# datasource = context.get_datasource("prod-trino")

asset_name="base__table"
schema_name="base"
table_name="table"
table_asset=datasource.add_table_asset(name=asset_name, 
									   schema_name=schema_name,
									   table_name=table_name)

# table_asset에서 spliter를 추가함
table_asset.add_spliter_column_value("dt")
```
- 관련 구문이 수행된 후, `great_expectations.yml`에 관련 asset이 추가된다
- `dt`라는 파티션을 기준으로 column 값을 분리

### BatchRequest 생성하기
- table_asset이 파티션된 테이블일 경우, `dt`로 분리하여 배치로 구성
- 이후 `batch_request`에서 `dt=2024-07-27`에 해당하는 내용만 필터링하여 가져옴
```python
batch_request = table_asset.build_batch_request({'dt': '2024-07-27'})
table_asset.get_batch_list_from_batch_request(batch_request)
```

### Expectation/Expectation Suite 생성
```python
from great_expectations.core.expectation_configuration import (
	ExpectationConfiguration,
)

ec2 = ExpectationConfiguration("expect_column_values_to_not_be_null", {"column": "uuid"})

suite_store = context.add_or_update_expectation_suite(expectation_suite_name="base.table")

suite_store.add_expectation(expectation_configuration=ec2)

# expectation_suite를 fs에 저장하는 과정
context.save_expectation_suite(suite_store, overwrite_existing=True)
```

### Validator 정의
```python
validator = context.get_validator(
	batch_request = batch_request,
	expectation_suite_name="base.table"
)

validator.expect_column_values_to_not_be_null(column="uuid")
```
- trino에서는 현재 `dt`가 date column 기준일때 문제가 발생함
	- expect를 하기 위해 trino 쿼리를 수행할 때,
	- `WHERE`조건에서 `dt=CASE('2024-07-27' AS DATE)`로 치환되어야 하나,
	- `WHERE dt='2024-07-27`형태로 조회하면서 문법 에러가 발생