# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "example"
  spec.version = "0.0.1"
  spec.summary = "An example"
  spec.files = Dir["lib/**/*.rb"].concat(Dir["ext/example/src/**/*.rs"]) << "ext/example/Cargo.toml" << "Cargo.toml" << "Cargo.lock" << "README.rdoc"
  spec.extensions = ["ext/example/Cargo.toml"]
  spec.bindir = "bin"
  spec.executables << "example_rust_loop_no_int" << "example_rust_loop_int" << "example_ruby_loop_delayed_int" << "example_no_int_block" << "example_int_block"
  spec.rdoc_options = ["--main", "README.rdoc", "--charset", "utf-8", "--exclude", "ext/"]
  spec.extra_rdoc_files = ["README.rdoc"]
  spec.authors = ["Mat Sadler"]
  spec.email = ["mat@sourcetagsandcodes.com"]
  spec.license = "MIT"

  spec.requirements = ["Rust >= 1.61.0"]
  spec.required_ruby_version = ">= 2.7.0"
  spec.required_rubygems_version = ">= 3.3.26"

  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "rb_sys", "~> 0.9"
end
