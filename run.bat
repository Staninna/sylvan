@echo off

@REM Build
cargo bootimage

@REM Set PATH
set PATH=%PATH%;C:\Program Files\qemu

@REM Run
qemu-system-x86_64 -drive format=raw,file=target\sylvan\debug\bootimage-sylvan.bin
