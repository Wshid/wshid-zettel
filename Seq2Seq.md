- 아래 두가지 요소로 이루어짐
	- Encoder: 주로 자연어 이해
	- Decoder: 주로 자연어 생성
- 역할
	- Encoder는 `Take your time`이라는 input을 이해하고 decoder에 전달
	- decoder는 이에 맞는 한국어에 맞게 번역하여 리턴

## Seq2Seq
- 순차적인 데이터를 입력받아 순차적인 데이터를 출력하는 모델
	- 여기에는 순차적인 자연어 토큰
- encoder -> context vector -> decoder
- 기계 번역에서 많이 사용됨
- 한번에 입력값을 받아들이는 것이 아님
- encoder: 입력 시퀀스에 문맥을 바녕해, context vector를 생성
	- e.g. take, your, time을 개별로 처리함
- decoder: context vector를 seed로 사용해 순차적으로 출력을 생성
	- 입력 시퀀스를 끝났다는 eos 벡터를 input으로 같이 받음
	- 이후 마지막 모든 토큰 생성이 완료되었다는 `eos`토큰이 출력 값으로 나올 때까지 계속됨

## Encoder
- encoder, decoder 모두 `LSTM` cell로 이루어짐
- 두개의 순환 신경망 구조로 이루어진 모델 (Seq2Seq)
- Encoder: LSTM 모델
- Context Vector
	- encoder LSTM의 마지막 time step에서의 hidden state
- LSTM은 각 time step 마다 아래 값을 입력값으로 가짐
	- 이전 타임 스텝에서의 hidden state + 현재 time step
	- 이후 hidden state를 반환
	- 이후 다음 스텝으로 계속 처리
- 마지막 hidden state가 context vector가 됨

## Decoder
- Encoder와 동일하게 LSTM layer로 이루어짐
- 이전 time step에 스스로 생성하는 token을 다시 입력으로 받음
- context vector: 0번째 hidden state로 간주, 첫 번째 decode LSTM의 입력이 됨
- encoder단에서 생성했던 context vector를 마치 0번째 hidden state 처럼 간주하여 사용
	- 첫번째 decoder에서 활용
- eos 토큰이 나올때까지 반복

## 수식
- encoder
$$
\text{입력 시퀀스: }x_{1,}..., x_{n}
$$
$$
\text{출력 시퀀스: }y_{1},..., y_{n}
$$
$$
\text{encoder: } h_{t}= f(x_{t},h_{t-1} )
$$
$$
\text{decoder: } s_{t}= g(s_{t-1},y_{t-1} )
$$
- $h_t$는 시점 $t$에서의 encoder의 hidden state
- $s_t$는 시점 $t$에서의 decoder의 hidden state, $s_{0}=h_{n}$


## Bahdanau Attention
- 모든 단어가 똑같이 중요할까?
	- 문장에는 더 중요하고, 덜 중요한 단어가 있음
- 위 seq2seq에서 context vector를 받는 형태로 변형됨
 $$
\text{decoder: } s_{t}= g(s_{t-1},y_{t-1}, c_t)
$$
- $s_t$는 시점 $t$에서의 decoder의 hidden state, $s_0=h_n$
- $c_{t}= h(h_1,...,h_n)$, 시점 $t$에서의 문맥 벡터
	- 각 시점 인코더들의 hidden state들을 어떤 기준으로 정한 것
	- 가중치를 반영하여 문맥 벡터를 만드는 것
- <img width="508" alt="image" src="https://github.com/user-attachments/assets/a4778145-d763-4ba1-8939-8bbf7784e2f2">
