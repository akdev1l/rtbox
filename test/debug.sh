#!/bin/bash

./util/bats --report-formatter junit -Tro . test
