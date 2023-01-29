#!/usr/bin/env bats

. test/common.sh

@test "rtbox export --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox export --help)"

  expected_output="$(cat <<EOF
Export an application, service or binary from a rtbox container to the host

Usage: rtbox export [OPTIONS] --container <CONTAINER>

Options:
  -c, --container <CONTAINER>        Container from where to export the application
  -b, --binary <BINARY>              Path to an executable that will be exported (must exist inside the container)
  -s, --service-unit <SERVICE_UNIT>  Service unit name that will be exported (must exist inside the container)
  -a, --application <APPLICATION>    Desktop application name that will be exported (must exist inside the container)
  -h, --help                         Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
