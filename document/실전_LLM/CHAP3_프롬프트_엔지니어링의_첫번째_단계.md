---
date: 2024-06-13
datetime: 2024-06-13 21:26:43
book: 실전_LLM
page:
  - "91"
tags: 
references: 
aliases:
---
- 사용자 경험을 향상 시키기 위해, 이를 분석하려면 어떻게 해야할까?
- LLM의 학습 능력을 활용하여 전체 과정을 완성하고
	- 유용한 end-to-end LLM 기반 app을 생성하고자 할 수 있음
	- 이때 prompt-engineering을 사용

# 3.2. 프롬프트 엔지니어링
- 프롬프트 엔지니어링은
	- 효과적으로 작업을 전달하여 정확하고 유용한 출력을 반환하도록 유도하는
	- LLM에 대한 입력(프롬프트)를 만드는 것

## 3.2.1. 언어 모델에서 정렬
- 언어 모델이 어떻게 사람의 입력에 **정렬**되는 지를 알아야 함
- 정렬(Alignment)
	- 모델이 사용자가 예상한 것과 '일치하는' 방식으로 입력 프롬프트를 이해하고 답변하는 것
- 표준언어 모델링은 선형 단어의 맥락을 기반으로
	- 다음 단어나, 단어의 시퀀스를 예측하도록 훈련됨
- 하지만 이 방식으로는 모델이 **특정 지시사항**이나 **프롬프트**에 완벽히 답변 불가
	- 특정 어플리케이션에 대해서는 유용하지 않을 수 있음
- 프롬프트가 언어 모델과 align되지 않는다면 잘못 답변 가능
- 몇몇 언어 모델은 추가적인 정렬 기능과 함께 개발됨
	- Anthropic의 AI 피드백 기반 강화 학습(RLAIF; Constitutional AI-driven Reinforcement Learning from AI feedback)
	- OpenAI의 GPT 계열에서의 인간 피드백 강화 학습(RLHF; Reinforcement Learning from Human Feedback)
	- 명확한 지시사항과 피드백을 모델 훈련에 통합 가능
- 정렬 기술은 특정 프롬프트를 **이해하고 답변하는 모델의 능력 향상**으로
	- 질문-답변이나 언어 번역과 같은 app을 유용하게 만들 수 있음
- 아래 모델들은 대량의 데이터와 전이학습, 파인튜닝 등의 기술을 사용하여 명령어 프롬프트에 대한 답변을 보다 효과적으로 생성 가능
	- ChatGPT: OpenAI closed source model
	- FLAN-T5: Google open source
	- Cohere 명령어 계열: closed source model