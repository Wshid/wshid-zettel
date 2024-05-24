---
date: 2024-05-25
datetime: 2024-05-25 08:11:08
book: 러스트_동시성_프로그래밍
page:
  - "39"
tags: 
references: 
aliases:
---
- 어떤 타입의 값을 여러 스레드에 공유할 수 있다면
	- 해당 타입은 `Sync` trait을 구현함
- 공유 레퍼런스 [[reference_&T|&T]]가 Send를 구현하는 경우에만 `T`는 Sync Trait을 구현함
- i32는 Sync trait을 구현하지만 [[Cell]]`<i32>`는  아님
	- [[Send]]를 구현