## NLP
- Language Model
	- 나열된 단어들의 조합에 확률을 부여하는 모델
	- 어떤 패턴이 더 그럴듯 한가?
- 자연어 문제 두가지
	- Natural Language Understanding(NLU): 자연어 이해
		- e.g. 이 내용이 긍정적인가? 아닌가?
	- Natural Language Generation(NLG): 자연어 생성
		- e.g. 주제에 맞는 그림 그리기
- NLP는 NLU와 NLG를 포함함
	- NLU, NLG의 교집합 존재
	- NLU + NLG는 NLP보다 작음
- 두 문제는 상호 베타적이지 않다는 의미

## Seq2Seq
- 아래 두가지 요소로 이루어짐
- Encoder: 주로 자연어 이해
- Decoder: 주로 자연어 생성
- 역할
	- Encoder는 `Take your time`이라는 input을 이해하고 decoder에 전달
	- decoder는 이에 맞는 한국어에 맞게 번역하여 리턴
- 소개할 언어 모델은
	- Encoder, Decoder를 모두 가지는 경우, 혹은 하나만 가진 경우도 존재

## 모델 분류

### Encoder, Decoder 존재 여부에 따라
- Encoder만 존재
	- BERT
	- 문장을 분류하는 문제에 존재
- Decoder만 존재
	- GPT, GPT-2, GPT-3, FLAN
	- 생성에 중점을 두었다는 의미
	- 이번 강의에서 중점
- Encoder & Decoder
	- seq2seq, T5, T0, BART
	- T 시리즈

### 사전 학습 여부에 따라
- 사전학습 x
	- seq2seq
- pre-trained: 사전학습 모델들
	- NSP(Next Sentence Prediction) , MLM: BERT
		- NSP: 두개의 문장이 서로 이어지는지를 판단
		- NLM: Masked Language Model, 텍스트 중간에 빈칸을 뚫고, 그 빈칸을 맞추도록
	- denoising: BART, T0, T5
		- 일부러 노이즈를 섞어둔 인풋을 원래대로 교정하게 하는 방식
	- LM: GPT, GPT-2, GPT-3, FLAN
		- Language Model: 맥락이 주어졌을 때 다음 토큰을 예측하게 함
		- 단어의 시퀀스에 대한 확률 부여 모델 
			- 사전 학습을 하게 되면, 어떤 시퀀스가 올바른지 판단이 가능해짐
- 사전학습은 **레이블이 없는 데이터**를 대상으로 함
	- 하지만 아예 없을 때는 학습이 불가능하므로
	- 문제를 자체 생성 및 해결하는 형식으로 training
	- self-supervised learning

## 생성 언어 모델의 등장 순서
- seq2seq
- Transformer
- GPT, BERT
- GPT-2, BART
- GPT-3, T5
	- 자연어 처리 문제의 동향이 바뀜
	- 특정한 하나의 다운스트림 task 처리가 아닌, 파인 튜닝 없이도 일반적으로 잘 작동하도록
- T0, FLAN
- InstGPT, GPT-3.5
- LLaMA

## Downstream task
- 모델의 성능 비교시 사용
- [[Natural Language Inference]]
- [[Reading Comprehension]]
- [[Common Sense]]
- [[Sentiment Analysis]]
- [[Machine Translation]]
- [[Summarization]]
