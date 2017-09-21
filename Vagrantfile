# -*- mode: ruby -*-
# vi: set ft=ruby :

$set_environment_variables = <<SCRIPT
tee "/etc/profile.d/cita_boot.sh" > "/dev/null" <<EOF
export USING_VAGRANT=1
export VAGRANT_DATA_PATH=/home/ubuntu/citadata
EOF
SCRIPT

Vagrant.configure(2) do |config|
  config.vm.box = "cryptape/easy_cita"
  config.vm.box_version = "0.0.1"
  
  config.ssh.shell = "bash -c 'BASH_ENV=/etc/profile exec bash'"

  config.vm.provider "virtualbox" do |v|
        v.memory = 4096
        v.cpus = 4
  end

  config.vm.provision "shell", inline: $set_environment_variables, run: "always"
end
