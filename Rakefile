# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("example.gemspec")

RbSys::ExtensionTask.new("example", GEMSPEC) do |ext|
  ext.lib_dir = "lib/example"
end

task default: :compile
