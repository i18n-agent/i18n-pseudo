class I18nPseudo < Formula
  desc "Pseudo-translate i18n files for testing internationalization"
  homepage "https://github.com/i18n-agent/i18n-pseudo"
  version "0.2.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/i18n-agent/i18n-pseudo/releases/download/v#{version}/i18n-pseudo-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_MACOS_ARM64"
    else
      url "https://github.com/i18n-agent/i18n-pseudo/releases/download/v#{version}/i18n-pseudo-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_MACOS_X86_64"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/i18n-agent/i18n-pseudo/releases/download/v#{version}/i18n-pseudo-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_ARM64"
    else
      url "https://github.com/i18n-agent/i18n-pseudo/releases/download/v#{version}/i18n-pseudo-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_SHA256_LINUX_X86_64"
    end
  end

  def install
    bin.install "i18n-pseudo"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/i18n-pseudo --version")
  end
end
