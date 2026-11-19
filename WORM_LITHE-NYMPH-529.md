# 🟢 `[WORM-LITHE-NYMPH-529]` We should move the test list to the left panel

- **UUID**: `31aa2d4b-786e-4189-8bb0-ef76a8675378`
- **ID**: `WORM-LITHE-NYMPH-529`
- **Status**: `RESOLVED`
- **Severity**: `MEDIUM`
- **Category**: `INFRASTRUCTURE`
- **Component**: `target`
- **Created**: `2026-11-19 03:43:19 UTC`
- **Resolved**: `2026-11-19 18:37:04 UTC`
- **Rejected**: `2026-11-19 16:02:14 UTC`

#### Description

We should move the test list and filter menu to the left panel so that the manual prompt has more space. The left panel should have way to select the active tab (test execution or system logging). Additionally, we should give the test execution list a stacked layout, similar to what is used for the VCS commit UI.

#### Reproduction Steps

1. Launch host daemon & synchronize firmware: python tools/host.py --package cat_detector --bin cat_detector_shell
2. Execute target command via python tools/host.py cli <command>
3. Observe defect/failure on target

#### Execution / Console Logs

```text
[] [daemon] [INFO] [Sync] Compiling host utilities (host_cli, host_fs)...
[] [daemon] [INFO] Building host utilities: host_cli and host_fs
[] [build] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.36s
[] [daemon] [INFO] Host tools ready: cli=/Users/<username>/gh/firmware/target/debug/host_cli, fs=/Users/<username>/gh/firmware/target/debug/host_fs
[] [daemon] [INFO] [Sync] Flashing target device via probe-rs...
[] [daemon] [INFO] Flashing target via cargo run: package=cat_detector, bin=cat_detector_shell, target=thumbv6m-none-eabi
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`dev` profile [optimized + debuginfo]]8;;\ target(s) in 0.07s
[] [flash] [INFO] [1m[92m     Running[0m `/Users/<username>/gh/firmware/tools/runner.sh target/thumbv6m-none-eabi/debug/cat_detector_shell --dump`
[] [flash] [INFO] [1m[92m    Finished[0m ]8;;https://doc.rust-lang.org/cargo/reference/profiles.html#default-profiles\`release` profile [optimized + debuginfo]]8;;\ target(s) in 0.22s
[] [flash] [INFO]      [1;32mFinished[0m in 12.24s
[] [flash] [INFO] RTT connected. Active channels:
[] [flash] [INFO]   - Up Channel: "defmt" (logs)
[] [flash] [INFO]   - Up Channel: "cli" (raw output)
[] [flash] [INFO]   - Down Channel: "cli" (raw input)
[] [flash] [INFO] shell> 
[] [flash] [INFO] [2K
[] [flash] [INFO]        |\      _,,,---,,_
[] [flash] [INFO]  Zzz   /,`.-'`'    -.  ;-;;,_
[] [flash] [INFO]       |,4-  ) )-,_. ,\ (  `'-'
[] [flash] [INFO]      '---''(_/--'  `-'\_)  
[] [flash] [INFO] Type 'help' to print usage.
[] [flash] [INFO] shell> 
[] [flash] [INFO] Draining buffered defmt logs:
[] [flash] [INFO] 0.002191 [32mINFO[0m MPU stack guard configured at 0x2003c000
[] [flash] [INFO] 0.023222 [32mINFO[0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] Flash download succeeded via cargo run
[] [daemon] [INFO] [Sync] Starting RTT streaming channels...
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002187 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023241 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] [Sync] Target synchronized & RTT connected successfully.
[] [system] [INFO] Switching to Test mode (on-device @device_test runner active)...
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002175 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002198 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023242 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002204 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.023236 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [daemon] [INFO] RTT stream disconnected
[] [daemon] [INFO] RTT streaming process connected
[] [target] [INFO] 0.002194 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2003c000
[] [target] [INFO] 0.029274 INFO [36m[platform::core_monitor][0m MPU stack guard configured at 0x2000cf00
[] [manual_test] [INFO] [MANUAL TEST: test_motor_calibration_routine] Procedure: Ensure fountain reservoir is filled above minimum waterline before starting calibration. | Expected: Is the reservoir filled and clear of obstructions?
[] [daemon] [INFO] Target bug created: WORM-FOSSORIAL-NIGHTCRAWLER-861 (Overlapping text in the test mode) with 2 attachments
```

#### Attached Commits

- `f20d3ec51ff63aef22169c685a13bdc0c4b849bb`
- `22307bbd`

#### Review Tasks & Functional Checklist

- [ ] Switch host.py ACTIVE MODE to TEST and verify the left panel opens on the TEST SUITE tab with tab switching to SYSTEM LOGS and spacious right-panel manual prompt layout | `python tools/host.py --no-detach` <!-- id:a5c70ced -->
- [ ] Open host.py in TEST mode and verify the left-panel TEST SUITE tab renders VCS-style stacked .test-stack-node cards with accent borders, card-click selection toggle, and attribute tags | `python tools/host.py --mode test` <!-- id:5d5f1f8b -->
- [ ] Click the per-card ▶ Play button next to any test status badge and verify it immediately transitions that card to RUNNING, updates the right-panel progress bar, and executes only that test | `python tools/host.py --mode test` <!-- id:19e222b8 -->

#### Resolution Notes

Replaced the 4-column test table with a VCS-style stacked card layout (#testStackList / .test-stack-node) matching Smartlog commit cards, updated 1s status polling to mutate existing card refs in-place instead of detaching button nodes mid-click, and bound direct + Text-node-safe click handlers on each card's ▶ Play button with immediate optimistic progress feedback.

#### Rejection Feedback

Moved the @device_test filter bar, run controls, and test list into #testSidebarView inside #logPanel with a left-panel tab switcher (TEST SUITE vs SYSTEM LOGS), freeing the right panel for spacious manual prompt interactions.
