## Plarza Crawler
### Dependencies
 - [ChromeDriver](https://googlechromelabs.github.io/chrome-for-testing) 119.0.6045.105 or later
 - [Chrome for Testing](https://googlechromelabs.github.io/chrome-for-testing) 119.0.6045.105 or later  
    #### (Ubuntu Specific)
 - [build-essential](https://packages.ubuntu.com/focal/build-essential)
 - [libasound2](https://packages.ubuntu.com/focal/libasound2)
 - [libssl-dev](https://packages.ubuntu.com/focal/libssl-dev)

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