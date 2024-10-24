
# Fukker

Fukker는 Nx 프로젝트에서 파일 변경 시 자동으로 리로드되지 않는 문제를 해결하기 위해 개발된 도구입니다. Nx 데몬이 작동하지 않아서 파일이 변경되어도 자동 리로드가 되지 않을 때, 이 프로그램을 사용하여 이 문제를 해결할 수 있습니다.

## 사용법

1. 먼저 `config.json` 파일을 생성한 후, 다음과 같은 설정을 추가합니다:

```json
{
    "watch_dir" : ["/workspace/nx-nestjs-template/apps", "/workspace/nx-nestjs-template/libs"],
    "watch_ext" : ["ts"],
    "working_dir" : "/workspace/nx-nestjs-template",
    "cmd" : "yarn dev",
    "server_port" : [3000],
    "refresh_interval" : 100,
    "watch_buffer_size" : 1024
}
```

- `watch_dir`: 감시할 디렉토리 경로 배열입니다.
- `watch_ext`: 감시할 파일 확장자입니다 (예: `"ts"`).
- `working_dir`: 작업 디렉토리입니다.
- `cmd`: 파일 변경 시 실행할 명령어입니다.
- `server_port`: 서버가 사용하는 포트 배열입니다.
- `refresh_interval`: 파일 변경을 감지하는 주기입니다 (단위: ms).
- `watch_buffer_size`: 감시할 때 사용하는 버퍼 크기입니다.

2. 설정을 완료한 후 프로그램을 실행하면, 파일이 변경될 때마다 자동으로 서버가 리로드됩니다.

## OS
- [ ] Windows
- [ ] macOS
- [x] Linux

## 설치

```bash
git clone https://github.com/your-repo/fukker.git
cd fukker
cargo run
```

---

# Fukker

Fukker is a tool developed to solve the issue of files not automatically reloading in Nx projects when changes are made. If the Nx daemon is not running and changes aren't being detected for automatic reload, this program provides a solution.

## How to Use

1. First, create a `config.json` file with the following configuration:

```json
{
    "watch_dir" : ["/workspace/nx-nestjs-template/apps", "/workspace/nx-nestjs-template/libs"],
    "watch_ext" : ["ts"],
    "working_dir" : "/workspace/nx-nestjs-template",
    "cmd" : "yarn dev",
    "server_port" : [3000],
    "refresh_interval" : 100,
    "watch_buffer_size" : 1024
}
```

- `watch_dir`: Array of directory paths to watch.
- `watch_ext`: File extensions to monitor (e.g., `"ts"`).
- `working_dir`: Working directory path.
- `cmd`: Command to run when files are changed.
- `server_port`: Array of server ports in use.
- `refresh_interval`: Interval to check for file changes (in ms).
- `watch_buffer_size`: Buffer size for watching files.

2. After configuring, run the program to automatically reload the server whenever a file change is detected.

## OS
- [ ] Windows
- [ ] macOS
- [x] Linux

## Installation

```bash
git clone https://github.com/your-repo/fukker.git
cd fukker
cargo run
```
