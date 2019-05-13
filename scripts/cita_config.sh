#!/bin/bash

# Enviroments
CITA_BIN=$(realpath "$(dirnamme "$0")")
CITA_SCRIPTS=$(dirname "$CITA_BIN")/scripts

# Wrap the create script.
if [ -e "$CITA_SCRIPTS"/create_cita_config.py ]; then
    "$CITA_SCRIPTS"/create_cita_config.py "$@"
else
    echo -e "\033[0;31mPlease run this command after build 🎨"
fi
