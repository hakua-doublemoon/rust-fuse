require 'color_echo'
require './test_lib.rb'

TEST_DIR = ARGV[0]

puts TEST_DIR
test_results = ["-- RESULTS --"]
fs_test = FsTest::new(TEST_DIR);

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

=begin
expect_fnames = ["hello.txt"]

fs_test.ls_test("01", test_results, expect_fnames)

# 02: touch
new_fname = "test.txt"
fs_test.create_test("02", test_results, new_fname)

# 03: ls

# 04: read
fs_test.read_test("04", test_results, "hello.txt", "hello hello world\n")

# 05: write[1]
trg_fname1 = "hello.txt"
expect_str1 = " noon\n"
fs_test.write_test("05", test_results, trg_fname1, expect_str1, File::RDWR|File::TRUNC)

# 05: write[2]
trg_fname2 = "test.txt"
expect_str2 = <<~'EOS'
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
    EOS
fs_test.write_test("06", test_results, trg_fname2, expect_str2[0...10], File::RDWR|File::TRUNC)
fs_test.write_test("07", test_results, trg_fname2, expect_str2[10..-1], File::RDWR|File::APPEND)

fs_test.read_test("09", test_results, trg_fname2, expect_str2)
=end

#####################################
# Results
CE.pickup(/^.+RESULTS.+$/, foreground=:green)
CE.pickup(/OK/, foreground=:green)
CE.pickup(/NG.+$/, foreground=:red)
puts test_results


