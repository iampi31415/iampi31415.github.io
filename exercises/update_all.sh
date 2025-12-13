#!/bin/bash

for CRATE in *
do
  if [ -d "${CRATE}" ]; then
    cd $CRATE;
    echo ${CRATE};
    cargo +nightly update --breaking -Z unstable-options;
    cd ..;
  fi
done
