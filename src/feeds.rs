pub fn get_feed_sources() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Sansan", "https://buildersbox.corp-sansan.com/rss"),
        ("Yahoo", "https://techblog.yahoo.co.jp/index.xml"),
        ("Recruit", "https://recruit-tech.co.jp/blog/feed/"),
        ("LINE", "https://engineering.linecorp.com/ja/feed/"),
        ("Mercari", "https://engineering.mercari.com/blog/feed.xml"),

        ("Google", "https://blog.google/rss"),
        ("Azure", "https://azure.microsoft.com/en-us/blog/feed/"),
        ("AWS Security", "https://aws.amazon.com/blogs/security/feed/"),

        ("Github","https://github.blog/feed/"),
        ("Salesforce", "https://www.salesforce.com/blog/feed/"),
        ("Developer Salesforce", "https://developer.salesforce.com/blogs/feed"),
        ("Strip","https://stripe.com/blog/feed.rss"),
        ("Netflix","https://medium.com/feed/netflix-techblog"),
        ("spotify","https://engineering.atspotify.com/feed"),
        ("Uber","https://www.uber.com/blog/engineering/rss/ "),
        ("Dropbox","https://dropbox.tech/feed")
    ]
}