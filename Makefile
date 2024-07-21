.PHONY: fmt
fmt:
	cd os ; cargo fmt; cd ../user; cargo fmt; cd .. 

.PHONY: build_apps
build_apps:
	cd user ; make build ; cd ..

.PHONY: run_os
run_os:
	cd os ; make clean run ; cd ..

.PHONY: clean
clean:
	cd os ; make clean ; cd ../user; make clean; cd ..
