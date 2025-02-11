#!/bin/bash
ulimit -Hn 2000000
ulimit -Sn 2000000
./target/debug/client client-options --url ws://localhost:8000 &
./target/debug/client client-options --url ws://localhost:8001 &
./target/debug/client client-options --url ws://localhost:8002 &
./target/debug/client client-options --url ws://localhost:8003 &
./target/debug/client client-options --url ws://localhost:8004 &