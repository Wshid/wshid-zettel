---
date: 2024-04-14
datetime: 2024-04-14 13:24:40
book: 러스트_동시성_프로그래밍
page: 
tags: 
references: 
aliases:
  - std::thread::spawn
---
- Rust Spawn
- `std::thread::Builder::new().spawn().unwrap()`을 간편하게 사용하기 위한 형태
- 함수명, closer 전달 가능
- `spawn`함수에서 리턴된 `JoinHandle`을 사용
- `join()`
	- 스레드가 작업을 종료할때까지 대기
	- `std::thread::Result`를 리턴
- static lifetime을 받는 타입을 입력 받음

### 비정상 종료
-  `std::thread::Result`는 [[panic]] 의 message 포함
- 이를 활용하여 예외 처리하거나 `unwrap()`을 사용하여 종료시 `panic` 발생하도록 유도 가능
