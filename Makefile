
bench_nms = bench-memchr bench-memcmp bench-memcpy bench-memeq bench-memmem bench-memset

target_base = x86_64-unknown-linux i686-unknown-linux i586-unknown-linux

define build-templ =
target/stamp/stamp.build.$(1).$(2):
	@mkdir -p target/stamp
	make -f makefile.build BENCH_NM=$(1) TARGET_GNU=$(2)-gnu TARGET_MUSL=$(2)-musl bench-build-all
	@touch target/stamp/stamp.build.$(1).$(2)

endef

define build-armv7-templ =
target/stamp/stamp.build.$(1).armv7:
	@mkdir -p target/stamp
	make -f makefile.build BENCH_NM=$(1) bench-build-arm-all
	@touch target/stamp/stamp.build.$(1).armv7

endef

define bench-templ =
target/stamp/stamp.bench.$(1).$(2):
	@mkdir -p target/stamp
	@mkdir -p target/result
	make -f makefile.build BENCH_NM=$(1) TARGET_GNU=$(2)-gnu TARGET_MUSL=$(2)-musl bench-all
	make -f makefile.build report | tee target/result/result.$(1).$(2).txt
	@touch target/stamp/stamp.bench.$(1).$(2)

endef

define bench-armv7-templ =
target/stamp/stamp.bench.$(1).armv7:
	@mkdir -p target/stamp
	@mkdir -p target/result
	make -f makefile.build BENCH_NM=$(1) bench-arm-all
	make -f makefile.build report | tee target/result/result.$(1).armv7.txt
	@touch target/stamp/stamp.bench.$(1).armv7

endef


all:

build-all: $(foreach bnm,$(bench_nms),$(foreach tbm,$(target_base),target/stamp/stamp.build.$(bnm).$(tbm)))

bench-all: $(foreach bnm,$(bench_nms),$(foreach tbm,$(target_base),target/stamp/stamp.bench.$(bnm).$(tbm)))

build-arm-all: $(foreach bnm,$(bench_nms),target/stamp/stamp.build.$(bnm).armv7)

bench-arm-all: $(foreach bnm,$(bench_nms),target/stamp/stamp.bench.$(bnm).armv7)

clean:
	@rm -fr target

$(foreach bnm,$(bench_nms),$(eval $(foreach tb,$(target_base),$(eval $(call build-templ,$(bnm),$(tb))))))

$(foreach bnm,$(bench_nms),$(eval $(foreach tb,$(target_base),$(eval $(call bench-templ,$(bnm),$(tb))))))

$(foreach bnm,$(bench_nms),$(eval $(call build-armv7-templ,$(bnm))))

$(foreach bnm,$(bench_nms),$(eval $(call bench-armv7-templ,$(bnm))))


clean-memcmp:
	@rm -f target/stamp/stamp.build.bench-memcmp.*
	@rm -f target/stamp/stamp.bench.bench-memcmp.*

clean-memeq:
	@rm -f target/stamp/stamp.build.bench-memeq.*
	@rm -f target/stamp/stamp.bench.bench-memeq.*

clean-memmem:
	@rm -f target/stamp/stamp.build.bench-memeq.*
	@rm -f target/stamp/stamp.bench.bench-memeq.*

result-memcmp:
	cat target/result/result.bench-memcmp.x86_64-unknown-linux.txt
	cat target/result/result.bench-memcmp.i686-unknown-linux.txt
	cat target/result/result.bench-memcmp.i586-unknown-linux.txt
	cat target/result/result.bench-memcmp.armv7.txt

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

