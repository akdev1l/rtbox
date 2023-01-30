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

@test "rtbox list | with empty list" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list)"

  expected_output="$(cat <<EOF
{"List":[]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list --all true | with empty list" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list --all true)"

  expected_output="$(cat <<EOF
{"List":[]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list --all false | should match rtbox list" {

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox list --all false)"

  expected_output="$(_rtbox list)"

  assert_equal "${expected_output}" "${tbox_output}"
}

@test "rtbox list --all false | matches plain list with one container running" {

  tbox_name="test-$(date +%s)"

  container="$(_podman --url 'unix:/var/run/docker.sock' run \
    --name test-container \
    --label com.github.containers.toolbox=true \
    --detach --rm \
    rtbox-tester:latest sleep 5s)"

  tbox_output="$(_rtbox list --all false)"
  expected_output="$(_rtbox list)"

  assert_equal "${expected_output}" "${tbox_output}"

  _podman kill "${container}"
}

@test "rtbox list | with one container running" {

  tbox_name="test-$(date +%s)"

  container_name="test-container-$(date +%s)"

  _podman --url 'unix:/var/run/docker.sock' run \
    --name "${container_name}" \
    --label com.github.containers.toolbox=true \
    --detach --rm \
    rtbox-tester:latest sleep 5s

  tbox_output="$(_rtbox list --all false)"
  expected_output="$(cat <<EOF
{"List":[{"name":"${container_name}","image":"localhost/rtbox-tester:latest"}]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"

  _podman kill "${container_name}"
}

@test "rtbox list | with two containers running" {

  containers=(
    "test-container-$(date +%s)-1"
    "test-container-$(date +%s)-2"
  )

  _podman run \
    --name "${containers[0]}" \
    --label com.github.containers.toolbox=true \
    --detach --rm \
    rtbox-tester:latest sleep 5s

  _podman run \
    --name "${containers[1]}" \
    --label com.github.containers.toolbox=true \
    --detach --rm \
    rtbox-tester:latest sleep 5s

  tbox_output="$(_rtbox list)"
  expected_output="$(cat <<EOF
{"List":[{"name":"${containers[0]}","image":"localhost/rtbox-tester:latest"},{"name":"${containers[1]}","image":"localhost/rtbox-tester:latest"}]}
EOF
)"

  assert_equal "${expected_output}" "${tbox_output}"

  _podman kill "${containers[@]}"
}
