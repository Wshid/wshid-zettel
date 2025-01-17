---
date: 2025-01-17
datetime: 2025-01-17 22:37:53
book: 
page: 
tags: 
references:
  - https://www.montecarlodata.com/blog-2025-data-engineering-trends
aliases:
---

## 1. We’re living in a world without reason (Tomasz)
- AI의 현재 상태
	- Prediction: complete a sentence, correct code error
	- Search: 질문에 답하기 위한 데이터 집합 활용
	- Reasoning: 추론. 복잡한 작업을 완료할 수 있는 workflow
- Reasoning(추론)의 경우 여전히 뒤처진 것으로 보임
	- 모델 정확도(model accuracy) 때문
- 현재 모델은 특정 패턴을 여러 번 겪어 본 적이 없다면
	- 작업 효과적으로 구분이 어려움
- So for now, it looks like its **AI copilots and partially accurate search** results for the win.

## 2. Process > Tooling (Barr)
- 새로운 툴은 딱 그 처리만 하는데 도움이 됨
- modern data stack을 발전 시키는 동안
	- 데이터 팀은 진퇴양난에 빠질때가 많음
	- 플랫폼은 '무엇'에만 지나치게 집중한 나머지
	- '어떻게'에 대해서는 충분한 주의를 기울이지 않기 때문
- enterprise 환경이 점점 production-ready AI에 가까워지면서
	- 이 모든 새로운 툴을 어떻게 운영할지 파악하는 것이 시급함
- 데이터 품질의 예시
	- data reader들은 실제 프로덕션에서 사용할 수 있는 AI의 가능성을 마주하고는 있지만
	- 데이터 품질에서는 DBT test, 포인트 솔루션으로 샘플 채취할 시간이 없음
	- 당장 가치를 내야하는 상황
	- 바로 효과적으로 온보딩하고 배포할 수 있는 신뢰할 수 있는 솔루션 필요
- 시장에서 가장 정교한 데이터 품질 플랫폼(최첨단 자동화, 최고의 copilot, 통합 기능)이 있더라도
	- 제대로 쓰지 못함
- 향후 1년간 데이터 팀은
	- **데이터 품질 소유권, 사고 관리, 장기적인 도메인 지원**과 같은 중요한 과제의 우선순위를 정하기 위해
	- 패치워크 툴킷보다 엔드투엔드 솔루션에 의존할 것으로 보임
	- 이러한 우선순위를 충족하는 솔루션이 AI 분야에서 승리할 것
## 3. AI is driving ROI—but not revenue (Tomasz)
- GenAI의 가치: reducing costs | generating revenue
- 수익을 창출하지 못한다면 AI는 비용을 절감해야 함
- AI 사용 사례는 3가지 중 하나가 충족되면 비용 절감 기회가 있음
	- Repetitive jobs
	- Challenging labor market
	- Urgent hiring needs
- EvenUp 회사 사례
	- 템플릿화 되어 있지만 고도로 전문화된 서비스를 지원하는 조직
	- 현재 형태의 AI를 통해 큰 효과를 볼 수 있는 독보적인 자리일 수 있음