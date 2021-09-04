cp ../sapp-jsutils/js/*.js .
mkdir -p target/wasm_demo
cd target/wasm_demo
cp ../../*.js .
cp ../../quad_indexed_db/*.js .
cp ../../index.html .
cp ../wasm32-unknown-unknown/release/solar_sailors.wasm .
python3 -m http.server