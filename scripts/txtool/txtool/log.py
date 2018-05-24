#!/usr/bin/env python3
# coding=utf-8

import logging
import logging.config
import yaml

with open('config/logging.yml', 'r') as f_conf:
    dict_conf = yaml.load(f_conf)

logging.config.dictConfig(dict_conf)

logger = logging.getLogger('info')

# logger.debug('debug test message')
# logger.info('info test message')
# logger.warn('warn test message')
# logger.error('error test message')
# logger.critical('critical test message')
