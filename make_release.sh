#! /bin/bash

VERSION=$1

if [ -z $1 ]; then 
  echo "Need to pass a version to the script"
  exit
fi 

echo "Buiding for version ${VERSION}" 

#Build the binaries
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:armv7-musleabihf cargo build --release
cargo build --target=x86_64-pc-windows-gnu --release
cargo build --release

#Temp dir for zips/bzips
if [ ! -d target/zips ]; then 
  echo "Making zip output dir target/zips"
  mkdir target/zips
fi 

rm -f target/zips/*

echo "Packaging up releases into bzips and zips"
echo

#ARMv7
echo "Building ARM V7......"
tar -cjf target/zips/hsctl_armv7_rpi_linux-${VERSION}.bz2  -C target/armv7-unknown-linux-musleabihf/release hsctl
echo

#Linux
echo "Building linux...."
tar -cjf target/zips/hsctl_linux_x86_64-${VERSION}.bz2 -C target/x86_64-unknown-linux-musl/release hsctl
echo

#OSX
echo "Building OSX...."
tar -cjf target/zips/hsctl_osx_mojave-${VERSION}.bz2  -C target/release hsctl
echo

#Windows
echo "Building windows..."
cp target/x86_64-pc-windows-gnu/release/hsctl.exe target/zips
cd target/zips
zip hsctl_windows_x86_64-${VERSION}.zip hsctl.exe
