import requests
from bs4 import BeautifulSoup

results = []

def reddit(user_name, count):

    url = f'https://reddit.com/user/{user_name}'
    response = requests.request('GET', url).json()