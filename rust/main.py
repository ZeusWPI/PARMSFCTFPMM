from flask import Flask, request, url_for
import json
from requests import get
from flask.templating import stream_template, render_template

app = Flask(__name__)

LEADERBOARD_DATA = []
MAX_SCORE = 1


@app.route('/')
def index():
    global LEADERBOARD_DATA
    global MAX_SCORE
    return render_template('index.html', LEADERBOARD_DATA=LEADERBOARD_DATA, MAX_SCORE=MAX_SCORE)



@app.route('/data', methods=['POST'])
def please_dont_spam():
    global LEADERBOARD_DATA
    global MAX_SCORE

    login_team_mapping = get('http://parmesan:5000/spoopy_admin_url_replace_me_pls').json()
    team_extra_score = get("http://manual_flags:80/scores").json()

    print(request.json)

    login_base_score = {
        k: v for k, v in request.json.items() if k in login_team_mapping
    }

    team_score = [
        (login_team_mapping[login], int(score) + team_extra_score[login_team_mapping[login]]) for login, score in login_base_score.items()
    ]

    print(team_score)

    team_score.sort(key=lambda x: x[1], reverse=True)

    LEADERBOARD_DATA = team_score
    MAX_SCORE = max(map(lambda x: x[1], team_score))

    return "OK"
