---
date: 2024-04-10
datetime: 2024-04-10 16:50:11
book: 
page: 
tags: 
references: 
aliases:
  - Trino Task
---
- Stage는 분산 쿼리 계획의 특정 세션 모델링
- Stage 자체는 Trino Worker에서 실행 x
- 분산 쿼리 계획은
  - 연속된 Stage가 분해되어 Task로 됨
  - Task는 Split 작업 실행
- Stage가 일련의 Task에 의해 병렬 실행 가능 하듯
  - Task는 **일련의 Driver와 병렬로 실행**
