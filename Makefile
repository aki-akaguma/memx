
#bench_nms = bench-memchr bench-memcmp bench-memcpy bench-memeq bench-memmem bench-memrchr bench-memrmem bench-memset
bench_nms = bench-memrmem

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


clean-memchr:
	@rm -f target/stamp/stamp.build.bench-memchr.*
	@rm -f target/stamp/stamp.bench.bench-memchr.*

clean-memcmp:
	@rm -f target/stamp/stamp.build.bench-memcmp.*
	@rm -f target/stamp/stamp.bench.bench-memcmp.*

clean-memcpy:
	@rm -f target/stamp/stamp.build.bench-memcpy.*
	@rm -f target/stamp/stamp.bench.bench-memcpt.*

clean-memeq:
	@rm -f target/stamp/stamp.build.bench-memeq.*
	@rm -f target/stamp/stamp.bench.bench-memeq.*

clean-memmem:
	@rm -f target/stamp/stamp.build.bench-memmem.*
	@rm -f target/stamp/stamp.bench.bench-memmem.*

clean-memrchr:
	@rm -f target/stamp/stamp.build.bench-memrchr.*
	@rm -f target/stamp/stamp.bench.bench-memrchr.*

clean-memrmem:
	@rm -f target/stamp/stamp.build.bench-memrmem.*
	@rm -f target/stamp/stamp.bench.bench-memrmem.*

clean-memset:
	@rm -f target/stamp/stamp.build.bench-memset.*
	@rm -f target/stamp/stamp.bench.bench-memset.*


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

result-memrchr:
	cat target/result/result.bench-memrchr.x86_64-unknown-linux.txt
	cat target/result/result.bench-memrchr.i686-unknown-linux.txt
	cat target/result/result.bench-memrchr.i586-unknown-linux.txt
	cat target/result/result.bench-memrchr.armv7.txt

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

