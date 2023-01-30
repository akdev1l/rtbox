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

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list -- with empty list" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list)"

  expected_output="$(cat <<EOF
{"List":[]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list --all true -- with empty list" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list --all true)"

  expected_output="$(cat <<EOF
{"List":[]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list --all false should match rtbox list" {

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list --all false)"

  expected_output="$(_rtbox list)"

  assert_equal "${expected_output}" "${tbox_output}"
}
