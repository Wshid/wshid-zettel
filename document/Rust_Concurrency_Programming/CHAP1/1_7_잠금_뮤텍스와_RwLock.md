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

# 1.7.2. 잠금 오염
- `unwrap()`은 [[잠금 오염]](lock poisoning)과 연관됨

## MutexGuard의 라이프 타임
- MutexGaurd가 drop 될때 [[Mutex]]가 잠금 해제되는 것은 매우 편리하나, 예상치 못한일이 발생할 수도 있음
- `MutexGaurd`의 변수 이름을 `let`을 사용해 선언하면,
	- 지역 변수는 선언된 범위를 벗어날 때 drop 되므로
	- 해당 변수가 언제 drop 되는지 예측 가능함
- 하지만 명시적으로 메모리에서 guard를 삭제하지 않는 것은
	- 뮤텍스를 필요 이상으로 잠금 상태로 오래 유지시킬 가능성 존재
- MutexGaurd를 **변수로 선언하지 않고** 사용하는 것도 가능함
	- 몇몇 케이스에서는 편함
- MutexGuard가 보호된 데이터에 대한 [[독점 레퍼런스]]처럼 동작하기 때문에, 변수로 만들지 않고 바로 사용 가능
- 예시
	- 한줄의 코드로 `Mutex<Vec<i32>>` 타입의 뮤텍스를 잠금 상태로 만들고
	- Vec에 새로운 원소를 추가한 다음 다시 뮤텍스를 잠금 해제
		```rust
		list.lock().unwrap().push(1);
		```
	- `lock()`이 리턴하는 `MutexGuard`와 같은 모든 임시 변수들은, 해당 선언문이 끝날 때 삭제 됨
		- 당연해보이지만, `match, if, let, while let`을 사용하는 경우 문제 발생  
		```rust
		if let Some(item) = list.lock().unwrap().pop() {
			process_item(item);
		}
		```
		- 리스트를 잠근 다음, 아이템을 하나 꺼내고, 리스트를 잠금 해제 한 후 다음 작업 처리
			- `MutexGuard`가 `if let` 문이 끝날 때 까지 제거되지 않음
			- 꺼낸 아이템을 처리하는 동안 Mutex가 잠금 상태로 남음
	- 단, 아래 코드는 문제가 발생하지 않음
		```rust
		if list.lock().unwrap().pop() == Some(1) {
			do_something();
		}
		```
		- `MutexGuard`가 if문이 시작되기 전에 삭제 됨
			- `if`문의 조건문은 항상 아무 값도 대여하지 않는 boolean 값이기 때문
			- 조건문에 사용된 변수를 `if`문 끝까지 연장할 필요가 없음
	- `if let`을 사용할 경우,
		- `pop()`이 아닌 `front()`를 쓴다면
			- `item`은 `list`로부터 값을 대여하므로 guard를 범위 안에 계속 유지해야 함
		- 대여 체커(borrow checker)는 여기서 검사만 진행,
			- 실제로 언제 그리고 무엇이 드랍되는지는 **영향을 주지 않아서** (`pop()`)을 사용하는 경우 같은 일 발생
	- 단, `pop()`을 별도 라인으로 분리하면 문제 해결
		- `guard`가 `if let`에 들어가기 전에 드랍
		```rust
		let item = list.lock().unwrap().pop();
		if let Some(item) = item {
			process_item(item);
		}
		```