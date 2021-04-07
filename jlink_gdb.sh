#!/bin/sh

# Start a jlink gdb server with the correct flags for imxrt1062evk

JLinkGDBServer -device MIMXRT1064xxx6A -if swd -speed 100000
