#!/usr/bin/env python3
# coding=utf-8

import logging
import logging.config
import re
import yaml


def replaceLogRecord():
    """This is a temporary method.
    We will remove pyethereum in the near future.
    """

    def makeRecord(self,
                   name,
                   level,
                   fn,
                   lno,
                   msg,
                   args,
                   exc_info,
                   func=None,
                   extra=None,
                   sinfo=None):
        name = re.sub(r'(^|[^a-zA-Z])eth([^a-zA-Z]|$)', r'\1cita\2', name)
        rv = logging._logRecordFactory(name, level, fn, lno, msg, args,
                                       exc_info, func, sinfo)
        if extra is not None:
            for key in extra:
                if (key in ["message", "asctime"]) or (key in rv.__dict__):
                    raise KeyError(
                        "Attempt to overwrite %r in LogRecord" % key)
                rv.__dict__[key] = extra[key]
        return rv

    def getMessage(self):
        msg = str(self.msg)
        if self.args:
            msg = msg % self.args
        msg = re.sub(r'(^|[^a-zA-Z])eth([^a-zA-Z]|$)', r'\1cita\2', msg)
        msg = re.sub(r'(^|[^a-zA-Z])gas([^a-zA-Z]|$)', r'\1quota\2', msg)
        return msg

    logging.Logger.makeRecord = makeRecord
    logging.LogRecord.getMessage = getMessage


replaceLogRecord()

with open('config/logging.yml', 'r') as f_conf:
    dict_conf = yaml.load(f_conf)

logging.config.dictConfig(dict_conf)

logger = logging.getLogger('info')

# logger.debug('debug test message')
# logger.info('info test message')
# logger.warn('warn test message')
# logger.error('error test message')
# logger.critical('critical test message')
