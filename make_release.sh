#! /bin/bash

#Temp dir for zips/bzips
if [ ! -d target/zips ]; then 
  echo "Making zip output dir target/zips"
  mkdir target/zips
fi 

rm -f target/zips/*

#ARMv7
echo "Building ARM V7......"
tar -cjf target/zips/hsctl_armv7_rpi_linux.bz2  -C target/armv7-unknown-linux-musleabihf/release hsctl
echo

#Linux
echo "Building linux...."
tar -cjf target/zips/hsctl_linux_x86_64.bz2 -C target/x86_64-unknown-linux-musl/release hsctl
echo

#OSX
echo "Building OSX...."
tar -cjf target/zips/hsctl_osx_mojave.bz2  -C target/release hsctl
echo

#Windows
echo "Building windows..."
cp target/x86_64-pc-windows-gnu/release/hsctl.exe target/zips
cd target/zips
zip hsctl_windows_x86_64.zip hsctl.exe
