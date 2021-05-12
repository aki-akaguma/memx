BENCH_STR = --bench=bench-ss-algo --bench=bench-ss-algo-indices
#BENCH_STR = --bench=bench-ss-algo-indices

TARGET_MUSL = --target=x86_64-unknown-linux-musl
TSK = taskset -c 2

all:

bench-all: bench bench-musl

bench-build-all: bench-build bench-build-musl


bench: bench.en.1 bench.ja.1

bench-musl: bench.en.1-musl bench.ja.1-musl

bench-build:
	cargo bench --no-run
	@touch target/stamp.bench-build

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


bench.en.1: target/stamp.bench-build
	@rm -f z.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) -- -n | tee -a z.bench.en.1.log

bench.ja.1: target/stamp.bench-build
	@rm -f z.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) -- -n | tee -a z.bench.ja.1.log

bench.en.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.en.1.log
	env AKI_TEST_DAT=en.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.en.1.log

bench.ja.1-musl: target/stamp.bench-build-musl
	@rm -f z.musl.bench.ja.1.log
	env AKI_TEST_DAT=ja.1 $(TSK) cargo bench $(BENCH_STR) $(TARGET_MUSL) -- -n | tee -a z.musl.bench.ja.1.log
