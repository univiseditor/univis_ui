# Roadmap Incremental Settlement

## Hadaf

- [ ] nwa9fo tikrar dyal lhisabat li ma kaybeddelch loutput
- [ ] n3rfo b de9a wash kol root settled
- [ ] n3rfo b de9a wash kol child kml `measure`, `solve`, w `render`
- [ ] nkhalliw propagation ghir 3la nodes li b7e9 t2atro

## Phase 0: Baseline w taswir l7ala l7aliya

- [ ] jme3 baseline mn `runtime_benchmarks` l scenarios li ahem: `root_capsules_96`, `text_measure_180`, `widget_panels_240`, `world3d_panels_48`
- [ ] zid counters f profiler bach n3rfo ch7al mn node t7seb f kol stage: `measure`, `solve`, `render`, `picking`
- [ ] zid counters bach n3rfo ch7al mn write tdar b nafs l9ima
- [ ] zid counters bach n3rfo ch7al mn root kayt3awdlo full refresh bla sabab
- [ ] khlli had baseline howa marja3 9bl ay phase jdida

## Phase 1: Wa9f change-noise

- [ ] f `root_stacking`, ma tktbch `ResolvedRootStack` ila ma tbdel 7tta 7aja
- [ ] f `pass_down`, ma tktbch `ComputedSize` ila size hiya nafs l9dima
- [ ] f `pass_down`, ma tktbch `Transform` ila x/y/z ma tbeddlouch
- [ ] f `text_label`, ma tktbch cache aw intrinsic ila natiija hiya nafs l9dima
- [ ] verify belli `Changed<>` wla kaytriggera ghir ila kayn taghyir 7a9i9i
- [ ] 3awd qiss benchmarks bach tchof wach full refresh hbatt

## Phase 2: Invalidation mdabza m3a dependencies

- [ ] qsem anwa3 dyal taghyir: `layout input`, `intrinsic output`, `root state`, `clip`, `render-only`
- [ ] dir mapping wazi7: chno kay2atar 3la `measure`, chno kay2atar 3la `solve`, chno kay2atar 3la `render`
- [ ] ila taghyir f leaf ma kaybeddelch output, ma t9llbch ancestors bzzaf
- [ ] ila taghyir ghir render, ma t3awdch `measure` wla `solve`
- [ ] ila taghyir ghir root transform w ma tbeddelch stack final, ma tdirch full hierarchy refresh
- [ ] khlli propagation tl3 ghir fin kayn dependency s7i7a

## Phase 3: Versioning per node

- [ ] zid state per node fih versions dyal stages
- [ ] zid `measure_input_version` w `measure_done_version`
- [ ] zid `solve_input_version` w `solve_done_version`
- [ ] zid `render_input_version` w `render_done_version`
- [ ] qanoon lclean state ykoun: stage clean ila `done_version == input_version`
- [ ] ila output tbeddel, zed version ghir l dependents li m3tamdine 3lih
- [ ] ila output ma tbeddelch, hbes propagation tmma

## Phase 4: Settlement per root

- [x] zid state per root fih `current_generation`
- [x] zid counters per root: `pending_measure`, `pending_solve`, `pending_render`
- [x] kol node y3rf root dyalo b cache mzyana
- [ ] mlli node ydkhol queue dyal stage, zed counter dyal had root
- [ ] mlli node ykml stage dyalo b success, n9s counter dyal had root
- [x] root ykoun settled ila pending counters kamlin `0`
- [x] l `UiWorkState` yb9a ghir orchestration, walakin l7okm 3la completion ykoun per root

## Phase 5: Queues w frontier processing

- [x] 3awed `depth scan` l systems dyal frontier queues
- [x] dir queue dyal `measure frontier`
- [x] dir queue dyal `solve frontier`
- [x] dir queue dyal `render frontier`
- [x] process ghir nodes li mdakhlin lqueue, machi ga3 `entities_by_depth`
- [x] retain ordering mlli kat7taj depth aw root ordering
- [x] ila queue khawya, stage ma ydkhlch lwork
- [x] ila node deja queued b nafs version, ma tzidouch mra khra

## Phase 6: Precise propagation rules

- [x] `leaf intrinsic changed` ypropagi ghir l parent li kay7taj intrinsic
- [ ] `container solved size changed` ypropagi ghir l children li position/constraints dyalhom kaytbdlo
- [x] `root canvas changed` ypropagi ghir l subtree dyal dak root
- [x] `clip changed` ypropagi ghir l descendants li kayt2atro b dak clip ancestor
- [x] `text autosize` y2atar ghir label nafsu w ancestors li kay7sbou content size
- [x] `render mesh/material` ma y2atarsh 3la `measure` w `solve`

## Phase 7: Tn9is scans w allocations

- [x] f `pass_up`, 7yed `node.clone`, `layout.clone`, w `Vec<Entity>` ila momkin khdem references aw scratch buffers
- [x] f `pass_down`, 7yed allocations jdod dyal `solver_items_owned`, `solver_items_refs`, `solved_children` f kol container
- [x] dir scratch buffers reusable per frame aw per system
- [x] f `fit_node_to_text_size`, khdem ghir 3la labels li `autosize == true`
- [x] f `picking`, zid broad-phase aw bucketing ila bban hotspot ba9i
- [x] qiss allocation churn b profiler aw allocator stats ila kayn

## Phase 8: Cached context w root refresh

- [x] rbat full refresh dyal `CachedUiContext` ghir b real root state changes
- [x] ma t3tabrch `ResolvedRootStack` changed ila ghir write-noise
- [x] ila root/camera/clip ma tbeddlouch, khdem incremental mode fa9at
- [x] verify belli moving root ma kaydirch refresh l roots okhrin
- [x] verify belli removing child aw clip kayb9a incremental ila momkin

## Phase 9: Tests dyal correctness

- [x] test: node tbeddel text dyalo w output ma tbeddelch, parent ma y3awdch solve
- [x] test: node tbeddel text dyalo w output tbeddel, parent kaytwasakh ghir wa7d lmarra
- [x] test: root wa7d tbeddel, root akhor ma ytsiftch lqueue
- [x] test: root settled means ga3 nodes dyalo `done_version == input_version`
- [x] test: no infinite requeue m3a autosize text
- [x] test: no stale layout mlli child size tbeddel
- [ ] test: no stale clip/picking/render mlli root state tbeddel

## Phase 10: Rollout b amane

- [x] dir feature gates aw rollout flags l versioning w frontier queues
- [x] khlli old path kayb9a kayn 7tta tkon parity wad7a
- [x] zid shadow validation bin old path w new path f scenarios m3yana
- [x] 3awd baseline b nafs warmup/iterations
- [x] dwi 3la regressions b `p95` 9bl `avg`
- [ ] mlli parity ttbata, 7yed path 9dim b chwiya

## Definition dyal success

- [ ] kol root y9dar y9oul b sra7a wash settled aw la
- [ ] kol child y9dar y9oul wash `measure/solve/render` dyalo mkamlin l current version
- [ ] ma yb9awsh full-tree scans f lframes li fiha taghyir sghir
- [ ] `root_capsules_96` yhbatt fih refresh noise
- [ ] `text_measure_180` yhbatt fih autosize/text churn
- [ ] `widget_panels_240` ybayan fih l2atar dyal precise propagation
- [ ] behavior yb9a nafsu bla regressions functional
