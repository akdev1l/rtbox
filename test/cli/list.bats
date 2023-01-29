#!/usr/bin/env bats

. test/common.sh

@test "rtbox list --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list --help)"

  expected_output="$(cat <<EOF
List all rtbox containers

Usage: rtbox list [OPTIONS]

Options:
  -a, --all <ALL>  Show all rtbox containers even if they not actively running [possible values: true, false]
  -h, --help       Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
