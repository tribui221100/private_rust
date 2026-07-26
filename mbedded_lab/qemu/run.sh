#!/bin/bash

source "$(dirname "$0")/env.sh"

qemu-system-aarch64 \
    -M ${MACHINE} \
    -cpu ${CPU} \
    -m ${RAM} \
    -smp ${SMP} \
    -nographic