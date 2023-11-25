## Plarza Crawler
### Dependencies
 - [ChromeDriver](https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/119.0.6045.105/linux64/chromedriver-linux64.zip) 119.0.6045.105 or later
 - [Chrome for Testing](https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/119.0.6045.105/linux64/chrome-linux64.zip) 119.0.6045.105 or later

### Developing
```bash
curl -o chrometesting.zip https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/119.0.6045.105/linux64/chrome-linux64.zip
unzip chrometesting.zip

sudo cp -a ./chrome-linux64/. /usr/bin/

curl -o chromedriver.zip https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/119.0.6045.105/linux64/chromedriver-linux64.zip
unzip chromedriver.zip

./chromedriver-linux64/chromedriver

cargo run
```