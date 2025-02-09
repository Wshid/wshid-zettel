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

# Introducing Maestro
- Maestro는 엔지니어, 비엔지니어의 모두의 다양한 요구사항 충족
- loosely coupled way로 common function, reusable patterns가 존재
- Workflow definition: JSON format
	- [Workflow denifinition example](https://github.com/Netflix/maestro/wiki/Workflow-definition-example)
- Workflow의 구조
	- 속성 및 버전이 지정된 워크플로 두가지 중요 섹션으로 구성
	- Properties
		- author, owner information
		- execution settings
		- Maestro는 workflow 버전간 중요 키 값을 보존함
		- 해당 일관성을 통해 관리 간소화
		- workflow ownership이 변경되면, 새 소유자는 버전을 만들지 않고도 ownership 주장 가능
		- User can also enabeld the triggering or alterting features for a given workflow over the properties
	- Version
		- Versioned worfklow는 여러 속성 포함
			- unique identifier, name, description, tags, timeout settings, criticality levels(low, medium, high)
		- workflow 변경시마다 새 버전이 기록되며, 자유롭게 reversion 가능
			- 기본적으로 latest version이 사용됨
	- Step
		- job, 하위 worfklow step을 사용하는 다른 workflow 혹은 foreach step을 사용하는 loop 표현 가능
		- step은 unique identifiers, step types, tags, input & output step parameters,
			- step dependencies, retry policeis, failure mode, step outputs, etc.
		- 오류 유형에 따라 구성 가능한 재시도 정책을 지원
			- step resilience(복원력)을 향상함
- 핵심은 복잡한 workflow를 유연하게 정의할 수 있음

## Workflow Run Strategy
- Users want to automate data pipelines
	- while retaining control over the execution order
	- 실행 순서를 제어하면서 데이터 파이프라인 자동화
- This is crucial when workflow cannot run in parallel or must halt current executions when new ones occur
- Maestro는 미리 정의된 실행 전략을 가지고 workflow instance 실행 여부 결정
- predefined run stategies

### Sequential Run Strategy
- default, FIFO order
- An execution does not depend on the previous states
- Once a workflow instance reaches one of the terminal states(succeeded or not)
	- will start the next one in the queue

### Strict Sequential Run Strategy
- FIFO로 trigger를 수행하나, workflow instance에 blocking error가 있는 경우 중단
- 새로 trigger된 workflow instnace는 실패한 인스턴스를 수동으로 다시 시작하거나
	- 실패한 인스턴스를 차단 해제 표시하여 오류가 해결될 때까지 대기열 대기
- 예시
	- run5가 오전 5시에 실패하고, 이후 실행은 대기열에 있으나 수행되지 않음
	- run5를 수동으로 차단 해제 하거나 다시 시작시 worfklow 수행
- 위 실행 전략은 time insensitive but business critical workflow에 유용
- owner가 추후 실패 검토 이후 정확성을 확인할 수 있으며
	- 이후 실행 차단 해제할 수 있는 옵션이 발생함

### First-only Run Strategy
- 실행중인 workflow가 완료되었는지 확인한 후 새 workflow instance를 queue에 추가
- 만약 workflow instance가 실행 중일 때 새 워크플로 인스턴스가 대기열에 추가되면
	- Maestro는 대기열에 추가된 인스턴스를 제거함
- 현재 실행중인 worfklow instance가 없는 경우에만 새 workflow instance 수행 방식
	- 대기열을 효과적으로 해제 가능
- **새 workflow를 instance queue에 두지 않으면서, 비활성화 문제를 방지하는데 도움이 됨**

### Last-only Run Strategy
- 실행중인 workflow가 가장 최근에 트리거된 워크플로인지 확인하고 last instance만 유지
- 이미 실행중인 기존 workflow가 있는 상태에서 새 workflow가 대기 중인 경우
	- Maestro는 실행중인 인스턴스 제거 및 새로 트리거된 인스턴스 수행
- 이 기능은 전체 테이블의 최신 스냅샷을 매번 처리하는 등
	- **워크플로가 항상 최신 데이터를 처리하도록 설계된 경우에 유용**

### Parallel with Concurrency Limit Run Strategy
- predefined Concurrency limit에 따라 여러 worfklow를 병렬 수행
- **오래된 데이터를 다시 채울 때(backfill)시 활용**

## Parameters and Expression Language Support
- parameters는 중요한 역할
- Maestro supports dynamic parameters with code injection
	- allowing using parameters to conrol execution logic
	- enable state sharing between workflows and their steps
		- downstream & upstream
- 다른 Maestro 기능과 같이 사용할 때 
	- workflow를 dynamic하게 정의하고 복잡한 사용 사례에 맞게 parameterized workflow를 구성할 수 있음
- 