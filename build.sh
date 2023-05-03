#!/bin/sh
set -e
LIB=$( cargo build $@ --message-format=json |
       jq "select( any( .target?.kind?[]?; . == \"cdylib\" ) and .filenames ) | .filenames[] | select( test( \".*.so\" ) )" |
       tr -d '"' )
cp $LIB $(dirname $LIB)"/nbt.so"
export PYTHONPATH=$(dirname $LIB)
python