#!/usr/bin/env bats

. test/common.sh

@test "rtbox rmi --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox rmi --help)"

  expected_output="$(cat <<EOF
Remove a rtbox container image

Usage: rtbox rmi [OPTIONS] [IMAGE_NAME]

Arguments:
  [IMAGE_NAME]  Name of image to remove

Options:
  -a, --all    Remove all rtbox container images
  -f, --force  Remove rtbox container images even if running containers are using it
  -h, --help   Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
