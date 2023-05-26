import requests
from datetime import datetime

results = []

def twitter(user_name, count, token):

    url = f'https://api.twitter.com/graphql/9zwVLJ48lmVUk8u_Gh9DmA/ProfileSpotlightsQuery?variables=%7B%22screen_name%22%3A%22{user_name}%22%7D'
    response = requests.request('GET', url, headers={'authorization': token}).json()

    USER_ID = response['data']['user_result_by_screen_name']['result']['rest_id']
    url = f'https://api.twitter.com/graphql/73BM9FU1mPITScnhs6iXug/UserTweets?variables=%7B%22userId%22%3A%22{USER_ID}%22%2C%22count%22%3A{count+2}%2C%22includePromotedContent%22%3Afalse%2C%22withQuickPromoteEligibilityTweetFields%22%3Atrue%2C%22withSuperFollowsUserFields%22%3Atrue%2C%22withDownvotePerspective%22%3Afalse%2C%22withReactionsMetadata%22%3Afalse%2C%22withReactionsPerspective%22%3Afalse%2C%22withSuperFollowsTweetFields%22%3Atrue%2C%22withVoice%22%3Atrue%2C%22withV2Timeline%22%3Atrue%7D&features=%7B%22responsive_web_twitter_blue_verified_badge_is_enabled%22%3Atrue%2C%22responsive_web_graphql_exclude_directive_enabled%22%3Afalse%2C%22verified_phone_label_enabled%22%3Afalse%2C%22responsive_web_graphql_timeline_navigation_enabled%22%3Atrue%2C%22responsive_web_graphql_skip_user_profile_image_extensions_enabled%22%3Afalse%2C%22tweetypie_unmention_optimization_enabled%22%3Atrue%2C%22vibe_api_enabled%22%3Atrue%2C%22responsive_web_edit_tweet_api_enabled%22%3Atrue%2C%22graphql_is_translatable_rweb_tweet_is_translatable_enabled%22%3Atrue%2C%22view_counts_everywhere_api_enabled%22%3Atrue%2C%22longform_notetweets_consumption_enabled%22%3Atrue%2C%22tweet_awards_web_tipping_enabled%22%3Afalse%2C%22freedom_of_speech_not_reach_fetch_enabled%22%3Afalse%2C%22standardized_nudges_misinfo%22%3Atrue%2C%22tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled%22%3Afalse%2C%22interactive_text_enabled%22%3Atrue%2C%22responsive_web_text_conversations_enabled%22%3Afalse%2C%22longform_notetweets_richtext_consumption_enabled%22%3Afalse%2C%22responsive_web_enhance_cards_enabled%22%3Afalse%7D'
    response = requests.get(url, headers={'authorization': token}).json()

    for i in range(count):
        try:
            response['data']['user']['result']['timeline_v2']['timeline']['instructions'][1]['entries'][i]['content']['itemContent']['tweet_results']['result']['legacy']['entities']['user_mentions'][0]['name']
        except:
            DATE_RAW = response['data']['user']['result']['timeline_v2']['timeline']['instructions'][1]['entries'][i]['content']['itemContent']['tweet_results']['result']['legacy']['created_at']
            DATE = int(datetime.strptime(DATE_RAW, "%a %b %d %H:%M:%S %z %Y").timestamp())

            CAPTION = response['data']['user']['result']['timeline_v2']['timeline']['instructions'][1]['entries'][i]['content']['itemContent']['tweet_results']['result']['legacy']['full_text']
            
            try:
                MEDIA_URL = response['data']['user']['result']['timeline_v2']['timeline']['instructions'][1]['entries'][i]['content']['itemContent']['tweet_results']['result']['legacy']['entities']['media'][1]['media_url_https']
            except:
                MEDIA_URL = None

            LINK_RAW = response['data']['user']['result']['timeline_v2']['timeline']['instructions'][1]['entries'][i]['content']['itemContent']['tweet_results']['result']['legacy']['conversation_id_str']
            LINK = f'https://twitter.com/{user_name}/status/{LINK_RAW}'

            results.append({'user': user_name, 'source': 'twitter', 'date': DATE, 'caption': CAPTION.replace("'","''"), 'media_url': MEDIA_URL, 'link': LINK})

    return results