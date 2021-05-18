
ADB_DEV = 3bc143c6
ADB_BBOX = /data/data/org.galexander.busybox/files

# env CARGO_PROFILE_RELEASE_LTO=fat

#BENCH_STR = --bench=bench-memcmp --bench=bench-memeq

BENCH_NM = bench-memchr
#BENCH_NM = bench-memcmp
#BENCH_NM = bench-memcpy
#BENCH_NM = bench-memeq
#BENCH_NM = bench-memmem
#BENCH_NM = bench-memset

BENCH_FNM = $(subst -,_,$(BENCH_NM))
BENCH_STR = --bench=$(BENCH_NM)

TARGET_GNU = x86_64-unknown-linux-gnu
TARGET_MUSL = x86_64-unknown-linux-musl
#TARGET_GNU = i686-unknown-linux-gnu
#TARGET_MUSL = i686-unknown-linux-musl

#TARGET_GNU = aarch64-unknown-linux-gnu
#TARGET_MUSL = aarch64-unknown-linux-musl
#TARGET_GNU = mips64el-unknown-linux-gnuabi64
#TARGET_MUSL = mips64el-unknown-linux-muslabi64

#TARGET_GNU = powerpc64le-unknown-linux-gnu

#TARGET_GNU = armv7-linux-androideabi
#TARGET_GNU = armv7-unknown-linux-gnueabihf
#TARGET_MUSL = armv7-unknown-linux-musleabihf

TSK = taskset -c 2


all:

bench-all: bench-gnu bench-musl

bench-build-all: bench-build-gnu bench-build-musl


bench-gnu: bench.en.1-gnu bench.ja.1-gnu

bench-musl: bench.en.1-musl bench.ja.1-musl

target/stamp.bench-build-gnu: bench-build-gnu
bench-build-gnu:
	cargo bench --no-run --target=$(TARGET_GNU)
	@touch target/stamp.bench-build-gnu

target/stamp.bench-build-musl: bench-build-musl
bench-build-musl:
	cargo bench --no-run --target=$(TARGET_MUSL)
	@touch target/stamp.bench-build-musl

bench-clean:
	@rm -fr target/criterion

clean:
	@cargo clean
	@rm -f z.*

report:
	cargo xtask shape_benchmark_results


bench.en.1-gnu: target/stamp.bench-build-gnu
	@rm -fr target/criterion
	@rm -f z.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) --target=$(TARGET_GNU) -- -n | tee -a z.bench.en.1.log

bench.ja.1-gnu: target/stamp.bench-build-gnu
	@rm -fr target/criterion
	@rm -f z.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) --target=$(TARGET_GNU) -- -n | tee -a z.bench.ja.1.log

bench.en.1-musl: target/stamp.bench-build-musl
	@rm -fr target/criterion
	@rm -f z.musl.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) --target=$(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl: target/stamp.bench-build-musl
	@rm -fr target/criterion
	@rm -f z.musl.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) --target=$(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log


bench-andr: bench.en.1-andr bench.ja.1-andr bench.en.1-musl-andr bench.ja.1-musl-andr

target/$(BENCH_FNM)_andr: target/stamp.bench-build-gnu
	cp `find target/$(TARGET_GNU)/release/deps/ -name $(BENCH_FNM)-* | sort | head -n 1` target/$(BENCH_FNM)_andr
	strip target/$(BENCH_FNM)_andr

target/$(BENCH_FNM)_musl: target/stamp.bench-build-musl
	cp `find target/$(TARGET_MUSL)/release/deps/ -name $(BENCH_FNM)-* | sort | head -n 1` target/$(BENCH_FNM)_musl
	strip target/$(BENCH_FNM)_musl

target/stamp.bench-build-gnu-andr: target/$(BENCH_FNM)_andr
	adb -s $(ADB_DEV) push target/$(BENCH_FNM)_andr /data/local/tmp/target/
	@touch target/stamp.bench-build-gnu-andr

target/stamp.bench-build-musl-andr: target/$(BENCH_FNM)_musl
	adb -s $(ADB_DEV) push target/$(BENCH_FNM)_musl /data/local/tmp/target/
	@touch target/stamp.bench-build-musl-andr

bench.en.1-andr: target/stamp.bench-build-gnu-andr
	@rm -f z.bench.en.1.log
	adb -s $(ADB_DEV) shell "$(ADB_BBOX)/env AKI_TEST_DAT=en.1 /data/local/tmp/target/$(BENCH_FNM)_andr -n --bench | $(ADB_BBOX)/cat" | tee -a z.bench.en.1.log

bench.ja.1-andr: target/stamp.bench-build-gnu-andr
	@rm -f z.bench.ja.1.log
	adb -s $(ADB_DEV) shell "$(ADB_BBOX)/env AKI_TEST_DAT=ja.1 /data/local/tmp/target/$(BENCH_FNM)_andr -n --bench | $(ADB_BBOX)/cat" | tee -a z.bench.ja.1.log

bench.en.1-musl-andr: target/stamp.bench-build-musl-andr
	@rm -f z.musl.bench.en.1.log
	adb -s $(ADB_DEV) shell "$(ADB_BBOX)/env AKI_TEST_DAT=en.1 /data/local/tmp/target/$(BENCH_FNM)_musl -n --bench | $(ADB_BBOX)/cat" | tee -a z.musl.bench.en.1.log

bench.ja.1-musl-andr: target/stamp.bench-build-musl-andr
	@rm -f z.musl.bench.ja.1.log
	adb -s $(ADB_DEV) shell "$(ADB_BBOX)/env AKI_TEST_DAT=ja.1 /data/local/tmp/target/$(BENCH_FNM)_musl -n --bench | $(ADB_BBOX)/cat" | tee -a z.musl.bench.ja.1.log
