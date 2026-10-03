<div align="center">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/brand/banner-night.svg">
  <img alt="Astrotop: Rust system dashboard. Live CPU/mem/network collection works; the predictive forecasting module is still a stub." src=".github/brand/banner-paper.svg" width="100%">
</picture>
<br><br>
<a href="#what-its-meant-to-be"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/brand/tab-what-its-meant-to-be-night.svg"><img alt="what it's meant to be" src=".github/brand/tab-what-its-meant-to-be-paper.svg"></picture></a>
<a href="#status"><picture><source media="(prefers-color-scheme: dark)" srcset=".github/brand/tab-status-night.svg"><img alt="status" src=".github/brand/tab-status-paper.svg"></picture></a>
</div>

<p>
<a name="what-its-meant-to-be"></a>
<picture><source media="(prefers-color-scheme: dark)" srcset=".github/brand/section-what-its-meant-to-be-night.svg"><img alt="what it's meant to be" src=".github/brand/section-what-its-meant-to-be-paper.svg" width="100%"></picture>
</p>

A system dashboard with *predictive* load forecasting — not just "CPU is at 40% now" but "CPU will be at 70% in five minutes."

<p>
<a name="status"></a>
<picture><source media="(prefers-color-scheme: dark)" srcset=".github/brand/section-status-night.svg"><img alt="status" src=".github/brand/section-status-paper.svg" width="100%"></picture>
</p>

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

<div align="center">
Made with ♥ by <a href="https://github.com/NoamFav">NoamFav</a>
</div>

<br>

<a href="https://nf-software.com">
<picture>
  <source media="(prefers-color-scheme: dark)" srcset=".github/brand/footer-night.svg">
  <img alt="NF Software" src=".github/brand/footer-paper.svg" width="100%">
</picture>
</a>
