---
date: 2024-04-14
datetime: 2024-04-14 11:58:16
book: 러스트_동시성_프로그래밍
page: 
tags: 
references: 
aliases:
---

- OS는 각 ps를 최대한 격리, ps가 다른 ps에 영향받지 않도록 함 
- ps는 다른 ps의 메모리 공간에 접근 x
	- 직접 통신도 불가능
	- 통신은 OS의 kernel을 통해 이루어짐
- ps는 여러개의 thread를 만들 수 있음
	- thread는 서로 격리되지 않음
	- 메모리 공유 및 상호작용 가능

- Rust의 thread가 어떻게 만들어지는지
- thread내 안전하게 메모리를 공유하는 방법

### 0. docker run
```bash
docker run --name rust_concurrency_programming -v /Users/sion/Workspace/wshid-zettel/workspace/rust:/root/workspace -it rust:latest /bin/bash
```

### 1.1 러스트의 스레드
- 모든 program은 main thread로부터 수행
	- main thread, main 함수 수행, 다른 스레드 실행
- 새로운 스레드의 생성
	- `std::thread::spawn`
		- 인수: 스레드에서 실행시킬 함수
- 수행시마다 결과가 달라짐


#### [[1_1_hello_world.rs]]
```bash
# in docker
cd /root/workspace/CHAP_1_1
rustc 1_1_hello_world.rs
./1_1_hello_world.rs
```
![[Pasted image 20240414130345.png]]
- 매번 수행시마다 결과가 다름
- main 함수가 종료되면 스레드가 종료됨

### [[1_1_hello_world_wait.rs]]
![[Pasted image 20240414130835.png]]
- [[spawn]]함수에서 리턴된 `JoinHandle`을 사용
- thread에서 [[panic]]이 발생하여 비정상 종료시
	- `std::thread::Result`는 panic의 message 포함
		- 이를 활용하여 예외 처리하거나 `unwrap()`을 사용하여 종료시 `panic` 발생하도록 유도 가능
- `println`의  [[출력 잠금]]
- [[spawn|std::thread::spawn]]의 closer 전달
	- 실제로 함수명보다 closer 전달이 더 잦음
	- closer를 사용하면, **특정 값을 스레드 안으로 이동**하는 것이 가능
```rust
let numbers = vec![1,2,3];

thread::spawn(move || {
	for n in numbers {
		println!("{n}");
	}
}).join().unwrap();

```
- numbers의 소유권이 새로 만들어진 thread로 이동
	- `move` 키워드가 `closer`에 사용되었기 때문
- `move`가 사용되지 않았다면, closer는 `number`를 reference로 `capture`하여
	- compile error 가 발생함
- 소유권을 빌린 변수 `numbers`보다 `thread`가 더 오래 지속될 수 있기 때문

#### static lifetime 타입
- thread는 program이 종료될 때까지도 계속 수행 가능하기 때문에
	- [[spawn|std::thread::spawn]] 함수는 인수로 `static lifetime`을 갖는 타입을 입력 받음
- **프로그램이 종료될 때까지 존재할 수 있는 함수**만 입력 받음
- `reference`로 **지역 변수**를 캡처하는 클로저는
	- 더 이상 지역변수가 존재하지 않는 순간 
	- reference가 유효하지 않음
	- 영원히 유지 불가
- closer의 return 값은 thread로 전달
	- `join`메서드 호출시 `Result`로 감싸져 전달
```rust
let numbers = Vec::from_iter(0..=1000);

let t = thread::spawn(move || {
	let len = numbers.len();
	let sum = numbers.into_iter().sum::<usize>();
	sum / len // 1
});

let average = t.join().unwrap(); //2
println!("average: {average}") 
```
- thread의 closer(1)에서 리턴된 값은
	- join(2)를 통해 메인 스레드로 전달
- `if numbers==0`,
	- 1에서 panic 전달
	- unwrap(2)에서 main thread panic 발생

#### thread builder
- [[spawn|std::thread::spawn]]
- `std::thread::Builder`, 스레드를 만들기 전 여러가지 설정 가능
	- e.g. stack memory size, thread name(`std::thread::current()::name())
	- thread name은 panic message에 포함(모니터링 도구, 디버깅 도구에서 확인 가능)
- thread 생성이 실패할 수 있기 때문에 `Builder::spawn`은 `std::io::Result`를 리턴
	- 이유?
		- os의 memory 부족
		- program의 메모리와 같은 리소스 할당 제한
	- 실패시 `std::thread::spawn`은 [[panic]]