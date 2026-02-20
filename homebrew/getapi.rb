class Getapi < Formula
  desc "Guided, interactive walkthroughs for setting up developer API credentials"
  homepage "https://github.com/m2de/getapi"
  version "1.1.0"
  license "MIT"

  on_arm do
    url "https://github.com/m2de/getapi/releases/download/v1.1.0/getapi-aarch64-apple-darwin.tar.gz"
    sha256 "e19f14173197c32048a7565b58c0e9d07f30fe36fe76994cf24d7ce4dcf9641c"
  end

  on_intel do
    url "https://github.com/m2de/getapi/releases/download/v1.1.0/getapi-x86_64-apple-darwin.tar.gz"
    sha256 "a676461ebe41a6ecce65d5050d8ea4e947ddd58f1cdead138e8bfa062f719681"
  end

  def install
    bin.install "getapi"
  end

  test do
    assert_match "getapi", shell_output("#{bin}/getapi --help")
  end
end
