task :default do
  base_dir = File.expand_path("..", __FILE__)
  Dir.glob("#{base_dir}/rust/*") do |dir|
    Dir.chdir(dir) { sh "cargo test" }
  end
end
