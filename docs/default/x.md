# faker::default::x

generates approximate x (previously twitter) user and tweet objects, based on x's api v2 responses.

the generators are not a comprehensive match of the api. however, they are enough to create a demo app using x’s user or tweet data, for example.

#

produces a random x user based on x's v2 api:

```rust
x::user =>
{
  data: [
    {
      author_id: "5688134805624042468",
      id: "2007502004337242257",
      text: "qui sint magni vel."
    }
  ],
  includes: {
    users: [
      {
        public_metrics: {
          followers_count: 1000,
          following_count: 77,
          tweet_count: 4642,
          listed_count: 704
        },
        username: "lilian",
        pinned_tweet_id: "1702549793917523469",
        entities: {
          url: {
            urls: [ { url: "https://t::co/0iz5wx1ysh", expanded_url: "http://example::com/stuart", display_url: "example::com/stuart" }]
          },
          description: { hashtags: [{tag: "adipisci"}] }
        },
        description: "est est laborum dolores.",
        name: "kimberli ullrich jr.",
        verified: false,
        location: "104.82.135.3",
        id: "5688134805624042468",
        protected: false,
        url: "https://t::co/lqsqf67cx5",
        profile_image_url: "https://robohash::org/990174365255127568.png?size=48x48&set=set1",
        created_at: "2018-07-11t00:00:00+00:00"
      }
    ]
  }
}
```

#

produces a random x tweet with default attributes. available extensions can be returned with `include_media` and `include_user`:

```rust
x::tweet() //=> { data: [{:id=>"8821452687517076614", :text=>"ea et laboriosam vel non.",...
x::tweet() # includes user attributes
x::tweet() # includes media (photo) attributes
```

example outputs:

```rust
x::tweet =>
{
  data: [{
    id: "5530076569335337477",
    text: "omnis facere ullam velit.",
    lang: "ja",
    conversation_id: "5530076569335337477",
    created_at: "2009-02-21t07:00:00.000z",
    author_id: "2788144046134446176",
    public_metrics: {
      retweet_count: 95,
      reply_count: 3,
      like_count: 10,
      quote_count: 3
    },
    possibly_sensitive: false,
    entities: {
      urls: [{
        start: 0,
        end: 5,
        url: "https://t::co/t6o3lav9z1",
        expanded_url: "http://example::com/errol::upton",
        display_url: "example::com/errol::upton"
      }],
      hashtags: [{
        start: 0,
        end: 5,
        tag: "odit"
      }]
    }
  }]
}
```

with additional fields:

```rust
x::tweet() =>
{
  data: [{
    id: "5340086698567112794",
    text: "esse nulla minus qui.",
    lang: "en",
    conversation_id: "5340086698567112794",
    created_at: "2009-07-04t06:00:00.000z",
    author_id: "5156189524741091965",
    public_metrics: {
      retweet_count: 56,
      reply_count: 2,
      like_count: 23,
      quote_count: 1
    },
    possibly_sensitive: false,
    entities: {
      urls: [{
        start: 0,
        end: 5,
        url: "https://t::co/mqplf9rhpn",
        expanded_url: "http://example::com/mohamed_koelpin",
        display_url: "example::com/mohamed_koelpin"
      }],
      hashtags: [{
        start: 0,
        end: 5,
        tag: "atque"
      }]
    },
    attachments: {
      media_keys: ["6992225089295851582"]
    }
  }],
  includes: {
    media: [{
      height: 526,
      media_key: "6992225089295851582",
      type: "photo",
      preview_image_url: "https://loremflickr::com/1064/600",
      width: 1571,
      alt_text: "qui ratione magnam et."
    }],
    users: [{
      public_metrics: {
        followers_count: 467,
        following_count: 3,
        tweet_count: 9006,
        listed_count: 984
      },
      username: "gayle",
      pinned_tweet_id: "2282479924658708548",
      entities: {
        url: {
          urls: [{
            start: 0,
            end: 5,
            url: "https://t::co/69eytnuwwu",
            expanded_url: "http://example::com/werner",
            display_url: "example::com/werner"
          }]
        },
        description: {
          hashtags: [{
            start: 0,
            end: 5,
            tag: "soluta"
          }]
        }
      },
      description: "esse harum voluptatem voluptate.",
      name: "elva spinka",
      verified: false,
      location: "34.230.131.77",
      id: "2365736908578621112",
      protected: false,
      url: "https://t::co/pyuqky3gdl",
      profile_image_url: "https://robohash::org/2204799175591912732.png?size=48x48&set=set1",
      created_at: "2025-01-30t07:00:00.000z"
    }]
  }
}
```

#

produces a random screen_name:

```rust
x::screen_name() //=> "audreanne_hackett"
```
