#!/bin/bash

TEST_DIR=/tmp/hello_dir

if [ ! -e $TEST_DIR ]; then
   mkdir $TEST_DIR 
fi

export RUST_BACKTRACE=1 
(../target/debug/hello_fs $TEST_DIR >&3) 3> >(./colorize.pl >&1) &

sleep 1
echo "---- TEST START !! ----"
echo "[FIRST]"
ruby test01.rb $TEST_DIR
echo "[SECOND]"
ruby test02.rb $TEST_DIR

#ls -l /tmp/hello_dir

#ls -l "/tmp/hello_dir/hello.txt"
#cat "/tmp/hello_dir/hello.txt"
#ls -l "/tmp/hello_dir/test.txt"
#cat "/tmp/hello_dir/test.txt"

sudo umount $TEST_DIR
