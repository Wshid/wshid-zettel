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

## 4. AI adoption is slower than expected - but leaders are biding their time (Tomasz)
- 작년, 모든 종류의 sw를 시도해 보고 싶어 했었으나 현재는 그렇지 않음
- 일부 조직은 초기 실험에서 가치를 보지 못했음
- 다른 족직은 빠른 진화를 어려움에 겪음
- 기술이 이론적으로 가치가 없다기 보단
	- 조직이 실제 효과적인 활용 방법을 알아내지 못했다는 의미
- 하지만 점차 각 팀별 어떤 것을 원하는지 알아가고 있기 때문에 적절히 기회만 온다면 행동할 준비가 되어 있음
	- Like the dress rehearsal before the big show,
	- legal, procurement - particularly data loss and prevention
## 5. Small data is the future of AI (Tomasz)
- open source vs manage는 꽤나 오래된 논쟁(debate)
	- AI에서는 더 복잡한 문제
	- 제어나 상호 운영성(interoperability)가 아닌 operational cost가 문제
- 대형 B2C 기업의 경우 기성 모델을 사용할 것으로 예상되나,
	- B2B는 자체적인 오픈 소스 모델을 사용하는 추세로 바뀔 것
	- B2B에서는 전반적으로 소규모 모델과 오픈소스가 많이 사용됨
		- 소규모 오픈소스 모델을 운영하는 것이 더 저렴하기 때문
- But it's not all dollars and cents
	- 모든 것이 다 좋은 것은 아님
- Small models also improve performance
	- 대규모 모델은 다양한 사용 사례에 서비스를 제공하도록 설계됨
	- 사용자는 대규모 모델에 사실상 모든 것에 대해 질문 가능
	- 적절한 답변을 제공하기 위한 충분한 데이터로 학습해야 함
	- 모델이 더 많은 주제에 대해 학습할 수록 여러 개념(concept)을 혼동할 가능성이 높아지며
		- 시간이 지날 수록 오류가 발생할 가능성이 높아짐
- 80억 개의 매개변수가 있는 llama 2를
	- 10k의 지원 타켓으로 미세 조정하면 훨씬 좋은 성능을 발휘할 수 있음
- 규제가 심한 산업에서 독점 모델(proprietary models)의 장기적인 채택에 영향을 미칠 가능성이 높으나
	- 그 영향의 심각성은 아직 불확실함
- 물론 독점 모델들도 가만히 있는 것은 아님(lying down either)
	- 독점 모델들은 이미 수요를 촉진하기 위해 공격적으로 가격을 인하하고 있음
	- chatGPT 같은 모델은 이미 가격을 50% 인하
- 비용 경쟁은 AI rams race에서 경쟁하고자 하는 B2C 기업에게 매우 필요한 혜택일 수 있음

## 6. The lines are blurring for analysts and data engineers (Barr)
- 분석가와 엔지니어의 경계가 모호해지다
- 파이프라인 생산 확장시 두개의 문제가 존재
	- 기술 경험이 부족한 분석가
	- 시간이 부족한 데이터 엔지니어
- 데이터 팀이 어떻게 진화할 것인가, 2025년에 엔지니어링과 분석 책임의 통합을 촉진할 수 있는 두 가지 주요 발전이 있음
	- Increased demand
		- 데이터 팀은 더 적은 자원으로 더 많은 일을 해야할 것
		- 병목 현상 최소화를 위해
		- 리더는 자연스럽게 전문화된 팀이, 파이프라인과 이해관계자에게 더 많은 책임을 맡을 수 있도록 해야 함
	- Improvements in automation(자동화의 개선)
		- 새로운 수요는 항상 혁신을 주도함
			- in this case, that means AI-enabled pipelines
		- 기술이 자연스럽게 자동화됨에 따라
			- 엔지니어는 더 적은 인력으로 더 많은 일을 할 수 있게 됨
			- 분석가는 스스로 더 많은 일을 함
- 수요가 증가함 -> 파이프라인의 자동화의 발전
	- 파이프라인을 만들고 관리하는 데 있어 장벽은 낮아짐
	- 기술 격차는 줄어들고 새로운 가치 창출의 증가
- The move toward self-serve AI-enabled pipeline management
	- 모든 사람의 업무에서 가장 힘든 부분이 자동화
	- 그 과정에서 새로운 가치를 창출하고 입증할 수 있는 능력의 확장

## 7. Synthetic data matters—but it comes at a cost (Tomasz)
- 합성 데이터의 중요성
- a snake eating its own tail
	- 자신의 꼬리를 잡아먹는 뱀
	- contemporary AI와 닮아 있음(resemble)
- 데이터가 계속 발전하기 위해 학습할 데이터의 양이 무한정 늘어나야 함
	- 데이터가 많을 수록 출력에 사용할 수 있는 context가 많아지며, 출력의 정확도도 높아짐
- 학습 데이터를 만들기 위해, 그 데이터를 자체 생산해야 함
	- OpenAI와 같은 기업을은 synthetic data가 향후 모델 학습 방법의 중요한 부분이라고 판단함
- synthetic data는 모델을 활용하여 실제로 더 많은 데이터가 존재하는 대체 현실에서
	- 누군가가 유기적으로 찾을 수 있는 것을 반영하는 인공 데이터 세트 생성
	- 그 새롱누 데이터를 사용하여 자체 모델 학습
	- 소규모로 보면 이 것은 실제적으로 많은 의미가 있음. 다다익선
- contextual malnutrition(상황별 영양 실조)
	- 신선한 유기농 원본: 모델 학습에 가장 영양가가 높은 데이터
	- 기존 데이터 세트에서 추출한 데이터: 본질적으로 이전에 나온 데이터보다 영양가가 떨어짐
- 약간의 인공적인 맛(artificial flavor)은 괜찮으나
	- 새로운 데이터가 도입되지 않고 합성 데이터가 계속 된다면
	- 그 모델은 결국 실패하거나 덜 매력적인 nail beds를 가지게 될 것
- It's not really a matter of if, but when
	- 만약의 문제가 아닌 언제의 문제
- 현재 모델 붕괘(model collapse)는 아직 멀었으나,
	- AI 연구가 모델의 기능적 한계까지 밀어 붙이고 있기 대문에
	- 조만한 AI 기능적 정체(functional plateau)로 도달할 수 있음

## 8. The unstructured data stack will emerge (Barr)
- 비정형 데이터 스택의 출현
- AI시대에서 비정형 데이터는 완전히 새로운 역할
- IDC의 보고서에 따르면 현재 조직의 비정형 데이터 중 절반 정도만 분석에 사용
- 이 동향은 곧 바뀔 것
	- GenAI, 기업의 성공 여부는 학습, 미세조정 및 보강시 사용되는 다양한 비정형 데이터
	- 비정형 데이터에 대한 열정, 급증하는 '비정형 데이터 스택'도 계속 증가할 것
- 일부 팀에서는 추가 LLM을 사용하여 비정형 데이터에 구조 추가를 추가하여
	- 추가적인 교육, 분석 사용 사례에서 유용성 확장
- 조직내의 어떤 unstructured first party data가 있는지,
	- 이해 관계자를 위해 그 데이터를 어떻게 활성화 할 수 있는지
	- 이는 데이터 플랫폼의 비즈니스 가치를 입증하려는 데이터 리더에게 새로운 기회
- 2024: exploring the potential of unstructured data
- 2025: realizing its value
	- 과연 어떤 도구가 rise to the surface 될 것인가

## 9. Agentic AI is great for conversation—but not deployment (Tomasz)
- 대화에는 좋지만 배포에는 적합하지 않은
- copilot: terrible code를 수정하는 멋진 AI 용어
- agents: 정보를 수집하고 이를 사용하여 작업을 수행하는 워크플로
	- terrible code의 블로그 작성 및 wordPress에 게시
- 2024년에 AI copilot는 많은 성공을 거두웠음(Github, Snowflake, MS)
	- AI agent는?
- agentic AI: a fun time wreaking havoc on customer support teams
	- wreaking havoc: 혼란을 일으킴
	- 이 단계에 머물수 있음
	- 중요한 진전이나, 워크플로우의 정확도가 낮음
- 정확도 관점에서
	- 75 ~ 90%의 정확도는 AI의 최첨단 수준
	- Most AI는 high school student와 같음
	- 하지만 75 ~ 90%의 정확도를 가진 3단계의 경우, 최종 정확도는 50%정도
- 대부분의 AI agent의 경우 조직의 수익 창출되기는 커녕
	- 현재 production에 투입될 경우 오히려 해가 될 수 있음
- 데모외에는 아무도 성공하지 못함
- AI agent에 대해 이야기 하고 싶어도 현재 성과로 이어지지는 않음

## 10. Pipelines are expanding—but quality coverage isn’t (Tomasz)
- 일관된 결과물을 얻는 데에는 진정한 품질 문제가 있음
- Monte Carlo survey
	- 매년 데이터 전문가들을 대상으로 데이터 품질 상태에 대한 설문 조사
	- shadow of AI
- Data quality risks are evolving - But data quality management isn't
	- 데이터 품질 위험은 증가하나, 데이터 품질 관리는 진화하지 않음
- vector database를 구축하거나 대규모로 모델을 embedding 하는 팀들이 많음
	- 대규모 SQLLite, 1억개의 소규모 데이터 베이스
	- 이 모든 작은 모델을 실행하기 위해 CDN Layer에서 아키텍처 설계
	- Iphone에는 머신 러닝 모델이 탑재됨
	- 파이프라인 수는 폭발적으로 증가하나, 데이터 양은 훨씬 줄어들 것
- The pattern of fine-tuning
	- 이로 인해 조직내 데이터 파이프라인 수는 증가함
	- 하지만 파이프라인이 늘어날수록 데이터 품질은 더 어려움
- 데이터 품질은 파이프라인 양과 복잡성에 정비례하여 증가
- 파이프라인이 많을수록 문제가 발생할 기회도 많아지도 제때 발견할 가능성이 낮아짐