import requests

results = []

def instagram(user_name, count, token):

    url = f'https://www.instagram.com/{user_name}/?__a=1&__d=dis'
    res = requests.get(url, headers={'cookie': token}).json()

    for i in range(count):
        DATE = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['taken_at_timestamp']

        CAPTION = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['edge_media_to_caption']['edges'][0]['node']['text']

        MEDIA = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['display_url']

        LINK_RAW = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['shortcode']
        LINK = f'https://www.instagram.com/p/{LINK_RAW}/'

        results.append({'user':user_name,'source':'instagram','date':DATE,'caption':CAPTION.replace("'","''"),'media':MEDIA,'link':LINK})

    return results