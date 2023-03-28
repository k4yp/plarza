from instagram import instagram
from twitter import twitter
from youtube import youtube
import itertools
import requests
import json

ig_posts = instagram('mrbeast',10,'ig_nrcb=1; mid=Y4XvvQAEAAG6EBCP0kuaQ16r6Wst; ig_did=E964BB5A-1A34-4E32-9CFC-4AD6F6653583; ds_user_id=39143931185; datr=1XWIY_dqBgljNJxexXuQapuL; csrftoken=yZZuaKCDj4U8zspIh2FENPgd7c6WYEd4; shbid="1203\05439143931185\0541711276952:01f76063ac14e83abdcde8c7cdea6cfc2b389329a7cd3aeab1e2579ecb891d4618d2d166"; shbts="1679740952\05439143931185\0541711276952:01f79a880573db187b2e40a7c24783bf87d13936291eba98049b3202a2d8fb2e499c307e"; rur="FRC\05439143931185\0541711528752:01f7c22fc862fb75e7f75dfd6162167e4fa9de056d6ba14082db58e0568434b12c11a613"; sessionid=39143931185%3AleOKUK93VYYPN9%3A11%3AAYeZiYqdEpzvxGmMu_oWyYvZ0nbOFelkuI7fOfD8Og')
tw_posts = twitter('mrbeast', 10, 'Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA')
yt_posts = youtube('mrbeast',10)

all_posts = itertools.chain(ig_posts, tw_posts, yt_posts)

sorted_posts = sorted(all_posts, key=lambda d: d['date'], reverse=True)

headers = {
  'Accept': 'application/json',
  'NS': 'test',
  'DB': 'test',
  'Content-Type': 'application/json',
  'Authorization': 'Basic cm9vdDpyb290'
}

for post in sorted_posts:
  payload = f"INSERT INTO posts (user, source, date, caption, media, link) VALUES ('{post['user']}', '{post['source']}', '{post['date']}', '{post['caption']}', '{post['media']}', '{post['link']}');"
  response = requests.request("POST", 'http://localhost:8000/sql', headers=headers, data=payload.encode('utf-8'))

  print(response.text)