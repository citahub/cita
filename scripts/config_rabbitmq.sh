#!/bin/bash
if [ "$(whoami)" = "root" ]; then
    alias sudo='bash'
fi

sudo /etc/init.d/rabbitmq-server restart
sudo rabbitmqctl add_vhost dev
sudo rabbitmqctl set_permissions -p dev guest ".*" ".*" ".*"
