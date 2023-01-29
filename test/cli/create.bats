#!/usr/bin/env bats

. test/common.sh

@test "rtbox create --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox create --help)"

  expected_output="$(cat <<EOF
Create a rtbox container

Usage: rtbox create [OPTIONS] <NAME>

Arguments:
  <NAME>  Container name, will also be used as part of the hostname

Options:
  -i, --image <IMAGE>  Image to use as base for the container
  -H, --home <HOME>    Set a custom HOME directory for the container
  -h, --help           Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
