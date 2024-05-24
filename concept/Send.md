---
date: 2024-05-25
datetime: 2024-05-25 08:10:38
book: 러스트_동시성_프로그래밍
page:
  - "39"
tags: 
references: 
aliases:
---
- 어떤 타입의 값을 다른 thread로 보낼 수 있다면
	- = 해당 타입의 값에 대한 소유권을 다른 스레드로 이전이 가능하다면
	- 해당 타입은 `Send` trait을 구현
- [[rust_arc|Arc]]`<i32>`는 Send를 구현하나, [[rust_rc|Rc]]`<i32>`는 그렇지 않음