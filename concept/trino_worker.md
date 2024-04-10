---
date: 2024-04-10
datetime: 2024-04-10 16:46:05
book: 
page: 
tags: 
references: 
aliases:
  - Trino Worker
---
- 실제 Task 수행, 데이터 처리 담당
- Coordinator로부터 받은 Task 수행, 데이터 처리
- Coordinator에서 데이터를 가져오고, 서로 중간 데이터를 교환
- Worker로부터 결과를 가져가 Client에게 최종 결과 반환
- process가 시작되면
  - Coordinator의 Discovery server(service)에 자신의 존재를 알림
  - Coordinator가 작업 실행을 위해 worker 노드를 사용할 수 있도록
- Coordinator 및 다른 Worker 노드와 REST API를 통해 통신
  - Test 목적일 경우 Coordinator/Worker가 동일 서버에 위치할 수 있음