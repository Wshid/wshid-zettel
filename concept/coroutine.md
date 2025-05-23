---
date: 2024-06-10
datetime: 2024-06-10 21:57:53
book: 크리에이티브_프로그래머
page:
  - "122"
tags: 
references: 
aliases:
---
일반적인 함수 호출과는 다르게 중간에 실행을 일시 중단하고, 나중에 재개할 수 있는
- 함수 또는 서브루틴의 일종
비동기 프로그래밍이나 협력적 멀티태스킹을 구현하는데 유용하게 사용


# 특징
- 일시 중단과 재개
	- 코루틴은 실행 중간에 일시 중단
	- 이후 중단된 지점부터 실행 재개 가능
	- 비동기 작업 효율적 처리 가능
- 비동기 프로그래밍
	- 비동기 작업을 동기식 코드처럼 작성 가능
	- File RW, Network IO 작업 처리시, 코드가 블로킹 되지 않도록 처리
- 상태 유지
	- 상태를 유지하면서 실행을 중단 및 재개 가능
- 협력적 멀티태스킹
	- 서로 양보하면서 수행 가능
	- 멀티 스레딩과 달리, 선점형 스케줄링이 아닌, [[협력적 스케줄링]]을 통해 실행됨