
# env CARGO_PROFILE_RELEASE_LTO=fat

#BENCH_STR = --bench=bench-memeq --bench=bench-memeq

#BENCH_STR = --bench=bench-memchr
#BENCH_STR = --bench=bench-memcmp
#BENCH_STR = --bench=bench-memcpy
#BENCH_STR = --bench=bench-memeq
BENCH_STR = --bench=bench-memmem
#BENCH_STR = --bench=bench-memset

TARGET_GNU = --target=x86_64-unknown-linux-gnu
TARGET_MUSL = --target=x86_64-unknown-linux-musl
#TARGET_GNU = --target=i686-unknown-linux-gnu
#TARGET_MUSL = --target=i686-unknown-linux-musl

#TARGET_GNU = --target=armv7-linux-androideabi

#TARGET_GNU = --target=aarch64-unknown-linux-gnu
#TARGET_MUSL = --target=aarch64-unknown-linux-musl
#TARGET_GNU = --target=mips64el-unknown-linux-gnuabi64
#TARGET_MUSL = --target=mips64el-unknown-linux-muslabi64

#TARGET_GNU = --target=powerpc64le-unknown-linux-gnu

#TARGET_GNU = --target=armv7-unknown-linux-gnueabihf
#TARGET_MUSL = --target=armv7-unknown-linux-musleabihf


TSK = taskset -c 2

all:

bench-all: bench-gnu bench-musl

bench-build-all: bench-build-gnu bench-build-musl


bench-gnu: bench.en.1-gnu bench.ja.1-gnu

bench-musl: bench.en.1-musl bench.ja.1-musl

target/stamp.bench-build-gnu: bench-build-gnu
bench-build-gnu:
	cargo bench --no-run $(TARGET_GNU)
	@touch target/stamp.bench-build-gnu

target/stamp.bench-build-musl: bench-build-musl
bench-build-musl:
	cargo bench --no-run $(TARGET_MUSL)
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
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.bench.en.1.log

bench.ja.1-gnu: target/stamp.bench-build-gnu
	@rm -fr target/criterion
	@rm -f z.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_GNU) -- -n | tee -a z.bench.ja.1.log

bench.en.1-musl: target/stamp.bench-build-musl
	@rm -fr target/criterion
	@rm -f z.musl.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl: target/stamp.bench-build-musl
	@rm -fr target/criterion
	@rm -f z.musl.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
