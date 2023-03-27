from instagram import instagram
from twitter import twitter
from youtube import youtube
import itertools
import json

ig_posts = instagram('mrbeast',10,'mid=Y-LN3gALAAH5btPQm6jMK8lN7C-G; ig_did=C520C543-B8E0-4694-8066-02422876998F; ig_nrcb=1; datr=383iY_Xb2sur-aBafV0PuE0f; csrftoken=dfMeLmzg7x6FF9Taelce3UsRYyv5SZKD; ds_user_id=39143931185; dpr=2; shbid="1203\05439143931185\0541710977945:01f7a5aa73f324da72075803843890f5028f209a2243a7a4bb808950c4351482c578397e"; shbts="1679441945\05439143931185\0541710977945:01f7339bb83e612034ce64b420a4647cf7fd094d3e50b7463565f176fd3cd73ac2c622b0"; sessionid=39143931185%3AHe7tyEMvETJGdf%3A26%3AAYfa2ITIEe-1WY77f9onn4I_F8qN4otwNjpefJXk0w; rur="FRC\05439143931185\0541711067279:01f75708e26302a3b4124039316864640a72afea17db976b8c628da99cb4a835409d6ff8"')
tw_posts = twitter('mrbeast', 10, 'Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA')
yt_posts = youtube('mrbeast',10)

all_posts = itertools.chain(ig_posts, tw_posts, yt_posts)

sorted_posts = sorted(all_posts, key=lambda d: d['date'], reverse=True)

for post in sorted_posts:
    with open('data.csv', 'a') as f:
        f.write(str(post['user']+'•'+post['source'])+'•'+str(post['date'])+'•'+str(post['caption']).replace('\n', ' ')+'•'+str(post['media'])+'•'+str(post['link'])+'\n')
    print(post['user'],post['source'],post['date'],post['caption'].replace('\n', ' '),post['media'],post['link'])