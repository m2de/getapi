class Getapi < Formula
  desc "Guided, interactive walkthroughs for setting up developer API credentials"
  homepage "https://github.com/m2de/getapi"
  url "https://github.com/m2de/getapi/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "getapi", shell_output("#{bin}/getapi --help")
  end
end
