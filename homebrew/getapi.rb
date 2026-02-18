class Getapi < Formula
  desc "Guided, interactive walkthroughs for setting up developer API credentials"
  homepage "https://github.com/m2de/getapi"
  url "https://github.com/m2de/getapi/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "eb4a3b6666894628d8d66494c69eb5a1389749bc945bf9381924a399549995d7"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "getapi", shell_output("#{bin}/getapi --help")
  end
end
