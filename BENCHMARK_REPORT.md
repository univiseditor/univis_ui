# Benchmark Report

Tarikh: `2026-04-09`

Had report kayjma3 l-ar9am lli 3andhom qima ta9ririya:

- comparison rasmi bin `v0.2.0-alpha.2` w `0.2.0-alpha.3`
- sustained runtime checks jdadin dyal `alpha-3`

Kol l-ar9am b `ms`.

## Scope

- `alpha-2`: tag rasmi `v0.2.0-alpha.2` (`4b0dcc8`)
- `alpha-3`: workspace `0.2.0-alpha.3`
- compare run bin `alpha-2` w `alpha-3`:
  - `warmup=64`
  - `iterations=320`
- sustained checks dyal `alpha-3`:
  - `warmup=32`
  - `iterations=120`
  - `--check`

Molahada mohimma: `alpha-2` ma kanatch fiha sustained scenarios jdadin, donc l-comparison bin `alpha-2` w `alpha-3` tdar ghir 3la scenarios l-moshtaraka binhom.

## Runtime Compare: Alpha-2 vs Alpha-3

| Scenario | Items | Alpha-2 Avg | Alpha-2 p95 | Alpha-2 Max | Alpha-3 Avg | Alpha-3 p95 | Alpha-3 Max | p95 Delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `root_capsules_96` | 96 | 1.967 | 2.586 | 3.377 | 0.741 | 0.880 | 1.340 | `-66.0%` |
| `text_measure_180` | 180 | 1.709 | 2.630 | 2.973 | 2.060 | 3.143 | 3.870 | `+19.5%` |
| `picking_grid_512` | 512 | 0.480 | 0.530 | 1.047 | 0.600 | 0.676 | 1.425 | `+27.5%` |
| `widget_panels_240` | 240 | 2.328 | 3.314 | 4.091 | 1.165 | 1.657 | 1.872 | `-50.0%` |
| `world3d_panels_48` | 48 | 1.000 | 1.291 | 1.992 | 0.647 | 0.770 | 1.596 | `-40.4%` |

### Runtime Aggregate

| Group | Alpha-2 Avg Sum | Alpha-3 Avg Sum | Avg Delta | Alpha-2 p95 Sum | Alpha-3 p95 Sum | p95 Delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Shared runtime scenarios | 7.484 | 5.213 | `-30.3%` | 10.351 | 7.126 | `-31.2%` |

### Runtime Hokm

- `alpha-3` rab7a b wodo7 f sustained runtime kamel.
- akbar rba7 kano f `root_capsules_96`, `widget_panels_240`, w `world3d_panels_48`
- baqi regressions f `text_measure_180` w `picking_grid_512`

## Solver Compare: Alpha-2 vs Alpha-3

| Scenario | Items | Alpha-2 Avg | Alpha-2 p95 | Alpha-2 Max | Alpha-3 Avg | Alpha-3 p95 | Alpha-3 Max | p95 Delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `dense_row_512` | 512 | 0.091 | 0.100 | 0.137 | 0.107 | 0.120 | 0.174 | `+20.0%` |
| `wrap_cards_400` | 400 | 0.026 | 0.026 | 0.061 | 0.031 | 0.031 | 0.079 | `+19.2%` |
| `grid_dashboard_196` | 196 | 0.018 | 0.018 | 0.027 | 0.022 | 0.023 | 0.026 | `+27.8%` |

### Solver Aggregate

| Group | Alpha-2 Avg Sum | Alpha-3 Avg Sum | Avg Delta | Alpha-2 p95 Sum | Alpha-3 p95 Sum | p95 Delta |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Shared solver scenarios | 0.135 | 0.160 | `+18.5%` | 0.144 | 0.174 | `+20.8%` |

### Solver Hokm

- solver microbench ma howach l-jihah lli rb7at f `alpha-3`
- hadchi mtaf9 m3a target dyal `alpha-3`: ta9lil l-3amal l-mokarrar f runtime sustained, machi darori ta9lil cold-start recompute

## Alpha-3 Sustained Runtime Checks

Had scenarios tzado specifically bach yqiso l-behavior li `alpha-3` msyyra lih:

- `idle_after_settle_96`
- `single_root_local_change_96`
- `render_only_change_512`

| Scenario | Items | Avg | p95 | Max | Budget | Budget Use | Status |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `idle_after_settle_96` | 96 | 0.728 | 0.884 | 1.108 | 2.000 | `44.2%` | `ok` |
| `single_root_local_change_96` | 96 | 0.425 | 0.502 | 0.769 | 1.200 | `41.8%` | `ok` |
| `render_only_change_512` | 512 | 0.317 | 0.451 | 2.015 | 1.500 | `30.1%` | `ok` |

### Mibyan Sghir: p95 vs Budget

```text
idle_after_settle_96       0.884 / 2.000  [#########-----------] 44%
single_root_local_change   0.502 / 1.200  [########------------] 42%
render_only_change_512     0.451 / 1.500  [######--------------] 30%
```

### Sustained Hokm

- idle ba9i khfif bzzaf ba3d ma UI tssettla
- local change f root wa7ed ba9i scoped w daz b headroom mzyan
- render-only visual churn daz mzyan bla ma y7taj budget kbir

## Kholasa Ntiija

- ila kan l-focus howa runtime sustained, `alpha-3` a7san mn `alpha-2` b far9 wazi7
- ila kan l-focus howa solver microbench cold-start, `alpha-3` at9al chwiya
- sustained scenarios jdadin kay2akdo blli l-maksad dyal `alpha-3` daz:
  - no-op/idle ba9i stable
  - localized mutations ba9in m7dodin
  - render-only changes ma kay7tajoch budget kbir
