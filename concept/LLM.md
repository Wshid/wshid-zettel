---
date: 2024-06-08
datetime: 2024-06-08 08:49:58
book: 챗GPT_API를_활용한_챗봇_만들기
page: 
tags: 
references: 
aliases:
---

# Playground
- https://platform.openai.com/playground/chat?models=gpt-3.5-turbo 
### Temperature
- 언어모델이 얼마나 창의적으로 말할 것인가
- 0: 일관적으로 대답
- 2: 매번 창의적

### Top P
- 다양성의 변수, Temperature와 유사
- Temperature는
	- 선택한 단어들에 대한 확률분포 모양새와 연관
	- 1보다 작으면 뾰족하게, 1보다 크면 펑퍼짐하게
- Top P의 경우
	- 다음 단어 선택시 후보군 중에 확률이 가장 높은 단어만 선택
	- 확률이 낮은 단어는 버림

### Temperature, Top P 활용
- 단어들에 대한 최종값 산출 후
- 후처리 진행
	- Temperature에 따라 각 후속 단어를 평탄하게 만듦
	- Top P에 의해 후보군 중 일부 단어 유지]
	- 잔여 후보군 중에서 확률적 샘플링('불균형한 동전 던지기')

### Maximum length
- 인공지능이 한번에 말하는 대화의 크기
- [[Token]]의 수

### Frequency Penalty, Presense Penalty
- 동일한 단어가 등장할 때 벌점을 부여하는 변수
- 최대 2까지 실정 가능
- Frequency Penalty
	- 동일한 단어의 등장 빈도에 비례하여 벌점 부여
	- 문장이 단조롭게 생성되는 것을 방지
- Presense Penalty
	- 동일한 단어가 등장했는지 여부에 따라 벌점 부여
	- 화제를 새롭게 전환하도록 유도
- 두 변수를 음수로 설정하면 정반대로 작동
	- 같은 단어가 자주 등장
- 보통은 두 변수 모두 기본값을 사용하며,
	- 임의의 값 부여시 API 문서 권장 값인 `0.1 ~ 1`를 추천

### 정리
- 매개변수는 일반적으로 `Temperature`만 조정하여 실험하는 것이 일반적
- 매개변수마다 값을 바꾸게 되면, 발생하는 경우의 수가 너무 많아지기 때문
- 효율성을 고려한다면,
	- 다양한 매개변수 조정보다, 프롬프트 작성에 노력을 기울일 것 