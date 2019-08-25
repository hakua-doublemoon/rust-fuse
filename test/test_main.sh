#!/bin/bash

TEST_DIR=/tmp/hello_dir

../target/debug/hello_fs $TEST_DIR &

sleep 1
ruby test01.rb $TEST_DIR
sudo umount $TEST_DIR
