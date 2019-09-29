#require 'color_echo'

REGULAR_MODE = "100644"

# 01: ls
class FsTest
    def initialize(test_dir)
        @test_dir = test_dir
    end

    def ls_test(test_no, test_results, expect_fnames)
      begin
        fns = [""]
        Dir.each_child(@test_dir) do |fname|
            next if fname == '.' or fname == '..'
            fns << fname
            expect_fnames.delete(fname)
            mode =  File.stat(@test_dir+"/"+fname).mode.to_s(8)
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

    def create_test(test_no, test_results, new_fname)
      begin
        fp = File.open(@test_dir+"/"+new_fname, File::CREAT, 0644)
        fp.close
        test_results[test_no.to_i] = "#{test_no}: OK"
      rescue =>e
        test_results[test_no.to_i] = "#{test_no}: NG => #{e.message}"
      end
    end

    def read_test(test_no, test_results, trg_fname, expect_str)
      begin
        str = ""
        File.open(@test_dir+"/"+trg_fname, File::RDONLY) do |fp|
            fp.read(nil, str)
            str.delete!("\0")
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

    def write_test(test_no, test_results, trg_fname, expect_str, mode)
      begin
        File.open(@test_dir+"/"+trg_fname, mode) do |fp|
        fp.write(expect_str)
        fp.close()
      end
        test_results[test_no.to_i] = "#{test_no}: OK"
      rescue =>e
        test_results[test_no.to_i] = "#{test_no}: NG => #{e.message}"
      end
    end

    def delete_test(test_no, test_results, trg_fname)
      begin
        fp = File.delete(@test_dir+"/"+trg_fname)
        test_results[test_no.to_i] = "#{test_no}: OK"
      rescue =>e
        test_results[test_no.to_i] = "#{test_no}: NG => #{e.message}"
      end
    end

end
