#!/bin/bash

RTBOX_CMD=target/release/rtbox

./util/bats test/*/**.bats
