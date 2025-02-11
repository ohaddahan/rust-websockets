#!/bin/bash
function create() {
  for i in `seq 0 10`; do sudo ifconfig lo0 alias 10.0.0.$i/32 up  ; done
}

function remove() {
  for i in `seq 0 10`; do sudo ifconfig lo0 -alias 10.0.0.$i  ; done
}

