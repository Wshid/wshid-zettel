---
date: 2024-07-06
datetime: 2024-07-06 21:57:10
book: 
page: 
tags: 
references:
  - https://codinghalbae.tistory.com/12
  - https://community.cloudera.com/t5/Support-Questions/Hive-Multiple-Small-Files/m-p/204046
  - https://trino.io/docs/377/connector/hive.html#alter-table-execute
aliases:
---

### Hive 쿼리 옵션
```
hive.merge.mapredfiles=true (default: false)
hive.merge.mapfiles=true (default: true)
hive.merge.tezfiles=true; (default: false)
hive.merge.size.per.task=256000000 (default: 256000000)
hive.merge.smallfiles.avgsize=200000000 (default: 16000000)
```
- hive.merge.smallfiles.avgsize
	- 출력 파일의 평균 사이즈 기준 지정
- hive.merge.size.per.task
	- 머지된 파일의 최대 크기

### CONCATENATE
```sql
ALTER TABLE table_name [PARTITION partition_spec] CONCATENATE
```
- ORC 파일에 대해 머지하는 방법

### Trino::EXECUTE(w/ dbt)
- https://trino.io/docs/377/connector/hive.html#alter-table-execute
- dbt로 수행시 아래와 같은 옵션 설정 필요
```sql
SET SESSION <catalog_name>.insert-existing-partitions-behavior=APPEND # OVERWRITE 사용 불가
SET SESSION <catalog_name>.non_transactional_optimize_enabled=true
```
- overwrite유지를 위해 pre_hook, post_hook 사용 가능
	- https://docs.getdbt.com/reference/resource-configs/pre-hook-post-hook
- 실제 수행시, 쿼리가 trino worker 수만큼은 최소 돌게 되면서 worker 수만큼 파일 생성
	- 의도한대로 올바르게 작동하지 않음

### DBT View
- https://docs.getdbt.com/terms/view
	- 테이블과 동일하게 데이터를 저장하지는 않음
	- 데이터를 가져와야 하는 논리 정의
	- 뷰 자체가 최종 목적지가 되서는 안됨
		- 데이터 모델링 속도가 느림
		- 테이블보다 쿼리 비용이 많이 드는 경우가 있음
		- 다운 스트림 프로세스에 연결하는데 좋지는 않음
- https://docs.getdbt.com/reference/resource-configs/trino-configs#view
- dbt-trino는 기본적으로 view matrialization을 사용시 아래와 같은 view_security 존재
	- definer(default)
		- 사용자로 쿼리 수행 x
		- 뷰의 소유자로 쿼리가 수행되는 모드(뷰의 작성자/정의자)
		- 사용자가 직접 액세스 할 수 없는 기본 테이블에 제한된 액세스 제공 가능
	- invoker
		- 뷰에 참조된 테이블이 액세스
		- 쿼리를 실행하는 사용자(뷰의 호출자)의 권한 사용
		- 생성된 view는 단순히 저장된 쿼리
- https://trino.io/docs/current/sql/create-view.html#security
	- 