import configparser
try:
    import httplib
except:
    import http.client as httplib

SETTING_PATH = 'config/setting.cfg'


def _join_url(host, port, scheme='http://'):
    return scheme + host + ':' + port


def host():
    config = configparser.ConfigParser()
    config.read(SETTING_PATH)
    host = config.get('jsonrpc_url', 'host')
    return host


def endpoint():
    config = configparser.ConfigParser()
    config.read(SETTING_PATH)
    host = config.get('jsonrpc_url', 'host')
    port = config.get('jsonrpc_url', 'port')
    return _join_url(host, port)


def have_internet(url):
    conn = httplib.HTTPConnection(url, timeout=4)
    try:
        conn.request("HEAD", "/")
        conn.close()
        return True
    except:
        conn.close()
        return False
