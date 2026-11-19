# Tasks

## 1. Container smoke job

- [x] 1.1 Add a `linux-container` job to `.github/workflows/ci.yml` that builds and renders every tab with `--once` inside a `rust` slim container, asserting each frame contains the tab strip. Verify: the job is listed in the workflow and runs the same render loop as the runner smoke job.
- [x] 1.2 Check that the Services tab shows an unavailable message without systemd. Verify: the container job greps for `Services are unavailable`.

## 2. Real Linux host

- [x] 2.1 Build and test on the Linux server at 192.168.0.55, then render every tab with `--once`. Verify: `cargo test` passes there and every frame contains the tab strip.

## 3. First CI run

- [ ] 3.1 Create a private GitHub repo and push main. Verify: `gh run list` shows a run for the pushed commit.
- [ ] 3.2 Fix every failure the first run shows on either platform, each as its own commit. Verify: `gh run view` reports every job green.
