import requests

COLOR = '\033[92m'
END = '\033[0m'

def instagram(user_name, count, token):

    url = f'https://www.instagram.com/{user_name}/?__a=1&__d=dis'
    res = requests.get(url, headers={'cookie': token}).json()

    for i in range(count):
        DATE = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['taken_at_timestamp']

        CAPTION_CONTENT = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['edge_media_to_caption']['edges'][0]['node']['text']

        MEDIA_PREVIEW_URL = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['display_url']

        LINK_RAW = res['graphql']['user']['edge_owner_to_timeline_media']['edges'][i]['node']['shortcode']
        LINK = f'https://www.instagram.com/p/{LINK_RAW}/'

        print (f'{COLOR}SOURCE:{END}\nINSTAGRAM\n{COLOR}DATE:{END}\n{DATE}\n{COLOR}CAPTION:{END}\n{CAPTION_CONTENT}\n{COLOR}MEDIA PREVIEW:{END}\n{MEDIA_PREVIEW_URL}\n{COLOR}LINK:{END}\n{LINK}\n')