originalDir = pwd

cd "$(dirname "$0")"
cd controller && cargo build
cd ..
cd map && cargo build
cd ..
cd reduce && cargo build
cd ..

cd $originalDir