새로운 DAG 구조가 필요함
단순히 model과의 의존성이 아닌
model X partition 의존성
e.g. hourly 파티션과 daily 파티션의 의존 관계
- hourly의 24개의 파티션은 daily 파티션 1개와 연관이 있음

DAG로 그리게 되면 24개의 ephemeral과 같은 내용이 작성되어야 함
- gak_log_client_00
- gak_log_client_01