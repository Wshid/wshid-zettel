---
date: 2024-04-10
datetime: 2024-04-10 11:19:23
tags: 
book: 데이터_품질의_비밀
page:
  - "26"
references:
---
### 개념
- 데이터가 수집되지 않아 누락되거나 부정확하게 측정되는 상황
- 데이터 손실로 인해 소프트웨어 또는 서비스의 가동이 중지
- 데이터를 다루는 기업들이 직면할 수 있는 문제
	- 대시보드에 잘못된 데이터를 표시, **잘못된 의사결정** 
- 데이터 품질 문제를 처리하기 위해 전체 업무 시간의 40% 이상을 소모함
- 소프트웨어 엔지니어링, 개발 및 운영 모두와 연관됨

### [[data_quality]] 이슈
- 데이터 베이스 스키마 변경에 따른 [[data_pipeline]] 중단
- 행 또는 열 중복 현상
- 대시보드 내 오류값 발생


### 해결 방법
- 데이터 품질을 유지, 관리하는 방법 제시

### 데이터 다운타임이 증가하는 이유
- 클라우드 마이그레이션
	- 데이터 분석의 확산, [[교차 가능 조직]]의 등장
	- [[Amazon Redshift]], [[Snowflake]], [[Google BigQuery]]
- 많은 데이터 소스
	- 데이터 소스를 많이 사용하면서, 하나라도 예상하지 못한 변경이나 이슈가 있다면 데이터 셋도 변경 됨
- 데이터 파이프라인의 복잡성 증가
	- 데이터 종속성을 정확히 파악하지 않으면
	- 특정 데이터셋 변경시, 다른 데이터셋의 정확성에 의도치 않은 결과 반환