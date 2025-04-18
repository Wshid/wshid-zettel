---
date: 2025-04-19
datetime: 2025-04-19 08:23:26
book: 
page: 
tags: 
references:
  - https://airflow.apache.org/docs/apache-airflow/stable/administration-and-deployment/priority-weight.html
aliases:
---
- executor queue의 우선순위 정의
- default: 1
- 임의의 정수 설정 가능
- weight_rule이 존재함
- pool 인자와 같이 사용 가능

### weight_rule
- weighting method를 지정하는 과정
- downsteam(default)
	- 모든 하위 항목의 집계, 하위 항목이 많을 경우
	- downstream 처리시에 모든 upstream이 완료되어야 하는 경우에 유용
- upstream
	- 상위 항목의 합계
	- 상위 작업을 시작하기 전에 각 DAG를 완료해야 하는 경우에 유용
- absolute
	- 추가 가중치를 정의하지 않은 Priority weight의 값
	- 각 작업에 어떤 우선순위 부여가 가능한지 정확히 알고 있을 때 수행하는 것이 좋음
	- 매우 큰 DAG의 경우 처럼 작업 생산 속도 크게 향상 효과 존재
## Custom Weight Rule
- 2.9.0 버전부터 도입됨
- `PriorityWeightStrategy`를 구현하여 설정 가능
- task 정의시 `weight_rule=custom_weight_rule_module.CustomPriorityWeightStrategy`로 지정 가능