<div align="center">

<img src="https://capsule-render.vercel.app/api?type=venom&height=200&color=gradient&customColorList=12&text=ASTROTOP&fontSize=80&fontColor=fff&animation=twinkling&desc=A%20System%20Dashboard%2C%20Half-Real%20So%20Far&descSize=16&descAlignY=65&stroke=FFFFFF&strokeWidth=1" alt="Astrotop Banner" />

<img src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&size=18&pause=1000&color=00D9FF&center=true&width=900&height=50&lines=Rust+%C2%B7+sysinfo+%C2%B7+%E2%9A%A0%EF%B8%8F+forecasting+is+a+stub" alt="Typing SVG" />

<br>

[![Rust](https://img.shields.io/badge/Rust-CE422B?style=for-the-badge&logo=rust&logoColor=white&labelColor=0D1117)](https://www.rust-lang.org)
[![Status](https://img.shields.io/badge/Status-Partial-FFA500?style=for-the-badge&labelColor=0D1117)](#status)

</div>

<img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%">

### What it's meant to be
A system dashboard with *predictive* load forecasting — not just "CPU is at 40% now" but "CPU will be at 70% in five minutes."

### Status: today vs. planned

| Module | Today | Planned |
|--------|-------|---------|
| `backend::collector` | ✅ Real — pulls live CPU %, memory %, and network in/out via the `sysinfo` crate | — |
| `backend::predict::forecast()` | ⚠️ Stub — always returns `0.0` for both CPU and memory | Actual time-series forecasting from snapshot history |
| `backend::analytics::emit()` | ✅ Real — prints a formatted snapshot + forecast line | Structured output / dashboard UI |

> [!NOTE]
> The system-stats collection genuinely works today. The "predictive analytics" half of the name doesn't exist yet — `forecast()` is a hardcoded no-op.

```sh
git clone https://github.com/NoamFav/Astrotop && cd Astrotop
cargo run
```

<img src="https://user-images.githubusercontent.com/73097560/115834477-dbab4500-a447-11eb-908a-139a6edaec5c.gif" width="100%">

<div align="center">
Made with ♥ by <a href="https://github.com/NoamFav">NoamFav</a>
<img src="https://capsule-render.vercel.app/api?type=waving&height=90&color=gradient&customColorList=12&section=footer" />
</div>
