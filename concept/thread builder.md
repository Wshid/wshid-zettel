---
date: 2024-04-14
datetime: 2024-04-14 13:33:43
book: 
page: 
tags: 
references: 
aliases:
---
- [[spawn|std::thread::spawn]]
- `std::thread::Builder`, 스레드를 만들기 전 여러가지 설정 가능
	- e.g. stack memory size, thread name(`std::thread::current()::name())
	- thread name은 panic message에 포함(모니터링 도구, 디버깅 도구에서 확인 가능)
- thread 생성이 실패할 수 있기 때문에 `Builder::spawn`은 `std::io::Result`를 리턴
	- 이유?
		- os의 memory 부족
		- program의 메모리와 같은 리소스 할당 제한
	- 실패시 `std::thread::spawn`은 [[panic]]