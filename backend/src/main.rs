// mod instagram;
mod twitter;
//mod youtube;

fn main()
{
    // let ig_posts = instagram::fetch("benawad97", 10, "cool");
    
    // for post in ig_posts {
    //     let link = post["link"].as_str().unwrap();
    //     println!("{}", link);
    // }

    let tw_posts = twitter::fetch("benawad", 10, "Bearer AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA");
    
    for post in tw_posts {
        let link = post["link"].as_str().unwrap();
        println!("{}", link);
    }

    // let yt_posts = youtube::fetch("bawad", 10);
    
    // for post in yt_posts {
    //     let link = post["link"].as_str().unwrap();
    //     println!("{}", link);
    // }
}
