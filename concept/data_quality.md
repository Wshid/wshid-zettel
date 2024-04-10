---
date: 2024-04-10
datetime: 2024-04-10 16:12:04
book: 데이터_품질의_비밀
page:
  - "26"
tags: 
references: 
aliases:
  - 데이터 품질
---
### 파이프라인별 품질 측정
- [[batch_pipeline]]과 [[streaming_pipeline]]에서 데이터 품질을 측정하는 방법
- [[data_downtime]]외의 시간을 [[up_time]]이라고 함
- [[up_time]]을 기준으로 데이터 신뢰성을 기준으로 품질을 측정함

### 데이터 품질의 정의
- data_reliability, data_completeness, data_accuracy를 측정하는 기능적인 측면부터 구체화
- 고품질 데이터는 강력한 분석 프로그램을 만든다는 [[필요조건]]
- 데이터가 비즈니스 요구사항을 만족한다는 강력한 요소
- data_lifecycle에 따른 단계별 상태로 정의
	- [[data_pipeline]]의 모든 단계 뿐 아니라 데이터 수집 전이나 운영 중 또는 분석 중에도 영향을 미침
- ML, data_science, data_analytics와 같이 전문적인 개념은 아니며
	- 조직에서는 데이터 품질가지 커버할 인력이 부족
	- 이로 인해 분석가나 엔지니어가 데이터 관리에 투입되는 일이 발생함

### 동향
- 2020년대 들어 기업들이 최우선순위로 고려하게 됨
	- 비즈니스와 직결되기 때문
- Data를 Code처럼 여기면서, 오랜 표준과 프레임워크를 데이터 조직, 아키텍처에 적용
	- [[dev_ops]]
	- [[sre]]
	- [[ci_cd]]
- [[data_ops]] 라는 개념으로 적용하기 시작