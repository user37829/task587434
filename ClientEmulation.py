from multiprocessing import Process
import time
import socket
import mechanize
import random

URL = "http://127.0.0.1:3000"
TIMEOUT = 10
N = 1000
RUNNING = True

class Worker:
    def __init__(self, URL, TIMEOUT, ID):
        self.url = URL
        self.timeout = TIMEOUT
        self.br = mechanize.Browser()
        self.br.set_handle_robots(False)
        self.id = ID
        
    def open_conn(self):
        global RUNNING
        while RUNNING:
            try:
                data = self.br.open(self.url, None, self.timeout).read().decode().strip()
                if time.time() - int(data) <= 30:
                    print ("Thread %s - OK" % self.id)
                else:
                    print ("Thread %s - Invalid data %s" % (self.id, data))
                    RUNNING = False
            except socket.error:
                print ("Thread %s - Timeout" % self.id)
                RUNNING = False
            time.sleep(random.randint(1, 35))
            #time.sleep(100)
        
pool = []
for i in range(N):
    w = Worker(URL, TIMEOUT, i)
    t = Process(target=w.open_conn)
    pool.append(t)
    t.start()
