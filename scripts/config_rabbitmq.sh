#!/bin/bash
sudo(){
    set -o noglob
    if [ "$(whoami)" == "root" ] ; then
        $*
    else
        /usr/bin/sudo $*
    fi
    set +o noglob
}
sudo /etc/init.d/rabbitmq-server restart
(sudo rabbitmqctl list_vhosts | grep dev) && sudo rabbitmqctl delete_vhost dev
sudo rabbitmqctl add_vhost dev
sudo rabbitmqctl set_permissions -p dev guest '.*' '.*' '.*'
exit 0
