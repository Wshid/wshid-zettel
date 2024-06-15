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

---

# CHAP 5. 프롬프트 엔지니어링의 핵심 기법

## 프롬프트를 잘 만드는 방법
- 프롬프트 엔지니어링 & 부하 직원에게 일 잘 시키는 법
	- 구체적이면서 명확하게 과업 지시를 내림
	- 과업의 배경 설명
	- 원하는 결과를 예시할 수 있다면 예시를 들어 설명
	- 과업 수행에 대한 단계별 가이드가 있다면 알려줌
	- 과업 수행시 제약 사항이 있으면 안내
	- 과업 수행 결과를 어떤 형식으로 작성하면 되는지 구체적으로 제시
	- 과업 수행의 주체가 스스로 생각하며 일 할 수 있도록 함

### 프롬프트 작성 예시
```python
template = """
당신은 번역 함수이며, 반환값은 반드시 JSON 데이터 이어야 함
STEP별로 작업을 수행하면서 그 결과를 아래 출력 결과 JSON 포맷에 작성하세요
STEP-1. 아래 세 개의 백틱으로 구분된 텍스트를 원문 그대로 읽어올 것
STEP-2. 입력받은 텍스트가 영어가 아니라면 false를 표기하고 STEP-3를 진행하지 말 것
STEP-3. 다음의 말투로 번역할 것:["지구의 나이는 45억 살이야.", "세종대왕은 조선의 위대한 국왕이야."]
\`\`\`{text}\`\`\`

---
출력 결과: {{"STEP-1": <입력 텍스트>, "STEP-2": <true/false>, "STEP-3": <번역결과>}}
"""

text="William Shakespeare was an English playwright, poet and actor. He is widely regarded as the greatest writer in the English language and the world's pre-eminent dramatist."
template = template.format(text=text)
context = [{"role": "user", "content": template}]
response = client.chat.completions.create(
										  model="gpt-4-0613",
										  message=context,
										  temperature=0,
										  top_p=0,
										  seed=1234
).model_dump()

pprint(json.loads(response['choices'][0]['message']['content']))
"""
```
- 위 예시는 설명 편의를 위해 여러 형태의 요구사항을 한번에 요청함
	- 여러 형태의 요구사항 요구는 흔하지 않음
	- 하나의 프롬프트 안에 너무 많은 것을 요구하면, 원하는대로 동작하지 않을 가능성이 큼
- `seed`는 모델 출력의 재현성을 높이는 매개변수(Integer)
- 동일한 `seed`설정시 일관된 결과를 생성할 확률이 높아짐(보장되는 값은 x)
- 이 매개변수는 `response_format`과 함께 `2023.11.06`이후 출시된 모델 부터 적용됨

### 프롬프트 작성 예시에 적용된 개념
- 과업 배경(컨텍스트): 당신은 번역 함수
- 원하는 결과 예시 제공: `template.STEP-3`의 값
- 단계별 가이드: STEP-1, STEP-2, STEP-3
- 제약 사항: 입력값이 영어가 아니라면 더 이상 진행하지 말 것
- 수행 결과 작성 양식: `출력 결과`에 담긴 값
	- 결론을 분명히 구분해 냄
	- 응답과 이후 프로세스 사이의 연동을 쉽게 하려는 목적
- 적절한 구분자의 사용: `{text}, ---`
- 구체적이며 명확한 과업 지시: 이상의 모든 것
- 스스로 생각하기: 미적용
	- 차후 In-context learning과 함께 설명 예정

### 구분자 활묭 방법
- backtick(\`)
	- 단락의 구분
	- `...`
- xml tag
	- 내용물의 구분
	- `<키워드>...</키워드>`
- json
	- 내용물의 구분
	- `{"이름": "김민지", "나이": 26}`
- css 최우선 순위 키워드
	- 중요한 사항임을 강조
	- `[!important]`
- 일반 문서에서 사용하는 대시, 중괄호, 대괄호, 콜론도 모두 인식하기 때문에, 일부러 프로그래밍 언어의 구분자 사용 필요 x
	- 단, 구분자는 일관되게 사용해야 함

### seed 활용
- [[LLM#Temperature, Top P 활용]]의 **확률적 샘플링**에 연관
- 난수 생성의 시작점 변수
	- 확률적 샘플링을 일관되게 재현할 수 있도록 함
- seed 사용시, 동일한 조건이 주어졌을 때 동일한 결과를 생성할 가능성이 높아짐