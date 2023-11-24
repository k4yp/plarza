from flask import Flask, request, send_file
from flask import make_response
from flask_cors import CORS
import psycopg2
import secrets
import time

app = Flask(__name__)
cors = CORS(app)

conn = psycopg2.connect(host='localhost', database='dev', user='dev', port=5432, password='MxTb3D6Kp8TpxxSsRTBMsNvaA2kXpEJr')

cur = conn.cursor()

@app.route('/posts')
def home():
    cur.execute('SELECT * FROM post')
    rows = cur.fetchall()
    result = []
    for row in rows:
        d = {
            'postid': row[0],
            'link': row[1],
            'user': row[2],
            'source': row[3],
            'date': row[4],
            'caption': row[5],
            'media_url': row[7],
        }
        result.append(d)

    return result

@app.route('/signup', methods = ['POST'])
def signup():
    data = request.json
    email = data['email']
    password = data['password']
    cur.execute(f'INSERT INTO "user" (username, email, password, date) VALUES (\'{secrets.token_hex(8)}\', \'{email}\', \'{password}\', {int(time.time())})')
    conn.commit()
    return 'success'

@app.route('/login', methods = ['POST'])
def login():
    data = request.json
    email = data['email']
    password = data['password']
    return 'success'

@app.route('/media/<string:postid>')
def media(postid):
    cur.execute(f"SELECT media_path FROM post WHERE postid = '{postid}'")
    try:
        result = cur.fetchone()[0]
        print(result)
        response = make_response(send_file(result, mimetype='image/png'))
        response.headers['Cache-Control'] = 'no-store, no-cache, must-revalidate, max-age=0'
        return response
    except:
        return 'None'
    

@app.route('/user/<string:username>')
def info(username):
    cur.execute(f"select userid, username, bio, display, date, media from \"user\" where username='{username}';")
    rows = cur.fetchall()
    result = []
    for row in rows:
        d = {
            'userid': row[0],
            'username': row[1],
            'bio': row[2],
            'display': row[3],
            'date': row[4],
            'media': row[5],
        }
        result.append(d)

    return result