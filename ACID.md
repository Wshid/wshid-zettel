---
date: 2024-08-17
datetime: 2024-08-17 12:52:25
book: 
page: 
tags: 
references: 
aliases:
---
### 원자성(Atomicity)
- 각 트랜잭션이 완전히 성공하거나 완전히 실패하는 단일 이벤트로 실행됨
- 중간 상태는 존재하지 x

### 일관성(Consistency)
- 쓰여진 모든 데이터가 데이터 레이크의 정의된 규칙에 따라 유효성을 갖도록 함
- 데이터의 정확성, 신뢰성 보장

### 격리(Isolation)
- 여러 트랜잭션이 서로 방해하지 않으면서 동시에 발생할 수 있도록 함
- 트랜잭션이 독립적으로 실행되도록 보장

### 지속성(Durability)
- 트랜잭션이 제출된 후 데이터가 손실되거나 손상되지 않음을 의미
- 정전과 같은 시스템 장애 발생시에도 데이터를 복구할 수 있음