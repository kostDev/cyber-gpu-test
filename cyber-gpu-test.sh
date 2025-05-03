#!/bin/sh

rm -f /tmp/cyb83rdo6/cyber-gpu-test
mkdir -p /tmp/cyb83rdo6/

if [ -f /userdata/bin/cyber-gpu-test ]; then
  cp /userdata/bin/cyber-gpu-test /tmp/cyb83rdo6/
  chmod +x /tmp/cyb83rdo6/cyber-gpu-test
  /tmp/cyb83rdo6/cyber-gpu-test
else
  echo "ERROR: cyber-gpu-test not found!"
fi