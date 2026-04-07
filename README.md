# Custom Alias

크로스플랫폼 쉘 alias 관리 데스크톱 앱

A cross-platform desktop app for managing shell aliases with a GUI.

---

## 한국어

### 소개

Custom Alias는 macOS와 Windows에서 쉘 alias를 GUI로 쉽게 등록, 수정, 삭제할 수 있는 데스크톱 앱입니다. 터미널에서 직접 설정 파일을 편집하지 않아도 됩니다.

### 지원 쉘

- **Bash** - `.bashrc`, `.bash_profile`, `.bash_aliases`
- **Zsh** - `.zshrc`
- **Fish** - `config.fish`, `conf.d/*.fish`
- **PowerShell** - `profile.ps1`

### 주요 기능

- **자동 쉘 감지** - 설치된 쉘을 자동으로 찾아 설정 파일 경로를 탐지
- **이중 탐지 전략** - 설정 파일 파싱 + 런타임 조회(`bash -ic 'alias'` 등)를 결합하여 모든 alias 수집
- **Managed Section** - 앱이 관리하는 영역을 설정 파일 안에 명확히 구분하여 안전하게 CRUD
- **자동 백업** - 설정 파일 수정 전 자동 백업 생성 (최대 10개 유지)
- **플러그인 alias 필터링** - oh-my-zsh 등 플러그인에서 오는 alias는 기본 숨김, 토글로 표시 가능

### 기술 스택

- **백엔드**: Rust + Tauri v2
- **프론트엔드**: React + TypeScript + Vite
- **스타일**: 다크 테마, 터미널 미학 디자인

### 실행 방법

```bash
# 의존성 설치
pnpm install

# 개발 모드 실행
pnpm tauri dev

# 프로덕션 빌드
pnpm tauri build
```

### 프로젝트 구조

```
src/                    # React 프론트엔드
  components/           # UI 컴포넌트
  hooks/                # React 훅 (useShells, useAliases)
  lib/                  # 타입 정의, Tauri invoke 래퍼
  pages/                # 페이지 컴포넌트

src-tauri/src/          # Rust 백엔드
  shell_detect.rs       # 쉘 감지
  alias_parser.rs       # 설정 파일 파싱
  alias_runtime.rs      # 런타임 alias 조회
  alias_merger.rs       # 파싱 + 런타임 결과 병합
  alias_writer.rs       # alias CRUD (managed section)
  backup.rs             # 자동 백업
  config_paths.rs       # 크로스플랫폼 경로 해결
```

---

## English

### Introduction

Custom Alias is a desktop application for managing shell aliases through a GUI on macOS and Windows. No more manually editing config files in the terminal.

### Supported Shells

- **Bash** - `.bashrc`, `.bash_profile`, `.bash_aliases`
- **Zsh** - `.zshrc`
- **Fish** - `config.fish`, `conf.d/*.fish`
- **PowerShell** - `profile.ps1`

### Key Features

- **Auto Shell Detection** - Automatically discovers installed shells and their config file paths
- **Dual Detection Strategy** - Combines config file parsing with runtime queries (`bash -ic 'alias'`, etc.) to collect all aliases
- **Managed Section** - Maintains a clearly marked block in config files for safe CRUD operations
- **Auto Backup** - Creates backups before every config file modification (keeps last 10)
- **Plugin Alias Filtering** - Aliases from plugins (oh-my-zsh, etc.) are hidden by default, toggleable

### Tech Stack

- **Backend**: Rust + Tauri v2
- **Frontend**: React + TypeScript + Vite
- **Design**: Dark theme with terminal-inspired aesthetics

### Getting Started

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Project Structure

```
src/                    # React frontend
  components/           # UI components
  hooks/                # React hooks (useShells, useAliases)
  lib/                  # Type definitions, Tauri invoke wrappers
  pages/                # Page components

src-tauri/src/          # Rust backend
  shell_detect.rs       # Shell detection
  alias_parser.rs       # Config file parsing
  alias_runtime.rs      # Runtime alias queries
  alias_merger.rs       # Merge parsed + runtime results
  alias_writer.rs       # Alias CRUD (managed section)
  backup.rs             # Auto backup
  config_paths.rs       # Cross-platform path resolution
```

### How the Managed Section Works

When you add an alias through the app, it writes to a clearly marked block in your shell config:

```bash
# >>> custom-alias managed >>>
# group: git
alias gs='git status'
alias gp='git push'
# <<< custom-alias managed <<<
```

Aliases outside this block are shown as read-only. You can import them into the managed section via the app.

---

## License

MIT
