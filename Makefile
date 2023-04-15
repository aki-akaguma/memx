
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test --offline --features test
	cargo test --offline --features test_pointer_width_128
	cargo test --offline --features test_pointer_width_64
	cargo test --offline --features test_pointer_width_32

AC_TESTS=--test memchr --test memrchr --test memnechr --test memrnechr --test memcmp --test memeq --test memcpy --test memset --test memchr_dbl -- test memrchr_dbl
test-alignment-check:
	cargo test --offline --features test_alignment_check $(AC_TESTS)
	cargo test --offline --features test_alignment_check,test_pointer_width_128 $(AC_TESTS)
	cargo test --offline --features test_alignment_check,test_pointer_width_64 $(AC_TESTS)
	cargo test --offline --features test_alignment_check,test_pointer_width_32 $(AC_TESTS)

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	cargo +nightly miri test --features test_pointer_width_128
	cargo +nightly miri test --features test_pointer_width_64
	cargo +nightly miri test --features test_pointer_width_32
	cargo +nightly miri test

clean:
	@cargo clean
	@rm -f z.*

clippy:
	cargo clippy --offline --tests --workspace -- -W clippy::uninlined_format_args

fmt:
	cargo fmt

doc:
	cargo doc

TARP_OPT = --offline --tests --engine llvm --line --out lcov --output-dir ./target
#TARP_OPT = --offline --tests --engine ptrace --line --out lcov --output-dir ./target

tarpaulin:
	@#cargo tarpaulin --offline --engine llvm --out html --output-dir ./target
	@#cargo tarpaulin --offline --engine ptrace --out html --output-dir ./target
	@
	cargo tarpaulin $(TARP_OPT) && mv target/lcov.info target/lcov.info.1
	RUSTFLAGS="-C target-feature=-sse2,-avx2" \
	cargo tarpaulin $(TARP_OPT) && mv target/lcov.info target/lcov.info.2
	@
	cargo tarpaulin --features test_pointer_width_128 $(TARP_OPT) && mv target/lcov.info target/lcov.info.3
	RUSTFLAGS="-C target-feature=-sse2,-avx2" \
	cargo tarpaulin --features test_pointer_width_128 $(TARP_OPT) && mv target/lcov.info target/lcov.info.4
	@ cargo tarpaulin --features test_pointer_width_64 $(TARP_OPT) && mv target/lcov.info target/lcov.info.5
	RUSTFLAGS="-C target-feature=-sse2,-avx2" \
	cargo tarpaulin --features test_pointer_width_64 $(TARP_OPT) && mv target/lcov.info target/lcov.info.6
	@
	cargo tarpaulin --features test_pointer_width_32 $(TARP_OPT) && mv target/lcov.info target/lcov.info.7
	RUSTFLAGS="-C target-feature=-sse2,-avx2" \
	cargo tarpaulin --features test_pointer_width_32 $(TARP_OPT) && mv target/lcov.info target/lcov.info.8
	@
	genhtml -o target/lcov --demangle-cpp target/lcov.info.*

COV_ENV1 = CARGO_INCREMENTAL=0 LLVM_PROFILE_FILE='$(CURDIR)/target/profraw/cargo-test-%p-%m.profraw' RUSTFLAGS='-Cinstrument-coverage'
COV_ENV2 = CARGO_INCREMENTAL=0 LLVM_PROFILE_FILE='$(CURDIR)/target/profraw/cargo-test-%p-%m.profraw' RUSTFLAGS='-Cinstrument-coverage -C target-feature=-sse2,-avx2'
GRCOV_TEST=--test memset
GRCOV_TEST=
grcov:
	@rm -rf $(CURDIR)/target/profraw
	@rm -rf $(CURDIR)/target/coverage
	$(COV_ENV1) cargo test --offline $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_pointer_width_128 $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_pointer_width_64 $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_pointer_width_32 $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_alignment_check $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_alignment_check,test_pointer_width_128 $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_alignment_check,test_pointer_width_64 $(GRCOV_TEST)
	$(COV_ENV1) cargo test --offline --features test_alignment_check,test_pointer_width_32 $(GRCOV_TEST)
	@mkdir -p $(CURDIR)/target/coverage
	grcov $(CURDIR)/target/profraw --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html

BG_PROF=CARGO_PROFILE_BENCH_LTO=no CARGO_PROFILE_BENCH_OPT_LEVEL=0
BG_PROF=
BG_PROF=CARGO_PROFILE_RELEASE_LTO=no CARGO_PROFILE_RELEASE_OPT_LEVEL=0
BENCH_GRCOV=
BENCH_GRCOV=--bench bench-memset
bench-grcov:
	@rm -rf $(CURDIR)/target/profraw
	@rm -rf $(CURDIR)/target/coverage
	$(COV_ENV1) $(BG_PROF) cargo xbench $(BENCH_GRCOV)
	@mkdir -p $(CURDIR)/target/coverage
	grcov $(CURDIR)/target/profraw --binary-path $(CURDIR)/target/release/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o $(CURDIR)/target/coverage/html

rustc_vers = 1.56.1 1.57.0 1.58.1 1.59.0 1.60.0 1.61.0 1.62.1 1.63.0 \
	1.64.0 1.65.0 1.66.1
target_base_vers = x86_64-unknown-linux-gnu i586-unknown-linux-gnu

define test-rustc-templ =
target/stamp/stamp.test-rustc.$(1).$(2):
	@mkdir -p target/stamp
	cargo +$(1) test --target=$(2)
	@touch target/stamp/stamp.test-rustc.$(1).$(2)
endef

#bench_nms = bench-memchr bench-memrchr bench-memnechr bench-memrnechr bench-memcmp bench-memeq bench-memcpy bench-memset bench-memmem bench-memrmem bench-memchr_dbl bench-memrchr_dbl
#bench_nms = bench-memchr_dbl bench-memrchr_dbl
#bench_nms = bench-memcmp bench-memeq
#bench_nms = bench-memcpy bench-memset
bench_nms = bench-memset

#target_base = x86_64-unknown-linux i686-unknown-linux i586-unknown-linux
#target_base = x86_64-unknown-linux i686-unknown-linux
#target_base = i686-unknown-linux i586-unknown-linux
target_base = x86_64-unknown-linux

define build-templ =
target/stamp.build/stamp.build.$(1).$(2):
	@#mkdir -p target/stamp
	$(MAKE) -f makefile.build BENCH_NM=$(1) TARGET_GNU=$(2)-gnu TARGET_MUSL=$(2)-musl bench-build-all
	@#touch target/stamp/stamp.build.$(1).$(2)

endef

define build-armv7-templ =
target/stamp/stamp.build.$(1).armv7:
	@mkdir -p target/stamp
	$(MAKE) -f makefile.build BENCH_NM=$(1) bench-build-arm-all
	@touch target/stamp/stamp.build.$(1).armv7

endef

define bench-templ =
target/stamp.bench/stamp.bench.$(1).$(2):
	@mkdir -p target/stamp.bench
	@mkdir -p target/result
	$(MAKE) -f makefile.build BENCH_NM=$(1) TARGET_GNU=$(2)-gnu TARGET_MUSL=$(2)-musl bench-all
	$(MAKE) -f makefile.build report | tee target/result/result.$(1).$(2).txt
	@touch target/stamp.bench/stamp.bench.$(1).$(2)

endef

define bench-armv7-templ =
target/stamp/stamp.bench.$(1).armv7:
	@mkdir -p target/stamp
	@mkdir -p target/result
	$(MAKE) -f makefile.build BENCH_NM=$(1) bench-arm-all
	$(MAKE) -f makefile.build report | tee target/result/result.$(1).armv7.txt
	@touch target/stamp/stamp.bench.$(1).armv7

endef

test-all-version: $(foreach ver,$(rustc_vers),$(foreach tb,$(target_base_vers),target/stamp/stamp.test-rustc.$(ver).$(tb)))

test-clean:
	@rm -fr target/stamp

$(foreach ver,$(rustc_vers),$(eval $(foreach tb,$(target_base_vers),$(eval $(call test-rustc-templ,$(ver),$(tb))))))

ENV_DAT_CHECK = AKI_TEST_DAT_CHECK="" CARGO_PROFILE_BENCH_LTO=no CARGO_PROFILE_BENCH_OPT_LEVEL=0 cargo xbench

bench-dat-check:
	@#AKI_TEST_DAT=ja.1 AKI_TEST_STATISTICS="" AKI_TEST_DAT_CHECK="" CARGO_PROFILE_BENCH_LTO=no cargo xbench --bench=bench-memnechr
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memchr
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memchr
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memrchr
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memrchr
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memnechr
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memnechr
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memrnechr
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memrnechr
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memcmp
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memcmp
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memeq
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memeq
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memcpy
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memcpy
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memset
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memset
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memmem
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memmem
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memrmem
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memrmem
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memchr_dbl
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memchr_dbl
	AKI_TEST_DAT=en.1 $(ENV_DAT_CHECK) --bench=bench-memrchr_dbl
	AKI_TEST_DAT=ja.1 $(ENV_DAT_CHECK) --bench=bench-memrchr_dbl

bench-build-all: $(foreach bnm,$(bench_nms),$(foreach tbm,$(target_base),target/stamp.build/stamp.build.$(bnm).$(tbm)))

bench-all: $(foreach bnm,$(bench_nms),$(foreach tbm,$(target_base),target/stamp.bench/stamp.bench.$(bnm).$(tbm)))

build-arm-all: $(foreach bnm,$(bench_nms),target/stamp/stamp.build.$(bnm).armv7)

bench-arm-all: $(foreach bnm,$(bench_nms),target/stamp/stamp.bench.$(bnm).armv7)

$(foreach bnm,$(bench_nms),$(eval $(foreach tb,$(target_base),$(eval $(call build-templ,$(bnm),$(tb))))))

$(foreach bnm,$(bench_nms),$(eval $(foreach tb,$(target_base),$(eval $(call bench-templ,$(bnm),$(tb))))))

$(foreach bnm,$(bench_nms),$(eval $(call build-armv7-templ,$(bnm))))

$(foreach bnm,$(bench_nms),$(eval $(call bench-armv7-templ,$(bnm))))


clean-memchr:
	@rm -f target/stamp.build/stamp.build.bench-memchr.*
	@rm -f target/stamp.bench/stamp.bench.bench-memchr.*

clean-memcmp:
	@rm -f target/stamp.build/stamp.build.bench-memcmp.*
	@rm -f target/stamp.bench/stamp.bench.bench-memcmp.*

clean-memcpy:
	@rm -f target/stamp.build/stamp.build.bench-memcpy.*
	@rm -f target/stamp.bench/stamp.bench.bench-memcpt.*

clean-memeq:
	@rm -f target/stamp.build/stamp.build.bench-memeq.*
	@rm -f target/stamp.bench/stamp.bench.bench-memeq.*

clean-memmem:
	@rm -f target/stamp.build/stamp.build.bench-memmem.*
	@rm -f target/stamp.bench/stamp.bench.bench-memmem.*

clean-memrchr:
	@rm -f target/stamp.build/stamp.build.bench-memrchr.*
	@rm -f target/stamp.bench/stamp.bench.bench-memrchr.*

clean-memrmem:
	@rm -f target/stamp.build/stamp.build.bench-memrmem.*
	@rm -f target/stamp.bench/stamp.bench.bench-memrmem.*

clean-memset:
	@rm -f target/stamp.build/stamp.build.bench-memset.*
	@rm -f target/stamp.bench/stamp.bench.bench-memset.*


result-memchr:
	cat target/result/result.bench-memchr.x86_64-unknown-linux.txt
	cat target/result/result.bench-memchr.i686-unknown-linux.txt
	cat target/result/result.bench-memchr.i586-unknown-linux.txt
	cat target/result/result.bench-memchr.armv7.txt

result-memcmp:
	cat target/result/result.bench-memcmp.x86_64-unknown-linux.txt
	cat target/result/result.bench-memcmp.i686-unknown-linux.txt
	cat target/result/result.bench-memcmp.i586-unknown-linux.txt
	cat target/result/result.bench-memcmp.armv7.txt

result-memcpy:
	cat target/result/result.bench-memcpy.x86_64-unknown-linux.txt
	cat target/result/result.bench-memcpy.i686-unknown-linux.txt
	cat target/result/result.bench-memcpy.i586-unknown-linux.txt
	cat target/result/result.bench-memcpy.armv7.txt

result-memeq:
	cat target/result/result.bench-memeq.x86_64-unknown-linux.txt
	cat target/result/result.bench-memeq.i686-unknown-linux.txt
	cat target/result/result.bench-memeq.i586-unknown-linux.txt
	cat target/result/result.bench-memeq.armv7.txt

result-memmem:
	cat target/result/result.bench-memmem.x86_64-unknown-linux.txt
	cat target/result/result.bench-memmem.i686-unknown-linux.txt
	cat target/result/result.bench-memmem.i586-unknown-linux.txt
	cat target/result/result.bench-memmem.armv7.txt

result-memnechr:
	cat target/result/result.bench-memnechr.x86_64-unknown-linux.txt
	cat target/result/result.bench-memnechr.i686-unknown-linux.txt
	cat target/result/result.bench-memnechr.i586-unknown-linux.txt
	cat target/result/result.bench-memnechr.armv7.txt

result-memrchr:
	cat target/result/result.bench-memrchr.x86_64-unknown-linux.txt
	cat target/result/result.bench-memrchr.i686-unknown-linux.txt
	cat target/result/result.bench-memrchr.i586-unknown-linux.txt
	cat target/result/result.bench-memrchr.armv7.txt

result-memrnechr:
	cat target/result/result.bench-memrnechr.x86_64-unknown-linux.txt
	cat target/result/result.bench-memrnechr.i686-unknown-linux.txt
	cat target/result/result.bench-memrnechr.i586-unknown-linux.txt
	cat target/result/result.bench-memrnechr.armv7.txt

result-memrmem:
	cat target/result/result.bench-memrmem.x86_64-unknown-linux.txt
	cat target/result/result.bench-memrmem.i686-unknown-linux.txt
	cat target/result/result.bench-memrmem.i586-unknown-linux.txt
	cat target/result/result.bench-memrmem.armv7.txt

result-memset:
	cat target/result/result.bench-memset.x86_64-unknown-linux.txt
	cat target/result/result.bench-memset.i686-unknown-linux.txt
	cat target/result/result.bench-memset.i586-unknown-linux.txt
	cat target/result/result.bench-memset.armv7.txt

