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