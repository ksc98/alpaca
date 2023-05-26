#!/bin/bash

docker exec -i dragonfly redis-cli <<EOF
MULTI
ZADD buy_list 1685055951 AAPL
ZADD buy_list 1685055951 GOOGL
ZADD buy_list 1685055951 MSFT
EXEC
EOF