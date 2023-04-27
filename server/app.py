from flask import Flask
from flask_cors import CORS, cross_origin
import psycopg2

app = Flask(__name__)
cors = CORS(app)
app.config['CORS_HEADERS'] = 'Content-Type'

conn = psycopg2.connect(host="localhost", database="testdb", user="postgres", port=5432)

cur = conn.cursor()

@app.route("/posts")
@cross_origin()
def home():
    cur.execute("SELECT * FROM post")
    rows = cur.fetchall()
    result = []
    for row in rows:
        d = {
            "link": row[0],
            "user": row[1],
            "source": row[2],
            "date": row[3],
            "caption": row[4],
            "media": row[5]
        }
        result.append(d)

    return result