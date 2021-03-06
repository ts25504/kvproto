#!/bin/bash

cmd_exists () {
    which "$1" 1>/dev/null 2>&1
}

# install rust-protobuf if it's missing
if ! cmd_exists protoc-gen-rust; then
    echo "missing rust-protobuf, try to download/install it"
    cargo install protobuf
fi

cd proto

echo "generate rust code..."
ret=0

gogo_protobuf_url=github.com/gogo/protobuf
GOGO_ROOT=${GOPATH}/src/github.com/gogo/protobuf

# download gogproto code and install its binary if it's missing
if ! cmd_exists protoc-gen-gofast || [ ! -e "$GOGO_ROOT" ]; then
    echo "gogoproto code/generator missing, try to download/install it"
    go get ${gogo_protobuf_url}/proto
    go get ${gogo_protobuf_url}/protoc-gen-gofast
    go get ${gogo_protobuf_url}/gogoproto
fi

# add the bin path of gogoproto generator into PATH if it's missing
if ! cmd_exists protoc-gen-gofast; then
    for path in $(echo "${GOPATH}" | sed -e 's/:/ /g'); do
        gogo_proto_bin="${path}/bin/protoc-gen-gofast"
        if [ -e "${gogo_proto_bin}" ]; then
            export PATH=$(dirname "${gogo_proto_bin}"):$PATH
            break
        fi
    done
fi

protoc -I.:${GOGO_ROOT}:${GOGO_ROOT}/protobuf --rust_out ../src *.proto || ret=$?

echo "extern crate protobuf;" > ../src/lib.rs
for file in `ls *.proto`
    do
    base_name=$(basename $file ".proto")
    echo "pub mod $base_name;" >> ../src/lib.rs
done

if [[ $ret -ne 0 ]]; then
	exit $ret
fi
cd ..
cargo build
