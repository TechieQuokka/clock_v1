# clock_v1

iPhone 시계 앱의 4가지 핵심 기능을 구현한 Rust CLI 툴입니다.

## 기능

| 서브커맨드 | 설명 |
|---|---|
| `clock time` | 현재 시각 실시간 표시 |
| `clock alarm <HH:MM>` | 지정 시각까지 대기 후 벨 알림 |
| `clock stopwatch` | 인터랙티브 스톱워치 (랩 기능 포함) |
| `clock timer <duration>` | 카운트다운 타이머 |

## 설치

```bash
cargo build --release
```

빌드 후 `target/release/clock` 바이너리를 PATH에 추가하거나 직접 실행하세요.

## 사용법

### time

현재 시각을 실시간으로 표시합니다. `Q` / `Esc` / `Ctrl+C`로 종료합니다.

```bash
clock time
```

### alarm

24시간 형식(`HH:MM`)으로 알람 시각을 지정합니다. 자정을 넘기는 경우도 처리됩니다.

```bash
clock alarm 07:30
clock alarm 22:00
```

### stopwatch

| 키 | 동작 |
|---|---|
| `Space` | 시작 / 일시정지 |
| `L` | 랩 기록 |
| `R` | 리셋 |
| `Q` / `Esc` / `Ctrl+C` | 종료 |

```bash
clock stopwatch
```

```
  Lap 1    00:03.512
  Lap 2    00:07.891
  ────────────────────
  00:11.234  [RUNNING]
  Space=start/pause  L=lap  R=reset  Q=quit
```

### timer

`h` (시간) / `m` (분) / `s` (초) 단위를 조합해서 입력합니다. `Q`로 조기 종료, 완료 시 벨이 울립니다.

```bash
clock timer 5m
clock timer 1h30m
clock timer 90s
clock timer 1h30m45s
```

## 의존성

- [`clap`](https://github.com/clap-rs/clap) — CLI 파싱
- [`chrono`](https://github.com/chronotope/chrono) — 시간 처리
- [`crossterm`](https://github.com/crossterm-rs/crossterm) — 터미널 제어 / 키 입력
