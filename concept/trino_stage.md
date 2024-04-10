---
date: 2024-04-10
datetime: 2024-04-10 16:49:10
book: 
page: 
tags: 
references: 
aliases:
  - Trino Stage
---
-  Trino는 Query 수행시 **계층 구조**로 분할하여 쿼리 실행
- Trino가 데이터를 집계해야하는 경우
  - Root Stage를 만들어 여러 Stage 출력 집계
- 모두 분산 쿼리 계획의 **다른 섹션**을 구현하도록 설계
- **Query를 구성하는 Stage 계층 구조**
  - Tree와 유사함
- Stage는 Coordinator가 **분산 쿼리 계획**을 모델링 할때 사용되나,
  - Stage 자체는 **Trino Worker**에게 실행되지 않음
