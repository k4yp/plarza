from flask import Flask, request
from flask_cors import CORS
import psycopg2
import secrets
import time

app = Flask(__name__)
cors = CORS(app)

conn = psycopg2.connect(host='localhost', database='testdb', user='postgres', port=5432)

cur = conn.cursor()

@app.route('/posts')
def home():
    cur.execute('SELECT * FROM post')
    rows = cur.fetchall()
    result = []
    for row in rows:
        d = {
            'link': row[0],
            'user': row[1],
            'source': row[2],
            'date': row[3],
            'caption': row[4],
            'media': row[5]
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