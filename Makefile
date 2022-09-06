DST:=bin
DST_HTML:=$(DST)/html
override NAME:=agentifa-555nake
override NAME_CLIENT:=$(NAME)-client
override NAME_SERVER:=$(NAME)-server
SRC_ASSETS:=$(NAME_CLIENT)/assets
SRC_HTML:=$(NAME_CLIENT)/html
TGT:=wasm32-unknown-unknown
UUID?=$(shell uuidgen)
SRV_KEY:=$(UUID)

.PHONY: \
	all \
	build \
	build_client \
	build_cross \
	build_server \
	clean \
	deploy \
	deploy_client \
	deploy_server \
	deploy_wasm

all: deploy

build: build_client	

build_client:
	SRV_KEY=$(SRV_KEY) cargo build -p $(NAME_CLIENT) --release

build_cross:
	SRV_KEY=$(SRV_KEY) cargo build -p $(NAME_CLIENT) --target=$(TGT) --release

build_server:
	SRV_KEY=$(SRV_KEY) cargo build -p $(NAME_SERVER) --release

clean:
	rm -rf $(DST)

deploy: clean deploy_client deploy_server

deploy_client: build_client $(DST)
	cp -r $(SRC_ASSETS) $(DST)
	cp target/release/$(NAME_CLIENT) $(DST)

deploy_server: $(DST)
	docker build \
		--build-arg SRV_KEY=$(SRV_KEY) \
		--build-arg SRV_ADDR=$(SRV_ADDR) \
		--build-arg SRV_PORT=$(SRV_PORT) \
		--build-arg SRV_PROT=$(SRV_PROT) \
		-t agentifa-555nake-server \
		.

	docker run \
		--rm \
		--entrypoint cat \
		agentifa-555nake-server \
		target/release/$(NAME_SERVER) > bin/$(NAME_SERVER)

	chmod +x $(DST)/$(NAME_SERVER)

deplay_wasm: TGT:=wasm32-unknown-unknown
deploy_wasm: clean build_cross deploy_server
	wasm-bindgen \
		--out-dir $(DST_HTML) \
		--target web \
		target/$(TGT)/release/$(NAME_CLIENT).wasm

	wasm-opt \
		-O \
		-ol 100 \
		-s 100 \
		-o $(DST_HTML)/$(NAME_CLIENT)_opt.wasm \
		$(DST_HTML)/$(NAME_CLIENT)_bg.wasm

	mv $(DST_HTML)/$(NAME_CLIENT)_opt.wasm $(DST_HTML)/$(NAME_CLIENT)_bg.wasm
	cp -r $(SRC_ASSETS) $(DST_HTML)
	cp -r $(SRC_HTML)/* $(DST_HTML)

$(DST):
	mkdir -p bin
