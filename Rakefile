def _in(lang, shell)
  base_dir = File.expand_path("..", __FILE__)
  Dir.glob("#{base_dir}/#{lang}/*") do |dir|
    Dir.chdir(dir) { sh(shell) }
  end
end

namespace :test do
  task(:rust) { _in(:rust, "cargo test") }
  task(:bash) { _in(:bash, "bats *_test.sh") }
  task(:go) { _in(:go, "go test") }
end

task default: [
  #"test:rust",
  #"test:bash",
  "test:go"
]
