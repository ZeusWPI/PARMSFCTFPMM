from flask import Flask, request, url_for
import json

app = Flask(__name__)


@app.route('/data', methods=['POST'])
def please_dont_spam():
    print(request.json)
    return
