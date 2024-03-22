# frozen_string_literal: true

begin
  /(?<ruby_version>\d+\.\d+)/ =~ RUBY_VERSION
  require_relative "example/#{ruby_version}/example"
rescue LoadError
  begin
    require_relative "example/example"
  rescue LoadError # Cargo Builder in RubyGems < 3.4.6 doesn't install to dir
    require_relative "example.so"
  end
end
