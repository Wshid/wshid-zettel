---
date: 2024-05-25
datetime: 2024-05-25 08:31:15
book: 
page: 
tags: 
references: 
aliases:
---
타입들이 공통적으로 갖는 동작을 추상화
- 다른 언어의 interface와 유사한 개념

특징
- 공통 동작 정의
	- 서로 다른 타입들이 공통적으로 가져야할 메서드, 속성 정의
- 다형성 구현
	- 서로 다른 타입들이 trait에 정의된 공통 동작 구현하도록 지원
- 제네릭 타입 제한
	- 제네릭 타입 매개변수에 trait 제약을 걸어 원하는 동작의 타입만 사용

타입간 추상화 및 다형성을 구현하는 수단