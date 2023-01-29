#!/bin/true

_rtbox() {
    RTBOX_CMD="${RTBOX_CMD:-target/debug/rtbox}"

    "${RTBOX_CMD}" "$@"
}

assert_equal() {
    expected_output="$1"
    actual_output="$2"

    if [ "${expected_output}" != "${actual_output}" ]; then
        sdiff -ts \
            <(awk 'BEGIN{print "expected\n+======="} 1' <<<"${expected_output}") \
            <(awk 'BEGIN{print "actual\n-====="} 1' <<<"${actual_output}")
    fi

    return 0
}
