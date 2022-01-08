from Network import *


net = Network("127.0.0.1", 10000)
res = Response(1, 2)
net.send(res)