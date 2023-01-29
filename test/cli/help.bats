#!/usr/bin/env bats

. test/common.sh

@test "rtbox --help" {   

  tbox_name="test-$(date +%s)"
  tbox_output="$(_rtbox  --help)"

  expected_output="$(cat <<EOF
A rust implementation of toolbx

Usage: rtbox [OPTIONS] <COMMAND>

Commands:
  create  Create a rtbox container
  rm      Remove a rtbox container
  rmi     Remove a rtbox container image
  run     Execute a command inside a rtbox container
  enter   Enter into a new shell session inside a rtbox container
  list    List all rtbox containers
  export  Export an application, service or binary from a rtbox container to the host
  help    Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose <VERBOSE>      
  -y, --assume-yes             Automatically answer yes to all questions
      --log-level <LOG_LEVEL>  Set the logging level [default: info] [possible values: info, warn, error, debug, all]
  -f, --format <FORMAT>        Set the output format [default: human] [possible values: json, human]
      --dry-run                Do not actually execute API calls
  -h, --help                   Print help information
EOF
)"

  [ "${tbox_output}" = "${expected_output}" ]
}
