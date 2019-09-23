require 'color_echo'

TEST_DIR = ARGV[0]
REGULAR_MODE = "100644"

puts TEST_DIR
test_results = ["-- RESULTS --"]

# 01: ls
def ls_test(test_no, expect_fnames, test_results)
    begin
        fns = [""]
        Dir.each_child(TEST_DIR) do |fname|
            next if fname == '.' or fname == '..'
            fns << fname
            expect_fnames.delete(fname)
            mode =  File.stat(TEST_DIR+"/"+fname).mode.to_s(8)
            if  mode != REGULAR_MODE  then
                raise "unexpected mode : #{fname} is #{mode}"
            end
        end
        puts fns
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

# 04: read
def read_test(test_no, trg_fname, expect_str, test_results)
  begin
    str = "hello world!!"
    File.open(TEST_DIR+"/"+trg_fname, File::RDONLY) do |fp|
        fp.read(nil, str)
        str.delete!("\0")
        #puts str
        puts ""
        p str.unpack("c*")
        p expect_str.unpack("c*")
        fp.close
    end
    if  str != expect_str  then
        raise "unexpected string: #{str}"
    end
    test_results[test_no.to_i] = "#{test_no}: OK"
  rescue =>e
    test_results[test_no.to_i] = "#{test_no}: NG => #{e.message}"
  end
end
read_test("04", "hello.txt", "hello hello world\n", test_results)

# 05: write[1]
trg_fname1 = "hello.txt"
expect_str1 = " noon\n"
begin
    File.open(TEST_DIR+"/"+trg_fname1, File::RDWR|File::TRUNC) do |fp|
        fp.write(expect_str1)
        fp.close()
    end
    test_results[5] = "05: OK"
rescue => e
    test_results[5] = "05: NG => #{e.message}"
end

# 05: write[2]
trg_fname2 = "test.txt"
expect_str2 = <<~'EOS'
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
        color of sky; song of sky; sound of sky
    EOS
begin
    File.open(TEST_DIR+"/"+trg_fname2, File::RDWR|File::TRUNC) do |fp|
        #p fp.path
        fp.write("color")
        fp.close()
    end
    File.open(TEST_DIR+"/"+trg_fname2, File::RDWR|File::APPEND) do |fp|
        fp.write(" of sky; song of sky; sound of sky\n")
        fp.write("color of sky; song of sky; sound of sky\n")
        fp.write("color of sky; song of sky; sound of sky\n")
        fp.write("color of sky; song of sky; sound of sky\n")
        fp.close()
    end
    test_results[6] = "06: OK"
rescue => e
    test_results[6] = "06: NG => #{e.message}"
end
read_test("07", trg_fname1, expect_str1, test_results)
read_test("08", trg_fname2, expect_str2, test_results)

#####################################
# Results
CE.pickup(/^.+RESULTS.+$/, foreground=:green)
CE.pickup(/OK/, foreground=:green)
CE.pickup(/NG.+$/, foreground=:red)
puts test_results


