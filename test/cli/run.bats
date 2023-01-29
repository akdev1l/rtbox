#!/usr/bin/env bats

. test/common.sh

@test "rtbox run --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox run --help)"

  expected_output="$(cat <<EOF
Execute a command inside a rtbox container

Usage: rtbox run --container <CONTAINER> [CMD]...

Arguments:
  [CMD]...  Command to execute

Options:
  -c, --container <CONTAINER>  Container name
  -h, --help                   Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
