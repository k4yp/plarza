from flask import Flask
from instagram import instagram
from twitter import twitter
from youtube import youtube
import itertools

ig_posts = instagram('benawad97',10,"sessionid=39143931185%3A3o48qc1sJsZVfG%3A11%3AAYdMbskZd2-0XRkGuM8OmAEvKO41r58FsCgRRmeMi-o; csrftoken=07OKmHycbuCYLDHcNT00Nsv7kaLg0gWL; ds_user_id=39143931185; ig_did=20C85762-2D4C-4399-9434-FECEA65B62AE; ig_nrcb=1; mid=Y89tmwAEAAEy2Lhw8Pnaeq7Ft6fL; shbid='1203\\05439143931185\\0541706074680:01f7d78183f0da853b537ace675f2f06f46d0c224edf89b94b412e14612264a79b9ccd11'; shbts='1674538680\\05439143931185\\0541706074680:01f78c48294f0f676593f4f2d5a7af1c164b8533eed6e7f124954b0dd8066f72c1b47723'")
tw_posts = twitter('benawad', 10, 'Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA')
yt_posts = youtube('bawad',10)

all_posts = itertools.chain(ig_posts, tw_posts, yt_posts)

sorted_posts = sorted(all_posts, key=lambda d: d['date'], reverse=True)

for post in sorted_posts:
    print(post['source'],post['date'],post['caption'],post['media'],post['link'])

app = Flask(__name__)

@app.route("/")
def hello_world():
    return f"<p>{sorted_posts[0]}</p>"