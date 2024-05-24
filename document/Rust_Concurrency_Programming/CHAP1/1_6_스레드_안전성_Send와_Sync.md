---
date: 2024-05-25
datetime: 2024-05-25 08:03:23
book: 러스트_동시성_프로그래밍
page:
  - "39"
tags: 
references: 
aliases:
---
단일스레드에서만 사용 가능한 [[rust_rc|Rc]]나 [[Cell]]과 같은 스레드 간의 안전성 보장되지 않는 몇가지 타입에 대해 살펴홈
- 단일 스레드에서만 사용한다면 정의되지 않은 동작 제어 가능
컴파일러는 컴파일 타임에 이러한 조건이 충족되는지를 검사,
- `unsafe`블록을 사용하지 않고도, 이러한 타입 사용 가능

러스트에서는 어떤 타입이 멀티스레드에서 사용 가능한지를 구별하기 위해 두가지 trait을 사용

### [[Send]]

### [[Sync]]


`i32, bool, str`와 같은 **원시 타입**들은 [[Send]]와 [[Sync]]를 모두 구현
위 두가지 trait은
- 구조체 타입에 따라, 구조체에 해당 trait이 자동 구현되는 **자동 트레이트**(auto trait)
	- 구조체의 필드가 모두 [[Send]]와 [[Sync]]를 구현하는 구조체는, 그 자체로도 Send, Sync 구현

[[PhantomData<T>|std::marker::PhantomData<T>]]

