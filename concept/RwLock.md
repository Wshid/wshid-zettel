---
date: 2024-05-17
datetime: 2024-05-17 23:53:47
book: 러스트_동시성_프로그래밍
page:
  - "37"
tags: 
references: 
aliases:
  - reader-writer lock
---
- RwLock
	- reader-writer lock
	- [[RefCell]]의 동시성 버전
- `RwLock<T>`는 `T`를 가지고 있으면서, 값이 몇 번 대여되었는지 내부적으로 추적
- 하지만 [[RefCell]]과는 다르게
	- 서로 상충되는 소유권 대여가 발생하더라도 [[panic]]을 발생시키지 x
- 대신 상충되는 소유권 대여가 해결 되는 동안
	- 현재 스레드를 잠시 **차단**해서 **잠자기**상태로 만듦
- 다른 스레드가 현재 데이터에 접근하는 것이 끝나기를 기다렸다가
	- 현재 스레드가 데이터에 접근할 차례가 되면 작업 재개
- RwLock에서 값을 대여하는 것을 Lock 이라고 부름