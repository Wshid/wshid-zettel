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