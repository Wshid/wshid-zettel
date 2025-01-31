---
date: 2025-01-31
datetime: 2025-01-31 22:26:55
book: 
page: 
tags: 
references:
  - https://netflixtechblog.com/maestro-netflixs-workflow-orchestrator-ee13a06f9c78
  - https://github.com/Netflix/maestro
aliases:
---

## What is Maestro
- horizontally scalable workflow orchestrator
	- manager large-scale Data/ML Workflow
- 재시도, 대기열, 컴퓨팅 엔진으로의 작업 분배 등을 포함하여
	- 처음부터 끝까지 워크플로 life cycle 감독
- Docker Image, Notebook, bash script, SQL, Python 등 다양한 형식으로 비즈니스 로직 패키징 가능
- DAG뿐 아니라 여러 추가 기능 지원
	- acyclic/cyclic workflow
	- multiple reusable patterns
	- foreach loops
	- subworkflow
	- conditional branch

## Our Journey with Maestro
- user를 대신하여 최소한의 중단으로 수십만 건의 워크플로를 성공적으로 마이그레이션
	- on behalf of: ~를 대신하여
	- hundreds of thousands: 수십만
	- seamless: 원활한
	- ever-growing: 계속 증가하는
- Maestro launches thousands of workflow instance
	- half a million jobs daily on average
	- around 2 million jobs on particularly busy days


## Scalability and Versatility(확장성 및 다용도성)
- fully managed workflow
	- Workflow-as-a-Service: 서비스로서의 워크 플로우
	- end user, applications, and services at Netflix
- wide range of workflow use cases
	- ETL pipelines
	- ML workflows
	- AB test pipelines
	- pipelines to move data between different storages
- can manage both a large number of workflows and a large number of jobs within a single workflow
	- 단일 워크플로 내에서 많은 수의 워크플로 및 작업 처리 가능
- At Netflix, workflows are intricately(복잡하게) connected.
- This approach also requires additional mechanisms to coordinate(조정하다) these fragmented workflows
	- 파편화된 워크플로를 조정하기 위한 추가적인 매커니즘 필요
- Netflix's data tables are housed in a single DW,
	- we believe a single orchestrator should handle all workflows accessing it