#!/bin/bash

# Enviroments
if [ `uname` == 'Darwin' ]; then
    CITA_BIN="$(dirname $(realpath $0))"
else
    CITA_BIN="$(dirname $(readlink -f $0))"
fi
CITA_SCRIPTS=$(dirname $CITA_BIN)/scripts

# Wrap the create script.
if [ -e $CITA_SCRIPTS/create_cita_config.py ]; then
    $CITA_SCRIPTS/create_cita_config.py $@
else
    echo -e "\033[0;31mPlease run this command after build 🎨"
fi
