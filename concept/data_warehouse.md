---
date: 2024-04-10
datetime: 2024-04-10 15:43:38
book: 데이터_품질의_비밀
page:
  - "31"
tags: 
references: 
aliases:
  - 데이터 웨어하우스
---
- 구조화된 데이터 저장소
- schema on write
	- 데이터가 들어오는 즉시 데이터 구조 정의
- [[데이터 위생]] 개선

### DW 기술
#### redshift
- aws 기반
- source connector를 활용하여 원시 데이터 소스를 관계형 스토리지로 파이프라인을 통해 전달
- 열기반 저장 구조와 병렬 처리 → 분석 작업의 부하를 줄임
#### bigquery
- GCP
- 열기반 저장 구조
- 빠른 처리를 위한 병렬처리
- serverless solution
	- 사용패턴에 따라 확장
#### snowflake
- aws google azare 등의 퍼블릭 클라우드 기반
- 유연한 비용구조를 원하는 팀에게 유용


### DW의 단점

#### 제한된 유연성
- 스키마가 있는 표형식
- JSON 적합 X
#### SQL 전용 지원
- ML등 활용시 데이터를 꺼내야함
- 데이터 이동시 볼륨 및 신선도에 영향
- schema anomaly의 원인이 되기도 함
#### 워크플로에서의 마찰(frictional workflows)
- 신속한 작업에 때로는 장애
- 기준이 있기 때문
- 지속적인 스키마 변경에 부적합