#!/bin/sh
/etc/init.d/rabbitmq-server restart
rabbitmqctl add_vhost dev
rabbitmqctl set_permissions -p dev guest ".*" ".*" ".*"
