# Roadmap Post-Review

Had roadmap hiya l current active roadmap mabniya 3la review dyal l project b halto daba, w l hadaf dyalo howa:

- y7ssen maintainability
- yzid thi9a f release quality
- y9lls regressions f layout / rendering / interaction
- ywjjed l project l next alpha b tari9a mratba

## Chno bano f review

Hadchi howa akbar 4 n9at li khas nerkzo 3lihom daba:

- core architecture mzyana, walakin kaynin files kbar bzaf f core logic
- testing w CI qwiya, walakin visual validation mazal manual
- docs mzyana bzaf, walakin khas-ha root-level polish w clarity mazid
- kayban ihtimam b performance, walakin ma kaynach benchmark suite rasmiya

## Awlawiyat 3amma

1. stability w clarity qbl ay refactor kbir
2. tqsim lfiles lkbar qbl ma yzid 3lihom debt jdida
3. performance evidence, maشي ghir performance claims
4. visual confidence qbl next release
5. release story waD7a, synced, w sahla l users

## Tarteeb Tanfid Jadid

- [x] ba3d ma tsddat l mar7ala dyal refactor, awlawiya jaya hiya benchmarks w performance program
- [x] visual validation ghadi tmchi mn ba3d performance pass, maشي qblha
- [x] numbering dyal sections bqat kif hiya bach l history w references ma ytkhaltoch, walakin order dyal tanfid wla:
  - current next focus: Phase 4
  - mn ba3d: Phase 3

## Phase 1: 0 ila 2 simanat

### Lhadaf

Nsddo l polish issues sghar, n7ddo debt b waD7, w n7afdo 3la baseline dyal quality li deja mzyan.

### Status

- [x] Phase 1 bdath
- [x] Phase 1 salat

### Chno ytdar

- [x] s7e7 typo dyal `mdbook serve` f `README.md`
- [x] n9i root/docs surfaces mn references li ma b9awch kaynin
- [x] mur 3la root files kamlin:
  - `README.md`
  - `README_AR.md`
  - `MIGRATION.md`
  - `MIGRATION_AR.md`
  - `RELEASE_NOTES.md`
  - `ROADMAP_POST_REVIEW.md`
  - `changelog.md`
- [x] 7dded b waD7:
  - shno stable enough
  - shno experimental
  - shno deprecated walakin baqi supported
- [x] dir inventory rasmi dyal technical debt f core files lkbar:
  - `crates/univis_ui_widgets/src/widget/text_label.rs`
  - `crates/univis_ui_engine/src/layout/layout_system.rs`
  - `crates/univis_ui_engine/src/layout/core/solver.rs`
  - `crates/univis_ui_widgets/src/widget/select.rs`
- [x] khrej issue aw note l kol file:
  - 3lach khas yttqssam
  - fin mkhllt logic
  - shno l modules li ymkn ytsnaw
- [x] khlli had validation mandatory qbl merge:
  - `./scripts/check_quality.sh`
  - `./scripts/check_representative_examples.sh`
  - `mdbook build docs`

### Deliverables

- [x] root docs msaybin w mtwaf9in
- [x] document sghir aw issues backlog dyal debt
- [x] checklist waD7a qbl merge

### Ntiija li khas tban

- ay contributor y3raf fin ymchi bach yfhem project
- ay wa7ed yqra root files yfhm wa9i3 l alpha بلا غموض
- debt ttwli ma3roufa w ma tb9ach mkhbiya

### Risks

- risk: nkthro mn docs polish bla impact
- solution: ay update khas-ha tkon mwasla b stable/experimental clarity aw onboarding betterment

## Phase 2: 2 ila 5 simanat

### Lhadaf

N7ssno maintainability b refactor m7soub, bla ma nkhsro stability.

### Status

- [x] Phase 2 bdath
- [x] Phase 2 salat

### Qae3da Tanthimiya

- [x] kol model aw feature family li ghadi ttssem khas-ha mojalad khass biha
- [x] ma nkhlliwch mojald wa7d yt3emmer b bzaaf dyal files mfrqin بلا grouping منطقي
- [x] `mod.rs` yb9a howa entry point, w dakchi dakhlo ytfr9 3la hsab المسؤوليات
- [x] ila kan area 3andha public API + runtime + tests, had tlata ykono mjem3in ta7t nafs lfolder

### Chno ytdar

- [x] qassem `text_label.rs` l folder `crates/univis_ui_widgets/src/widget/text_label/`
  - `mod.rs`
  - `model.rs`
  - `measure.rs`
  - `render.rs`
- [x] qassem `layout_system.rs` l folder `crates/univis_ui_engine/src/layout/layout_system/`
  - `mod.rs`
  - `types.rs`
  - `root_stacking.rs`
  - `ui3d_sync.rs`
  - existing `root_resolution.rs`
  - existing `screen_transform.rs`
- [x] qassem `solver.rs` l folder `crates/univis_ui_engine/src/layout/core/solver/`
  - `mod.rs`
  - `types.rs`
  - `helpers.rs`
  - `translate.rs`
  - `absolute.rs`
- [x] qassem `select.rs` l folder `crates/univis_ui_widgets/src/widget/select/`
  - `mod.rs`
  - `model.rs`
  - `runtime.rs`
  - `interaction.rs`
  - `visuals.rs`
  - `events.rs`
- [x] kol wa7d mn had l areas tsna b folder dyalo maشي b files mfr9in kamlin f nafs lmostawa
- [x] t7afd 3la nafs behavior qbl/ba3d refactor
- [x] validation dyal phase dazet b:
  - `cargo check --workspace`
  - `./scripts/check_quality.sh`
  - `./scripts/check_representative_examples.sh`

### Deliverables

- [x] core files tkssmo l folders mخصصin 3la hsab lresponsibility
- [x] modules asghar w ashal lmoraja3a
- [x] tests kamlin baqin green

### Ntiija li khas tban

- code review twlli ashal
- tracing dyal bugs ywli أسرع
- features jdod ma ybdawch mn files monolithic

### Risks

- [x] risk: refactor ykhrj regressions khfya
- [x] solution:
  - khlli refactor incremental
  - matbdelch behavior w API f nafs commit ila ma kanch darori
  - zid tests qbl matfrrq code ila l manta9 khass

## Phase 3: 5 ila 8 simanat

### Lhadaf

Nzido thi9a f visual behavior, 7it framework UI ma kaykfi fih compile/tests bo7dhom.

### Status

- [ ] Phase 3 bdath
- [ ] Phase 3 salat
- [ ] deferred 7tta ytsala l performance pass li ta7t

### Chno ytdar

- [ ] khtar representative visual pack mn examples:
  - `hello_world`
  - `root_screen_hud`
  - `root_world_scale`
  - `root_fit_content`
  - `border_light_3d`
  - `interaction`
  - `panel_window`
  - `text_field`
  - `mixed_bidi_text`
  - `text_label`
  - `sci_fi`
- [ ] l kol example, dir wa7ed mn juj:
  - screenshot reference
  - manual visual checklist waD7a
- [ ] zid docs dyal visual validation:
  - ashno kaytchaf
  - fin kayt7tt reference
  - kifach kayt2akd release manager mn natija
- [ ] zid policy:
  - ay bug visual mham khaso yjib m3ah example reproducer
  - ay bug fix visual mham khaso yjib m3ah update f visual checklist aw reference
- [ ] ila kan wa9t, bda smoke screenshot regression ghir 3la representative examples

### Deliverables

- [ ] representative examples mghattiyin basriyan
- [ ] visual release pass mratab
- [ ] docs dyal visual validation ashal l team

### Ntiija li khas tban

- regressions visual kaytbanu qbl release
- examples ma yb9awch ghir compile targets
- confidence f world2d/world3d/text/panels tzid

### Risks

- [ ] risk: visual workflow ywli 9ass7 w yt9el 3la contributors
- [ ] solution:
  - bda b manual checklist sahla
  - khlli automation ghir f representative pack

## Phase 4: 8 ila 12 simanat

### Lhadaf

N7awlo l performance mn "ihtimam waD7" l "evidence waD7a".

### Status

- [x] Phase 4 bdath
- [x] Phase 4 salat
- [x] had l section hiya next execution focus qbl Phase 3

### Chno ytdar

- [x] zid benchmark program rasmi l:
  - [x] layout solver
  - [x] text measurement / cache
  - [x] picking
  - [x] root resolution
  - [x] panel / widget-heavy scenes
- [x] 7dded scenarios 9yasiya:
  - [x] solver dense row
  - [x] solver wrapped cards
  - [x] solver grid dashboard
  - [x] scene text-heavy
  - [x] scene widget-heavy
  - [x] scene world3d-heavy
- [x] 7dded perf budgets ta9ribiyan:
  - [x] solver p95 budgets t7etto l current harness
  - [x] text/picking/root budgets
- [x] ila ma bghitich تدخل tooling kbir daba, bda b harness bsit dakhili, w mn ba3d zid solution akthar rasmiya
- [x] zid docs performance:
  - [x] kifach ytsel benchmark
  - [x] kifach tqra natayj
  - [x] fin yban regression f current solver harness

### Deliverables

- [x] benchmark folder aw perf harness waD7
- [x] perf scenarios m3arfin l solver w runtime waves
- [x] baseline numbers mt7afdin

### Ntiija li khas tban

- ay claim 3la speed aw cache efficiency ykoun m3ah number
- regressions performance ytwjdo bda bda
- next alpha tkon 3andha story performance a9wa

### Risks

- [ ] risk: benchmarking yakhod wa9t kbir bla value f lwal
- [ ] solution:
  - bda ghir b 3 aw 4 scenarios mhamin
  - ma t7awelch t9is kolchi mn nhar luwel

## Phase 5: 12 ila 16 simanat

### Lhadaf

Nwjjdo next release b clarity dyal public API, migration, w release readiness.

### Status

- [ ] Phase 5 bdath
- [ ] Phase 5 salat

### Chno ytdar

- [ ] 7dded supported API surface rasmi mn had l alpha phase
- [ ] confirmi future dyal:
  - `URootUi`
  - `UiSpace`
  - `UiCanvasSize`
  - `UiCameraRef`
  - deprecated wrappers b7al `UScreenRoot` w `UWorldRoot`
- [ ] update:
  - `RELEASE_NOTES.md`
  - `MIGRATION.md`
  - `MIGRATION_AR.md`
  - `ROADMAP_POST_REVIEW.md`
  - docs migration pages
- [ ] dir release readiness pass:
  - quality
  - docs
  - representative examples
  - full examples
  - visual pass
  - package rehearsal
- [ ] zid root-level note sghira:
  - ash t9dar tbni 3lih daba
  - ash khask t7sb lih 7it baqi transitional

### Deliverables

- [ ] release story mtwaf9a bin root files w docs
- [ ] stable surface waD7a l users
- [ ] migration path sahl

### Ntiija li khas tban

- user jdida yfhm project w ybda b confidence
- maintainer y9der yقطع release بلا tafri3
- public messaging twlli mratba w consistent

### Risks

- [ ] risk: release notes ywliw duplication dyal README
- [ ] solution:
  - khlli kol root file 3ando waDifa waD7a
  - mat3awdch nafs lhdra f kol file

## Quick Wins li nnsah bihom daba

Ila bghiti a9wa impact b a9al majhoud f had simanat lawlin:

- [x] s7e7 `README.md` typo dyal `mdbook serve`
- [ ] khrej debt inventory rasmi l 4 files lkbar
- [ ] bda b refactor `text_label.rs`
- [ ] mn ba3d dir `layout_system.rs`
- [ ] zid visual checklist l representative examples

## Chno ma nnsahch bih daba

- [ ] ma tdirch redesign kbir l architecture, 7it l base deja mzyana
- [ ] ma tzidch widget family kbira qbl ma t7ssn maintainability
- [ ] ma t7awelch tautomatisi visual regression 100% mn luwel
- [ ] ma t7awelch tn9s kol clippy allowances f da9a wa7da

## Success Metrics

Hadi metrics بسيطة bach n3rfo wach roadmap khdama:

- [ ] 4 akbar files wlao m9smin w review dyalhom ashal
- [ ] representative visual pack wla documented w kayt3awd f kul release
- [ ] benchmark baseline twjdat l 3la9al l core scenes
- [ ] root docs kamlin kaygolo nafs l story
- [ ] next alpha release t9dar ttbna 3la checklist waD7a, maشي 3la memory

## Final Recommendation

Ila khas nرتبو had roadmap b priority waD7a bzaf, had howa tartib li nnsah bih:

- [x] docs clarity + debt inventory salat
- [ ] refactor l core files lkbar
- [ ] visual validation pack
- [ ] performance benchmarks
- [ ] release-surface hardening

Ila tdar had tartib, l project ghaliban ghadi y7afed 3la l innovation dyalo, w f nafs lwa9t yzid f stability, maintainability, w thi9a dyal users w contributors.
