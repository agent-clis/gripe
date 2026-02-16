class Gripe < Formula
  desc "Submit structured feedback as GitHub issues"
  homepage "https://github.com/agent-clis/gripe"
  version "VERSION"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/agent-clis/gripe/releases/download/v#{version}/gripe-darwin-arm64.tar.gz"
      sha256 "SHA256_DARWIN_ARM64"
    else
      url "https://github.com/agent-clis/gripe/releases/download/v#{version}/gripe-darwin-amd64.tar.gz"
      sha256 "SHA256_DARWIN_AMD64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/agent-clis/gripe/releases/download/v#{version}/gripe-linux-arm64.tar.gz"
      sha256 "SHA256_LINUX_ARM64"
    else
      url "https://github.com/agent-clis/gripe/releases/download/v#{version}/gripe-linux-amd64.tar.gz"
      sha256 "SHA256_LINUX_AMD64"
    end
  end

  def install
    bin.install Dir["gripe-*"].first => "gripe"
  end

  test do
    assert_match "gripe", shell_output("#{bin}/gripe --help")
  end
end
