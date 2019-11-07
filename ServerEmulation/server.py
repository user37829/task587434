from flask import Flask
import time

application = Flask(__name__)

@application.route("/")
def index():
    time.sleep(2)
    return str(int(time.time()))

