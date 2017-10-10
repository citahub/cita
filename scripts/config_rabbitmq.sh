#!/bin/sh
sudo (){
    cmd=$*
    if [ "$(whoami)" = "root" ]; then
        ${cmd}
    else
        /usr/bin/sudo ${cmd}
    fi
}
sudo /etc/init.d/rabbitmq-server restart
sudo rabbitmqctl add_vhost dev
sudo 'rabbitmqctl set_permissions -p dev guest ".*" ".*" ".*"'
