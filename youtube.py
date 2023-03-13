import requests
from bs4 import BeautifulSoup
from datetime import datetime
import xmltodict

COLOR = '\033[92m'
END = '\033[0m'

def youtube(user_name, count):
    res = requests.get(f'https://www.youtube.com/@{user_name}')

    res_formatted = BeautifulSoup(res.content, 'html.parser')
    CHANNEL_ID = res_formatted.find('meta', {'property': 'og:url'}).get('content')[-24:]

    url = f'https://www.youtube.com/feeds/videos.xml?channel_id={CHANNEL_ID}'
    res = requests.get(url)
    res_formatted = xmltodict.parse(res.text)
    for i in range(count):
        DATE_RAW = res_formatted['feed']['entry'][i]['published']
        DATE = int(datetime.fromisoformat(DATE_RAW).timestamp())

        CAPTION_CONTENT =  res_formatted['feed']['entry'][i]['title']

        MEDIA_PREVIEW_URL = res_formatted['feed']['entry'][i]['media:group']['media:thumbnail']['@url']

        LINK = res_formatted['feed']['entry'][i]['link']['@href']
        
        print(f'{COLOR}SOURCE:{END}\nYOUTUBE\n{COLOR}DATE:{END}\n{DATE}\n{COLOR}CAPTION:{END}\n{CAPTION_CONTENT}\n{COLOR}MEDIA PREVIEW:{END}\n{MEDIA_PREVIEW_URL}\n{COLOR}LINK:{END}\n{LINK}\n')