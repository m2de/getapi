class Getapi < Formula
  desc "Guided, interactive walkthroughs for setting up developer API credentials"
  homepage "https://github.com/m2de/getapi"
  version "1.0.0"
  license "MIT"

  on_arm do
    url "https://github.com/m2de/getapi/releases/download/v1.0.0/getapi-aarch64-apple-darwin.tar.gz"
    sha256 "20e0baf8ffd4b9c559911e1ff926aafb167e48445456aead9eec18c67ad7fa15"
  end

  on_intel do
    url "https://github.com/m2de/getapi/releases/download/v1.0.0/getapi-x86_64-apple-darwin.tar.gz"
    sha256 "0258c369d1687d481f34a5638b19068a5238c09345505e9993ab1ecf3694df28"
  end

  def install
    bin.install "getapi"
  end

  test do
    assert_match "getapi", shell_output("#{bin}/getapi --help")
  end
end
