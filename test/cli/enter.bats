#!/usr/bin/env bats

. test/common.sh

@test "rtbox enter --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox enter --help)"

  expected_output="$(cat <<EOF
Enter into a new shell session inside a rtbox container

Usage: rtbox enter <NAME>

Arguments:
  <NAME>  Container to enter into

Options:
  -h, --help  Print help information
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}
