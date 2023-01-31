#!/usr/bin/env bats

. test/common.sh

@test "rtbox init --help" {

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox init --help)"

  expected_output="$(cat <<EOF
Used to initialize rtbox containers

Usage: rtbox init --gid <GID> --home <HOME> --shell <SHELL>

Options:
      --gid <GID>
      --home <HOME>
      --shell <SHELL>
  -h, --help           Print help information
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox init | errors out when not running as PID1" {

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox init --gid $(id -g) --home $HOME --shell $SHELL)"

  expected_output="$(cat <<EOF
{"Error":{"command":"init","message":"this is only supposed to be run as the init system of a container","root_cause":"we are not running as PID 1"}}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox init | will start up when running as PID1" {

  tbox_name="test-$(date +%s)"
  tbox_output="$(_podman run \
    --rm -it \
    --security-opt label=disable \
    -v $PWD/target/debug:/workdir \
    rtbox-tester:latest \
    /workdir/rtbox init \
        --home $HOME --shell $SHELL --gid $(id -g)
  )"

  expected_output="$(cat <<EOF
{"Init":null}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}
