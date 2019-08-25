TEST_DIR = ARGV[0]
REGULAR_MODE = "100644"

puts TEST_DIR
test_results = ["-- RESULTS --"]

# 01: ls
def ls_test(test_no, expect_fnames, test_results)
    begin
        Dir.each_child(TEST_DIR) do |fname|
            next if fname == '.' or fname == '..'
            puts fname
            expect_fnames.delete(fname)
            mode =  File.stat(TEST_DIR+"/"+fname).mode.to_s(8)
            if  mode != REGULAR_MODE  then
                raise "unexpected mode : #{fname} is #{mode}"
            end
        end
        if expect_fnames.length > 0 then
            raise "cannot find : #{expect_fnames.join(',')}"
        end
        test_results[test_no.to_i] = "#{test_no}: OK"
    rescue => e
        test_results[test_no.to_i] = "#{test_no}: NG => #{e.message}"
    end
end
expect_fnames = ["hello.txt"]
ls_test("01", expect_fnames, test_results)

# 02: touch
  new_fname = "test.txt"
  begin
    fp = File.open(TEST_DIR+"/"+new_fname, File::CREAT, 0644)
    fp.close
    test_results[2] = "02: OK"
  rescue =>e
    test_results[2] = "02: NG => #{e.message}"
  end

# 03: ls
expect_fnames = ["hello.txt", new_fname]
ls_test("03", expect_fnames, test_results)


# Results
puts test_results


