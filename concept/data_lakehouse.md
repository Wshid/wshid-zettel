---
date: 2024-04-10
datetime: 2024-04-10 16:15:39
book: 
page: 
tags: 
references: 
aliases:
  - 데이터 레이크하우스
---

### 배경
- Redshift Spectrum, Databricks Lakehouse 기능 추가 
- Data Lake에도 SQL 질의 및 스키마 같은 [[data_warehouse|데이터 웨어하우스]]의 기능이 추가되는 추세
- 오늘날에는 warehouse와 lake의 기능적 차이가 줄어들 수 있으며
	- 이 두 형태의 장점을 결합한 **데이터 레이크하우스**가 확산되고 있음

### LakeHouse로의 마이그레이션
- [[data_pipeline|데이터 파이프라인]]이 점점 더 복잡해지고 있음을 시사
- 누군가는 [[data_warehouse|데이터 웨어하우스]]와 [[data_lake|데이터 레이크]]의 기능을 모두 활용하기 위해 단일 벤더의 기술을 사용할 것
- 반면 다수의 스토리지에 데이터를 마이그레이션 하고, 레이어를 각각 처리하는 사용자도 존재할 것
	- 단, 충분한 테스트를 거쳤더라도 [[data_downtime|데이터 다운타임]]이 발생할 가능성이 큼

### 특성
- Redshift Spectrum | Delta Lake
	- 자유도가 높은 형태의 이점 제공
- 최근 DW, DL의 격차가 많이 줄어듦

#### 우수한 성능의 SQL
- Presto, Spark와 같이 DL에서 interactive 속도의 SQL 제공
- DW에 대한 요약
- ETL 없이 DL가 분석 및 탐색 요구 직접 처리 가능

#### 스키마
- Parquet, DL에 더 엄격한 스키마 도입
- 쿼리 효율성을 높히기 위해 Columnar 형식 도입

#### ACID
- Delta Lake, Apache Hudi 등에서 RW 트랜잭션의 신뢰성을 높힘

#### 관리 서비스
- 클라우드 제공업체에서 운영 가능
- Databricks
	- Apache Hive, Delta Lake, Apache Spark 관리 버전
- Amazon Athena
	- 완전 관리형 Lake SQL Engine
- Amazon Glue
	- 완전 관리형 메타데이터 서비스 제공
