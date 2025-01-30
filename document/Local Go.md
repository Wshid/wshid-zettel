- 로컬 환경에서 Go 띄우기
- Docker 환경에서 구동한다

```bash
docker pull golang:latest


docker run -it -v /Users/sion/Workspace/local-go:/app -w /app -p 8080:8080 --name go-container golang:latest /bin/bash


# in container
go mod tidy
go mod init example.com/myapp
```


### docker-compose up 에러시
- `zsh` 상에서 찾지 못하던 상황
```bash
sudo ln -sf /Applications/Docker.app/Contents/Resources/cli-plugins/docker-compose /usr/local/bin/docker-compose
```


#### main.go
```go
package main

import (
    "fmt"
    "net/http"
)

func handler(w http.ResponseWriter, r *http.Request) {
    fmt.Fprintf(w, "<h1>Hello from Go inside Docker!</h1>")
}

func main() {
    http.HandleFunc("/", handler)
    fmt.Println("Server running on port 8080...")
    http.ListenAndServe(":8080", nil)
}
```

### go run vs go build
- go run
	- 빌드 없이 바로 수행
	- 개발간에 사용
	- `go run main.go`
- go build
	- 컴파일 후 바이너리 파일 생성
	- 운영 배포시 사용함.
	- `go build -o myapp main.go`