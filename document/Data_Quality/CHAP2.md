---
date: 2024-04-23
datetime: 2024-04-23 21:52:07
book: 데이터_품질의_비밀
page: 
tags: 
references: 
aliases:
---
### 운영데이터와 분석데이터의 차이
- OLTP vs OLAP
#### 운영데이터
- 실제 비즈니스 프로세스
- 비즈니스 운영
- 트랜잭션 데이터베이스
- 짧은 지연시간에 중점

#### 분석 데이터
- 데이터 기반 의사 결정
- 운영 데이터의 집계, 확장 포함
- 높은 처리량에 중점

### [[data_warehouse|데이터 웨어하우스]]
- BI 도구와 함께 방법론 실현
- 데이터의 존재 이유
- 비즈니스 목표와 일치
- DW는 데이터 분석팀에 적합
- 패키징된 기능
- SQL에 대한 강력한 지원


데이터 조직은 분석 워크로드에 대해 DW DL을 모두 채택하여 사용

### [[data_lake|데이터 레이크]]


[[data_warehouse|데이터 웨어하우스]]는 데이터 조직이 효율적으로 운영하도록 구조 제공
- 하지만 특정 어플리케이션에 **유연하게 맞출 수 없음**
[[data_lake|데이터 레이크]]는 한없이 유연, 광범위한 사용 사례
- 하지만 데이터 구성 및 거버넌스에 대한 문제 발생

### [[data_lakehouse#특성|데이터 레이크하우스의 특성]]

### 데이터 웨어하우스와 데이터 레이크간 동기화
- 서로 다른 데이터 웨어하우스와 데이터 레이크는 데이터 통합 레이어로 연결됨
- 데이터 통합 툴
	- AWS Glue, Fivetran, Matillion
	- 서로 다른 소스에서 데이터를 수집
	- 데이터를 통합한 후 upstream source로 변환
- 일반적 사례
	- 레이크 속 데이터를 수집하여
	- 구조화된 형식으로 데이터 웨어하우스에 적재
- [[ETL]]


## 2.4. 데이터 품질 지표 수집

### 2.4.1. 데이터 품질 지표란?
- 온라인 app은 거의 모든 산업에서 [[mission_critical|미션 크리티컬]] 함
- 현재 데이터 조직에 정확하고 신뢰할 수 잇는 데이터에 대한 책임을 묻기 위해 [[Data_SLA_SLO_SLI#SLA, SLO, SLI||SLA]]를 시행하고 있지만, 아직 표준은 아님
- 아래 지표는 `데이터가 다운되었나?`라는 포괄적인 질문보다 구체적
#### 주요 지표
- 데이터가 최신 상태인가?
- 데이터가 완전한가?
- 필드가 예상 범위 내에 있는가?
- NULL 비율이 예상보다 높은 것은 아닌가?
- 스키마가 변경되었는가?

### 2.4.2. 데이터 품질 지표를 가져오는 법
- 위 내용의 답변은 데이터 자산 분석에서 나온 것으로
	- 앞서 설명한 리소스(데이터 웨어하우스, 데이터 레이크, 이 둘간의 레이어 변화)등에 따라 다름

#### 확장성
- 많은 수의 데이터, 및 데이터셋을 추적하는 작업은 까다로움
- 요청을 아래와 같이 처리
	- 한번에 처리
	- 규모에 맞게 쿼리 최적화
	- 중복 제거
	- 스키마 정규화
	- 확장 가능한 데이터 저장소에 저장
- 시간이 지남에 따라 운영과 업데이트 및 유지 관리할 수 있는 전용 파이프라인 구축 필요

#### 그 외 스택에 걸쳐 모니터링 하기
- [[data_observability|Data Observability]]를 달성하려면, 지표 수집 이상의 것이 필요
	- 스트리밍 데이터, 데이터 레이크, 대시보드, 머신러닝 모델 및 기타 자산의 신뢰성 모니터링
- 웨어하우스 뿐만 아니라, 다른 자산에서도 지표와 메타 데이터를 가져올 수 있는 방법 필요
- 이런 통합을 통해 **최종 사용자**에게 잘 맞는 솔루션에 투자하기
	- 포지션과 무관하게 중요
	- DE, AE, ML, ...
- snowflake 예시
	- 하지만 redshift, bigquery, 기타 인기 있는 OLAP 기반 웨어하우스에서 데이터 품질 정보를 가져올 때도 유사한 프로세스를 거침

#### 스노우플레이크에서 데이터 품질 지표 가져오기
- snowflake
	- 가장 인기 있는 클라우드 데이터 웨어하우징 툴 중 하나
	- 초기부터 데이터 품질, 무결성을 우선으로 하는 설계 채택
- 데이터 품질 지표를 가져와 쉽게 분석할 수 있도록 시각화 가능

#### 스노우플레이크 데이터 품질지표 수집 4단계

##### 1. 인벤토리 매핑하기
- `ANALYTICS`라는 단일 데이터베이스만 있다고 할 때
- 추적할 데이터베이스명을 `ANAYTICS`로만 변경하면 됨
- 웨어하우스에 있는 모든 테이블을 매핑하여 추적해야할 사항 파악
	- 매핑 스키마는 각 테이블에 어떤 내용이 있는지, 시간에 따라 어떻게 변화하는지를 이해하는데 유용한 도구
- 관련 메타 데이터가 있는 테이블 목록을 끌어오는 쿼리
```sql
SELECT
	TABLE_CATALOG,
	TABLE_SCHEMA,
	TABLE_NAME,
	TABLE_OWNER,
	TABLE_TYPE,
	IS_TRANSIENT,
	RETENTION_TIME,
	AUTO_CLUSTERING_ON,
	COMMENT
FROM 'ANAYTICS'.information_schema.tables
WHERE
	table_schema NOT IN ('INFORMATION_SCHEMA')
	AND TABLE_TYPE IN ('VIEW', 'EXTERNAL TABLE')
ORDER BY TABLE_CATALOG, TABLE_SCHEMA, TABLE_NAME;
```
- 스노우플레이크의 테이블에 대한 스키마 검색
```sql
SELECT
	'"' || TABLE_CATALOG || '"."' TABLE_SCHEMA || '"."' TABLE_NAME || '"' AS FULL_NAME,
	COLUMN_NAME,
	DATA_TYPE,
	COLUMN_DEFAULT,
	IS_NULLABLE,
	COMMENT,
	CHARACTER_MAXIMUM_LENGTH,
	NUMERIC_PRECISION,
	NUMERIC_SCALE,
	DATETIME_PRECISION
FROM "ANALYTICS".information_schema.columns;
```
- 뷰와 외부 테이블의 메타 데이터를 가져오려면, 아래와 같은 쿼리 필요
```sql
SHOW VIEWS IN DATABASE "ANALYTICS";
SHOW EXTERNAL TABLES IN DATABASE "ANALYTICS";
```
- 구현시 복잡성을 증가시킬 수 있으나,
	- 위의 쿼리는 `information_schema.tables`를 쿼리할때 사용할 수 없는 유용한 정보를 가져옴
	- e.g. 뷰에 대한 기본 SQL 쿼리를 조회하는 통찰력 제공

##### 2. 데이터 신선도와 볼륨 모니터링 하기
- 볼륨과 신선도를 추적하는 작업은
	- 스노우플레이크 [[data_observability]]와 [[data_pipeline|데이터 파이프라인]]을 이해하는데 매우 중요
- 스노우플레이크는 데이터가 [[data_warehouse|데이터 웨어하우스]] 테이블에 기록되는 것을 추적함
- 테이블에 있는 바이트 수, 최근에 업데이트 된 시간을 가져오는 쿼리
```sql
SELECT
	TABLE_CATALOG,
	TABLE_SCHEMA,
	TABLE_NAME,
	ROW_COUNT,
	BYTES,
	CONVERT_TIMEZONE('UTC', CREATED) as CREATED,
	CONVERT_TIMEZONE('UTC', LAST_ALTERED) as LAST_ALTERED
FROM "ANALYTICS".information_schema.tables
WHERE
	table_schema NOT IN ('INFORMATION_SCHEMA')
	AND TABLE_TYPE NOT IN ('VIEW', 'EXTERNAL_TABLE')
ORDER BY TABLE_CATALOG, TABLE_SCHEMA, TABLE_NAME;
```
- 위 지표를 저장하고 시간이 지남에 따라 어떻게 변경되는지 관찰
	- 테이블이 업데이트 되는 빈도와 각 업데이트에서 예상되는 데이터양 가늠 가능
- 누락된 데이터가 있는지, 데이터에 비정상적인 업데이트는 없었는지 식별
- 뷰와 외부테이블의 경우에는
	- 뷰의 볼륨과 신선도를 측정하는 것은,
		- 기본 쿼리에 포함된 테이블의 함수이기 때문에 **명확하지 않음**
	- 외부 테이블에 대해서는 `SHOW EXTERNAL TABLES...`부분을 활용하여 신선도 정보를 확인하는 것이 좋음

##### 3. 쿼리 기록 작성하기
- 모든 쿼리에서 정확한 기록을 보유 -> 문제 해결에 유용한 도구
- 최근 작성된 테이블을 언제, 어떻게 작성했는지 확인 가능
- 광범위한 쿼리 로그 분석 -> 테이블 간의 종속성 매핑시 도움
- 쿼리 로그 추출 쿼리
	- 노이즈를 줄이기 위해 시스템 및 오류 필터링
	- 이를 가지고 주어진 테이블을 **어떤 용도로 사용했는지**에 대한 중요한 정보 확인 가능 
```sql
SELECT
	"QUERY_TEXT",
	"DATABASE_NAME",
	"SCHEMA_NAME",
	"QUERY_TYPE",
	"USER_NAME",
	"ROLE_NAME",
	"EXECUTION_STATUS",
	"START_TIME",
	"END_TIME",
	"TOTAL_ELAPSED_TIME",
	"BYTES_SCANNED",
	"ROWS_PRODUCED",
	"SESSION_ID",
	"QUERY_ID",
	"QUERY_TAG",
	"WAREHOUSE_NAME",
	"ROWS_INSERTED",
	"ROWS_UPDATED",
	"ROWS_DELETED",
	"ROWS_UNLOADED"
FROM snowflake.account_usage.query_history
WHERE
	start_time BETWEEN to_timestamp_ltz('2021-01-01 00:00:00.000000+00:00')
		AND to_timestamp_ltz('2021-01-01 01:00:00.000000+00:00')
	AND QUERY_TYPE NOT IN ('DESCRIBE', 'SHOW')
	AND (DATABASE_NAME IS NULL OR DATABASE_NAME NOT IN ('UTIL_DB', 'SNOWFLAKE'))
	AND ERROR_CODE is NULL
ORDER BY start_time DESC;
```
- 작업 기록을 살피고, 데이터가 어떻게 적재되고 이동 하였는지 확인 가능
	- 테이블의 신선도를 이해하는데 도움을 줌

```sql
SELECT
	"FILE_NAME",
	"STAGE_LOCATION",
	"LAST_LOAD_TIME",
	"ROW_COUNT",
	"FILE_SIZE",
	"ERROR_COUNT",
	"STATUS",
	"TABLE_CATALOG_NAME",
	"TABLE_SCHEMA_NAME",
	"TABLE_NAME",
	"PIPE_CATALOG_NAME",
	"PIPE_SCHEMA_NAME",
	"PIPE_NAME",
	"PIPE_RECEIVED_TIME"
FROM snowflake.account_usage.copy_history
WHERE
	LAST_LOAD_TIME between to_timestamp_ltz('2021-01-01 00:00:00.000000+00:00')
		AND to_timestamp_lts('2021-01-01 01:00:00.000000+00:00')
		AND STATUS != 'load_failed'
ORDER BY LAST_LOAD_TIME DESC;
```

##### 4. 상태 점검하기
- 무언가 잘못되었음을 알리는 `NULL, 0`값을 모니터링하는 쿼리 수행
- 일부 중요한 테이블의 경우, 데이터 품질 검사를 실행
	- 모든 필드가 올바르게 채워지고
	- 정상적인 값을 가지고 있는가
- 시간 경과에 따른 **상태 메트릭**을 추적하고
	- 이를 과거 배치와 비교
	- 다양한 데이터 품질 문제가 나타나는 즉시 해당 문제를 찾을 수 있음

```sql
SELECT
	DATE_TRUNC('HOUR', created_on) as bucket_start,
	DATEADD(hr, 1, DATE_TRUNC('HOUR', created_on)) as bucket_end,
	COUNT(*) as row_count,
	-- string field
	COUNT(account_id) / CAST(COUNT(*) AS NUMERIC) as account_id___completeness,
	COUNT(DISTINCT account_id) AS account_id___approx_distinct_count,
	COUNT(DISTINCT account_id) / CAST(COUNT(*) AS NUMERIC) as account_id___approx_distinctness,
	AVG(LENGTH(account_id)) as account_id___mean_length,
	MAX(LENGTH(account_id)) as account_id___max_length,
	MIN(LENGTH(account_id)) as account_id___min_length,
	STDDEV(CAST(LENGTH(account_id) as double)) as account_id___std_length,
	SUM(IFF(REGEXP_COUNT(TO_VARCHAR(account_id), ... as account_id___text_int_rate,
	... as account_id__text_number_rate,
	... as account_id___text_all_spaces_rate,
	... as account_id___text_null_keyword_rate,
```
- 데이터의 특정 숫자 필드(분포)의 정확도를 추적하는 쿼리
	- `account_id`, `num_of_users` 두 필드에 걸친 데이터 분포에 대한 정보 수집

```sql
-- numeric-field
COUNT(num_of_users) / CAST(COUNT(*) AS NUMERIC) as num_of_users___completeness,
SUM(IFF(num_of_users = 0, 1, 0)) / CAST(COUNT(*) AS NUMERIC) as num_of_users___zero_rate,
SUM(IFF(num_of_users < 0, 1, 0)) / CAST(COUNT(*) AS NUMERIC) as num_of_users___negative_rate,
COUNT(DISTINCT num_of_users) / CAST(COUNT(*) AS NUMERIC) as num_of_users___approx_distinctness,
AVG(num_of_users) as num_of_users___numeric_mean,
MIN(num_of_users) as num_of_users___numeric_min,
MAX(num_of_users) as num_of_users___numeric_max,
STDDEV(CAST(num_of_users as double)) as num_of_users___numeric_std,
ARRAY_CONSTRUCT(APPROX_PERCENTILE(num_of_users, 0.00),
				APPROX_PERCENTILE(num_of_users, 0.20),
				APPROX_PERCENTILE(num_of_users, 0.40),
				APPROX_PERCENTILE(num_of_users, 0.60),
				APPROX_PERCENTILE(num_of_users, 0.80),
				APPROX_PERCENTILE(num_of_users, 1.00)) as nu_of_users___approx_quantiles

FROM analytics.prod.client_hub
WHERE
	DATE_TRUNC('HOUR', measurement_timestamp) >= DATEADD(day, -1, CURRENT_TIMESTAMP())
GROUP BY bucket_start, bucket_end
ORDER BY bucket_start ASC;
```
- `client_hub` 테이블의 두 필드에 대한 상태 지표 수집
- 문자열인 `accoutn_id` 필드는 아래 지표등을 추적함
	- 완전성(NULL이 아닌 값의 비율)
	- 유일성(고유값 비율)
	- UUID의 비율(UUID 형식과 일치하는 레코드 비율)
- 위 지표를 시간에 따라 추적하면
	- ID가 없는 계정, 중복 레코드, 잘못된 형식의 ID와 같은 일반적인 문제 식별 가능
- 숫자 필드인 `num_of_users`아래와 같은 지표들 추적
	- 값이 `0`인 레코드의 비율
	- 평균 및 분위수 
- 위 지표를 시간에 따라 추적하면
	- 데이터 누락으로 인해 사용자 수가 0이 되거나
	- 사용자 수가 크게 빗나가는 버그와 같은 일반적인 문제 식별 가능
- 확장성을 위해 최근 데이터만 추적하고, 과거 데이터가 이전 쿼리에 저장 되었다면,
	- 필요한 경우 샘플링과 함께
	- 상당 규모의 데이터셋을 효율적, 효과적인 비용으로 추적 가능
- 문제가 발생하여 [[data_pipeline|데이터 파이프라인]]에서 [[근본 원인 분석]]을 수행중인 팀이라면
	- [[data_quality|데이터 품질]]지표를 추적하기 위해 수집한 정보를
	- 다른 구성원이 쉽게 사용할 수 있도록 해야함
		- 문제가 감지될때마다 자동으로 알림을 보내는 방식
		- 워크 플로를 더 잘 처리하기 위해 사용하는 **중앙 집중형** UI 사용
	- 며칠동안 지속되는 데이터 재해를 빠르게 해결 가능

### 2.4.3. 쿼리 로그를 통한 데이터 웨어하우스의 데이터 품질 파악
- 웨어 하우스 환경에서 메타 데이터의 강력한 소스
- [[쿼리 로그]]


### 2.4.4. 쿼리 로그를 사용한 데이터 레이크의 데이터 품질 파악
- [[data_lake|데이터 레이크]]는 주로 허용하는 저장소 형식의 **유연성** 측면에서 [[data_warehouse|데이터 웨어하우스]]와 다름
- 데이터 레이크는 **원시 파일 형식**으로 저장하고 조작할 수 있는 [[읽기 스키마]]접근 프로토콜을 허용함
	- 명확한 장점이나, [[늪지화]] 위험을 증가 시킴
- 데이터 삽입시, 스키마가 시스템에 의해 강제 되지 않기 때문에
	- 데이터 웨어하우스에 존재하는 데이터 품질 지표를 얻기 어려움
- 하지만 요즘 데이터 품질을 보장하기 위한 방법은 많음
- 데이터 레이크는 새로운 데이터가 추가될 때 객체 메타데이터를 수집하고 저장
	- 의도치 않게 장점으로 활용 가능
	- e.g. AWS S3의 경우 객체 관리를 위해, 객체 삽입 시간과 payload 크기 저장 필요
		- 이를 활용하면, 객체의 마지막 업데이트 시기는 언제인지
		- 이 유형의 파일은 평균 크기가 얼마인지
		- 최근에 증가 추세인지를 확인할 수 있음
- 현대 데이터 레이크에서 제공하는 시스템 메타데이터는 다음을 포함함
	- 객체 삽입 시간
	- 객체 크기(바이트)
	- (인식되는 경우) 객체 파일 형식
	- 암호화 사용 여부
- 시스템이 저장하는 메타데이터 외, 생성시 **추가 객체 헤더**를 지정할 수 있음
	- 데이터 품질을 위한 솔루션은 보다 개방적
	- [[data_downtime|데이터 다운타임]]을 평가하는데 필요한 시스템 정의 메타데이터에 무엇이 필요할지 고민해보기
- 필요한 메타데이터 예시
	- 객체 생성에 관해 어떤 작업 파이프라인, 사용자에게 책임이 있는가?
	- 객체가 사용하거나 의존하는 스키마는 무엇인가?
		- ETL workflow에서 upstream 변환이 이뤄지고
		- 객체(리소스)가 특정 ETL workflow의 스키마에 따라 변환되었는지,
		- 또는 변환의 한쪽이 더 이상 사용되지 않았는지 여부 확인 가능
- 하지만, **이 객체 생성의 책임자는 누구인가**라는 질문에 답하는
	- 또 다른 일반적이고 전체적인 방법은
	- **업격하게 접근 권한**을 적용하고
	- **단일 파이프라인에만 쓰기 권한을 부여하는 것임을 명시하기**
- 이는, 데이터 품질이 좀 더 능동적으로 처리되도록 보장
- 데이터 품질에 대한 이해를 돕는 데이터 스택의 또 다른 핵심 요소는 **데이터 카탈로그**임
