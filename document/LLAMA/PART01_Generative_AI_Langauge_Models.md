## [[NLP]]

## [[Seq2Seq]]
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

## Transformer Attention
- 2017년에 등장 모델
- [[Seq2Seq#Bahdanau Attention|바다나우 어텐션]]과 기능적으로는 유사하면서 내부 구조는 다름
	- seq2seq에서 가중치 개선
	- Input Text에 존재하는 token들의 중요도를 가지겠다는 목적은 동일
	- 단, 바다나우 어텐션은 LSTM의 hidden layer에 붙여 가중지를 붙여주는 역할을 했었음
- Dot Product Attention
	- 주가 되는 모델에 붙어서 부가적인 역할을 하지 않음
	- 오히려 모델의 주가 됨
- <img width="347" alt="image" src="https://github.com/user-attachments/assets/15b57f89-0fc5-42f2-b388-f29f3276d2af">
	- 왼쪽은 encoder, 실제로는 여러개가 쌓임
		- Positional Encoding -> 위치 정보를 나타내는 벡터
		- 입력벡터가 인코더를 지나 인코딩이 됨
			- 입력 시퀀스의 의미를 나타낼 수 있는 값
	- 오른쪽은 Decoder
		- 디코더의 입력은, encoder를 통해 만들어진 맥락 정보를 통해 새로운 토큰 생성
		- 맥락 정보와 이전에 만들어 둔 토큰 정보를 활용
		- shifted right
			- 하나 이전의 출력 값을 참고해야 하기 때문에 `i-1`번째를 참고함
			- 한번 오른쪽으로 shift 한 값
- Positional Encoding, Attention
	- 위 Chart를 단순화 한 모습
	- <img width="953" alt="image" src="https://github.com/user-attachments/assets/b9a6d47d-c28e-441b-9e52-0e28088d6796">
	- 입력 값이 encoder, decoder를 거쳐 출력 값으로 변환되는 과정
	- 실제로는 오른쪽 처럼 여러 encoder/decoder가 쌓여있는 형태
		- 쌓여있음: 이전 인코더 블록 출력 = 다음 인코더 블록의 입력
	- 1번 encoder, 2번 encoder이 학습이 진행됨에 따라 블록의 가중치는 다를 수 있음

## Positional Encoding
- [[Tokenization]] 이후 Gen Embedding
	- token으로 다 쪼개진 이후
	- 각각 vector로 변경됨
- Transformer의 Encoder는 Attention을 활용함
	- Attention을 활용한 인코더는 입력값을 **순차적으로 받지 x**
	- 이전에는 자연스럽게 반영될 수 있었던 단어의 순서 정보가 더 이상 반영 x
- 순서는 매우 중요
	- 같은 토큰들의 조합이어도, 어떤 순서로 나열하느냐에 따라 전혀 다른 의미를 가질 수 있음
- 그럼 순서를 어떻게 반영할 수 있을 것인가?
	- 어텐션에 위치 정보를 줌
	- 위치 정보를 어떻게 인코딩 하면 좋을지?
- Ideal Positional Encoder
	- 위치에 따라 벡터의 크기가 달라지면 안됨
	- 위치가 가까울 수록 인코딩 결과가 가깝고, 멀면 멀어야 함
	- 삼각함수를 활용한 인코딩을 통해 구현 가능

## Dot Product Attention
- Dot Product: 내적
- Query: 무엇을 찾고 있는가
- Key: 데이터에 대한 정보를 담으면서, query와 대조할 수 있는 값
	- 책의 제목
- Value: 실제 활용하고자 하는 데이터
	- 책의 내용
- e.g. 책으로 가득한 방에서 어떤 책이 내가 찾고자 하는 내용을 가지고 있는가
- Query, Key, Value 각각의 매트릭스를 하나씩 할당
- 각 단어의 Q, K, V는 들어온 임베딩을 매트릭스에 곱해서 사용 가능
- 임베딩이
	- w, q 행렬에 곱해지면 쿼리
	- w, k 행렬에 곱해진다면 키
	- w, v 행렬에 곱해진다면 벨류
- 값이 구해졌을 경우, dot product 값을 구해서, 쿼리와 각 아이템의 연관성을 구하면 됨
	- 학습이 진행되면서 값이 업데이트 됨
- <img width="748" alt="image" src="https://github.com/user-attachments/assets/510235aa-4849-4393-84e1-afeec5ed3d95">
	- 오른쪽은 attention score를 의미
		- e.g. 지시 대명사가 어떤 사람을 지칭하는지 확인하는 것과 유사

## Self-Attention
- 인코더에서도 attention을 활용함
- 인코딩하는 토큰들만을 가지고 attention을 사용
- **주어진 텍스트의 토큰들 사이에서 어텐션을 적용하는 것**

## Multi-headed Attention
- 여러 어텐션으로 리스크를 줄이는 방법
- Transformer는 실제 한번에 여러 attention을 사용함
- 하나의 토큰의 임베딩을 구할 때
	- 하나의 attention을 사용하는 것이 아닌
	- 더 작은 여러개의 어텐션을 계산하여 concat하여 사용
- <img width="785" alt="image" src="https://github.com/user-attachments/assets/da2427ed-8d28-410f-83c5-9f2ff1dc5a3a">


## Add & Normalization
- 원래 널리 사용되는 기법
- Add: Residual Connection
	- [[vanishing gradient]] 방지
	- 맨 처음 인코더에 입력된 입력값을 한번 더 더해주는 방법
- Normalization
	- 서로 다른 스케일의 데이터를 분포는 유지한채로 공통적인 스케일로 변환
	- 학습과정이 안정화됨
	- [[vanishing gradient]], [[exploding gradient]] 방지

## Pointwise FFNN
- FeedForward Network
- 각각의 단어에 대해 FeedForward Network가 point-wise로 적용됨
	- 각각의 단어 안에서 조정
	- 독립적, 병렬화에 도움
- 인코더 블록 통과 이후
	- 단어 임베딩들의 값은 변하게 되나
	- 포지션은 고정이 됨
- 각 단어에 대한 고유한 벡터 존재
	- 바로 다음 인코더 블록으로 들어갈 수 있음

## Decoder, Masked Attention
- [[#Self-Attention]]이라는 점은 앞에서 확인했던 [[#Multi-headed Attention]]과 동일
- 디코더에서 생성하는 토큰들을 가지고 [[#Self-Attention]]을 한번 취하게 됨
- 현재 시점상 아직 생성되지 않은 시점의 토큰은 masking을 함
	- `-unlimited` 값을 부여하여 이 토큰들을 참조할 수 없도록 함
