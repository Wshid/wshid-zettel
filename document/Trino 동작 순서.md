---
date: 2024-04-10
datetime: 2024-04-10 16:53:21
book: 
page: 
tags: 
references: 
aliases:
---
### FLOW
- ![image](https://github.com/Wshid/daily-poc/assets/10006290/c527e572-fe49-410e-930a-75c81e0bfc16)
- Trino Worker ps 시작시
  - Coordinator::Discovery Serveice 등록
    - Coordinator가 Task를 Worker에 배정 가능
  - Discovery Service <-> Worker, Health check
- [Client] HTTP 기반, Coordinator에게 쿼리 전송
- [Coordinator] 쿼리 구문 분석, query plan 생성
  - Query Plan: 데이터 처리 ~ 결과 반환까지의 모든 단계
  - Parser/Analyzer: Table/Column, Type에 대한 정보 수집
  - Planner: row 개수, 테이블 크기에 대한 정보 수집
  - Scheduler: Query Plan 생성
- [Coordinator] Query Plan에 필요한 스키마 데이터를 Connector Plugin에 요청
- [Coordinator] Worker로 수행해야할 task 전달
- [Worker] Connector Plugin을 통해 Data Sources로부터 데이터를 읽음
- [Worker] Query Plan에 따라 할당된 task를 메모리에서 수행
- 실행결과를 Coordinator를 거쳐 Client에게 전달
