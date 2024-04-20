### 문제 상황
- dev, stage, main의 브랜치가 존재함
- main 브랜치가 업데이트 되었을 때, dev, stage 브랜치를 overwrite 하는 형태
	- 브랜치를 overwrite하고 dev,stage 환경에서 재빌드
- 이때, 과거 dev, stage에서 checkout한 feature의 경우, 과거 형상의 빌드 파일을 가지게 됨
- 과거 형상의 feature와 overwrite된 dev/stage의 빌드파일에서 conflict가 발생함

### conflict가 발생하는 이유
- 아래 두 형상이 다르기 때문
	- feature의 빌드 파일
	- dev/stage의 빌드 파일
- 빌드 파일 변경 사항을 revert하거나, dev,stage에서는 gitignore를 추가하면 됨?
	- gitignore을 추가할 수 있는 방법이 있나?

### gitignore의 사용
- dev,stage에서는 빌드 파일 유지가 필요하기 때문에 gitignore에서 제외해야 함
- conflict를 해소하려면 빌드 파일 변경을 제외를 해야함

### WAY 1. 특정 디렉터리의 변경사항 git action 추가
- 임의적으로 디렉터리 하위 변경사항이 생길경우 트리거
- 이때, 해당 변경사항 하위 내용을 제거하는 commit/push
- 사용자 입장에서 pull 필요


- 현재 reset 하려는 대상 자체가 models, spark-models가 아닐 경우 이긴 함

```yaml
name: Enforce Main State Except Specific Directories

on:
  pull_request:
    paths-ignore:
      - 'spark-models/**'
      - 'models/**'
    branches:
      - main

jobs:
  enforce-main-except-specific-dirs:
    runs-on: ubuntu-latest
    steps:
    - name: Checkout the base branch (main)
      uses: actions/checkout@v2
      with:
        ref: 'main'
        path: 'main-branch'
    - name: Checkout the head branch (PR branch)
      uses: actions/checkout@v2
      with:
        ref: ${{ github.head_ref }}
        path: 'pr-branch'

    - name: Copy specific directories from PR branch to main branch directory
      run: |
        cp -R main-branch/spark-models/* pr-branch/spark-models/
        cp -R main-branch/models/* pr-branch/models/

    - name: Commit and push changes if there are changes
      run: |
        cd pr-branch
        git config --global user.name 'github-actions'
        git config --global user.email 'github-actions@github.com'
        git add .
        git commit -m "Apply changes from PR except for spark-models and models directories" || echo "No changes to commit"
        git push origin HEAD:${{ github.head_ref }}
```


### WAY 2. 불필요한 빌드 파일 제거
- main브랜치에서 dev,stage의 해당 빌드 결과물은 필요 없던 상황
- main 브랜치상에서 해당 결과물을 모두 제거한다
- 추가적으로 지정한 디렉터리 외 변경사항이 존재할때, 검증 실패를 발생시키면 되지 않을까?