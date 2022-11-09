from flask import Flask, request, url_for
import csv
import re
import json

NAMES_FILE = "data/logins.csv"
USED_NAMES_FILE = "data/used.json"

app = Flask(__name__)

@app.route("/")
def hello_world():
    return f"""Stuur deze form 1 keer per team, zo krijg de login-gegevens voor picoctf voor jouw team<br>Toegestane karakters voor teamnaam: a-Z, A-Z, 0-9 (behalve de lowercase letter q)
<form action="{url_for('please_dont_spam')}" method="post">
    <label for="teamname">Teamnaam</label>
    <input type="text" name="teamname">
    <input type="submit">
</form>
"""


@app.route('/please_dont_spam', methods=['POST'])
def please_dont_spam():
    teamname = request.form['teamname'][:32]
    teamname = ''.join(c for c in teamname if c.isalnum() and c != 'q')
    with open(USED_NAMES_FILE) as usedfile:
        used = json.load(usedfile)
    if teamname in used.values():
        return 'sorry, teamnaam bestaat al, neem een andere teamnaam'
    with open(NAMES_FILE) as infile:
        reader = csv.DictReader(infile)
        for line in reader:
            if line['username'] not in used:
                used[line['username']] = teamname
                with open(USED_NAMES_FILE, 'w') as usedfile:
                    json.dump(used, usedfile)
                return f'Save this, you will only see this once<br>Username: {line["username"]}<br>Password: {line["password"]}'
    return "Sorry, geen usernames meer over, contacteer een van de admins"


@app.route('/spoopy_admin_url_replace_me_pls')
def adminapi():
    with open(USED_NAMES_FILE) as usedfile:
        return json.load(usedfile)
