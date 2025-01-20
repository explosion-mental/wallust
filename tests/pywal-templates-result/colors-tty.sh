#!/bin/sh
[ "${TERM:-none}" = "linux" ] && \
    printf '%b' '\e]P0000000
                 \e]P1010000
                 \e]P2020000
                 \e]P3030000
                 \e]P4040000
                 \e]P5050000
                 \e]P6060000
                 \e]P7070000
                 \e]P8080000
                 \e]P9090000
                 \e]PA0A0000
                 \e]PB0B0000
                 \e]PC0C0000
                 \e]PD0D0000
                 \e]PE0E0000
                 \e]PF0F0000
                 \ec'
