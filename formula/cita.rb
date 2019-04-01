# CITA Formula
class Cita < Formula
  desc "A high performance blockchain for enterprise users."
  homepage "https://github.com/cryptape/cita"

  version "0.22"
  url "https://github.com/cryptape/cita/releases/download/v0.22.0/cita_secp256k1_sha3.tar.gz"
  sha256 "3f28abc41f98b4e2dc5122ad72d8032a1fb09751d148ab6483e9c2e50af7800d"
  resource "cita" do
    url "https://github.com/cryptape/cita.git",
        :branch => "develop"
    # Completing at v0.23
    # :tag => ""
    # :revision => ""
  end

  def install
    libexec.install Dir["*"]

    resource("cita").stage do
      system  "cp", "-r", "env.sh", "#{libexec}/bin/cita-env"
      system  "cp", "-r", "scripts/cita", "#{libexec}/bin/cita"
      system  "cp", "-r", "scripts/cita_config.sh", "#{libexec}/bin/cita-config"
    end

    bin.install_symlink Dir["#{libexec}/bin/cita"]
    bin.install_symlink Dir["#{libexec}/bin/cita-env"]
    bin.install_symlink Dir["#{libexec}/bin/cita-config"]
  end

  def caveats; <<~EOS
     By default, binaries installed by cita will be placed into:
     #{libexec}
     Usage: cita_commander <command> <node> [options]
     where <command> is one of the following:
         { help | create | port | setup | start | stop | restart
           ping | top | backup | clean | logs | logrotate }
     Run `cita help` for more detailed information.
     happy hacking!
  EOS
  end
end
