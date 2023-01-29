#!/usr/bin/env bats

. test/common.sh

@test "rtbox rm --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox rm --help)"

  expected_output="$(cat <<EOF
Remove a rtbox container

Usage: rtbox rm [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Container to remove

Options:
  -f, --force <FORCE>  Remove container even if it is currently running [possible values: true, false]
  -a, --all <ALL>      Remove all rtbox containers [possible values: true, false]
  -h, --help           Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
