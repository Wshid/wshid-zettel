---
date: 2024-05-19
datetime: 2024-05-19 12:24:05
book: 
page: 
tags: 
references:
  - https://netflix.github.io/chaosmonkey/
  - https://github.com/Netflix/chaosmonkey
  - https://effortguy.tistory.com/16
  - https://www.itworld.co.kr/news/152680
aliases:
---
## Chaos Monkey
- Netflix에서 사용하는 기술 스택 중 하나
- Production환경에서 인스턴스를 무작위로 종료하여 **서비스의 복원력**을 테스트 하는 도구
- Spinnaker를 통해 Chaos Monkey의 동작 구성 가능
	- 지정된 지침 사용 배포

## 개발 배경
- Netflix, 2010년 개발
- 초기 클라우드 아키텍처에서 다양한 장애를 시뮬레이션 하기 위한 **Simian Army**의 일부
- Choas Engineering 개념 발전
	- 시스템의 정상 상태를 정의하고, 가설 테스트
	- 실제 세계의 변수 도입, 대조군과 실험군 간의 차이 식별
- 위 접근 방식은 소프트웨어 업계에서 널리 채택

## GitHub repo
- Netflix/chaosmonkey
- 최근 Golang의 semantic import versioning을 준수하기 위해 `/v2`를 패키지에 추가하는 것이 포함됨

## Choas Monkey for Spring Boot(CM4SB)
- Netfix에서 개발한 도구
- Spring Boot app에 대한 Chaos Engineering 기능 도입
- CM4SB는 Chaos Engineering 개념 설명 및 Spring boot app에 적용 할 수 있는 방법 설명
- 공격 대상: `@Controller, @Repository, @Service, @RestController, @Component`
- 감시자: Watcher(공격 대상별 존재)
- 공격 종류: Assault
	- Latency Assault: 응답 지연
	- Exception Assault: 예외 발생
	- AppKiller Assault: 어플리케이션 종료
	- Memory Assault: 메모리 누수
```bash
# 설정 정보 조회
GET localhost:8080/actuator/chaosmonkey

# 활성화
POST localhost:8080/actuator/chaosmonkey/enable

# watcher 활성화
POST localhost:8080/actuator/chaosmonkey/watchers&restController=true

# latencyActive 활성화
POST localhost:8080/actuator/chaosmonkey/assaults&latencyActive=true

# 응답 지연 발생, 33% 확률로 1~3초 응답 지연
POST localhost:8080/actuator/chaosmonkey/assaults&latencyRangeStart=1000&latencyRangeEnd=3000&level=3

```