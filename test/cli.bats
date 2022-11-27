#!/usr/bin/env bats

@test "tbox create -i fedora:latest test1" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(tbox create -i fedora:latest "${tbox_name}")"

  expected_output="$(cat <<EOF
podman create --name ${tbox_name} fedora:latest
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
