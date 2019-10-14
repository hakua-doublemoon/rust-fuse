require 'color_echo'
require './test_lib.rb'

TEST_DIR = ARGV[0]

puts TEST_DIR
test_results = ["-- RESULTS --"]
fs_test = FsTest::new(TEST_DIR);
#STDIN.gets.chomp

fs_test.delete_test("01", test_results, "test.txt")

expect_fnames = ["hello.txt"]
fs_test.ls_test("02", test_results, expect_fnames)

trg_fname1 = "hello.txt"
expect_str1 = " noon\n"
fs_test.read_test("03", test_results, trg_fname1, expect_str1)

fs_test.create_test("04", test_results, "test01.txt")
fs_test.create_test("05", test_results, "test02.txt")
fs_test.create_test("06", test_results, "test03.txt")
fs_test.write_test("07", test_results, "test01.txt", "01", File::RDWR|File::TRUNC)
fs_test.write_test("08", test_results, "test02.txt", "02", File::RDWR|File::TRUNC)
fs_test.delete_test("09", test_results, "test02.txt")
fs_test.read_test("10", test_results, "test01.txt", "01")
fs_test.double_open_test("11", test_results, "test01.txt", File::RDWR|File::TRUNC)

#####################################
# Results
CE.pickup(/^.+RESULTS.+$/, foreground=:green)
CE.pickup(/OK/, foreground=:green)
CE.pickup(/NG.+$/, foreground=:red)
puts test_results


