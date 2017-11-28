-- run:
-- wrk -c 100 -d 10 -t 2 -s peerCount.lua http://127.0.0.1:1337

wrk.method = "POST"
wrk.body = '{"jsonrpc":"2.0","method":"peerCount","params":[],"id":74}'