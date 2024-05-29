---
date: 2024-05-29
datetime: 2024-05-29 21:05:14
book: 러스트_동시성_프로그래밍
page:
  - "41"
tags: 
references: 
aliases:
---
[[Mutex]]

# 1.7.1. 러스트의 뮤텍스
- `std::sync::Mutex<T>`로 사용
	- 뮤텍스가 보호하고 있는 데이터의 타입인 `T`의 제네릭 형태
- `T`를 뮤텍스의 일부로 사용하면
	- 모든 스레드들이 오직 뮤텍스로 내부 데이터에 접근할 수 있는 안전한 인터페이스 제공
- 뮤텍스를 잠근 스레드만 잠금 해제 할 수 있도록
	- `unlock()`메서드를 가지지 않음
	- 대신, `lock()`에서 `MutexGuard`를 리턴함
- `MutexGuard`는 현재 뮤텍스가 잠겨있음을 의미
- `DerefMut` [[rust_trait|trait]]을 통해 [[독점 레퍼런스]]처럼 사용 가능
	- 뮤텍스가 보호하는 데이터에 접근 가능
- `MutexGaurd`가 drop되는 순간, 뮤텍스는 즉시 잠금 해제 상태가 됨
	- `MutexGaurd`가 drop되면 더 이상 현재 스레드에서 뮤텍스가 보호하는 데이터 접근 불가
	- `MutexGuard::Drop` [[rust_trait|trait]] 구현체가 뮤텍스의 잠금 해제 수행

```rust
use std::sync::Mutex;
fn main() {
	let n = Mutex::new(0);
	thread::scope(|s| {
		for _ in 0..10 {
			s.spawn(|| {
				let mut guard = n.lock().unwrap();
				for _ in 0..100 {
					*guard +=1;
				}
			});
		}	
	});
	assert_eq!(n.into_inner().unwrap(), 1000);
}
```
- 정수 값을 보호하는 `Mutex<i32>`를 선언
	- 총 10개의 스레드를 생성하여
	- 각각의 스레드에서 뮤텍스가 보호하는 정수를 100번 증가 시킴
- 각 스레드는 먼저 뮤텍스를 잠금상태로 만들어
	- `MutexGuard`를 얻은다음
	- `guard`를 이용해 정수에 접근하여 값을 1씩 증가시킴
	- `guard`는 범위를 벗어나면 drop됨
- 모든 스레드의 작업이 끝나면, 뮤텍스에서 `into_inner()`를 사용하여 안전하게 값 제거 가능
	- `into_inner()`
		- 뮤텍스의 소유권을 가져감
		- 더 이상 뮤텍스의 레퍼런스가 존재할 수 x
		- 더 이상 뮤텍스를 잠금 필요가 없어짐
- `for` 루프에서 정수를 1씩 증가 시켰으나,
	- 스레드들이 값을 증가시키기 시작하는 순간에, 정수가 항상 100의 배수를 값으로 가짐
	- **뮤텍스의 잠금이 해제**되어야만 값에 접근이 가능하기 때문
	- 뮤텍스 덕분에 정수의 값을 `100`씩 증가시키는 작업은
		- 더 이상 나눌 수 없는 `atomic`한 작업 단위가 됨

뮤텍스 잠금 전 1초를 기다리는 코드
```rust
use std::time::Duration;
fn main() {
	let n = Mutex.new(0);
	thread::scope(|s| {
		for _ in 0..10 {
			s.spawn(|| {
				let mut guard = n.lock().unwrap();
				for _ in 0..100 {
					*guard +=1;
				}
				thread::sleep(Duration::from_secs(1));
			});
		}
	});
	assert_eq!(n.into_inner().unwrap(), 1000);
}
``` 
- 프로그램 수행시 약 10초 정도 소요됨
- 스레드들이 각 1초를 기다리나, 한번에 한 스레드만 접근 가능하기 때문

`guard`를 강제로 메모리에서 삭제하면, 1초간 잠자기 상태 전 뮤텍스가 즉시 잠금 해제되어 동시 수행
```rust
use std::time::Duration;
fn main() {
	let n = Mutex::new(0);
	thread::scope(|s| {
		for _ in 0..10 {
			s.spawn(|| {
				let mut guard = n.lock().unwrap();
				for _ in 0..100 {
					*guard += 1;
				}
				drop(guard); // 스레드 대기 이전에 guard 제거
				thread::sleep(Duration::from_secs(1));
			});
		}
	});
	asert_eq!(n.into_inner().unwrap(), 1000);
}
```
- 프로그램은 단 1초만에 수행됨
- 뮤텍스를 잠금 상태로 만들었을 때, 소요되는 시간을 최소화 해야함
- 뮤텍스를 필요 이상으로 잠금 상태로 사용시
	- 병렬 처리의 이점이 사라지고, 작업이 순서대로 진행됨
