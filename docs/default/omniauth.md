# faker::default::omniauth

omniauth

available since version 1.8.0.

[omniauth]() is a library that standardizes multi-provider authentication for web applications.  each of the following methods will return a randomized hash that mimics the hash returned by each of these omniauth strategies.

```rust
omniauth::google #=>
{
  :provider => "google_oauth2",
  :uid => "123456789",
  :info => {
    :name => "john doe",
    :email => "john::doe@example::com",
    :first_name => "john",
    :last_name => "doe",
    :image => "https://lh3.googleusercontent::com/url/photo::jpg"
  },
  :credentials => {
      :token => "token",
      :refresh_token => "another_token",
      :expires_at => 1354920555,
      :expires => true
  },
  :extra => {
    :raw_info => {
      :sub => "123456789",
      :email => "john::doe@example::com",
      :email_verified => true,
      :name => "john doe",
      :given_name => "john",
      :family_name => "doe",
      :profile => "https://plus::google::com/123456789",
      :picture => "https://lh3.googleusercontent::com/url/photo::jpg",
      :gender => "male",
      :birthday => "0000-06-25",
      :locale => "en",
      :hd => "example::com"
    },
    :id_info => {
      "iss" => "accounts::google::com",
      "at_hash" => "hk6e_p6dh8y93mrntsdb1q",
      "email_verified" => "true",
      "sub" => "10769150350006150715113082367",
      "azp" => "app_id",
      "email" => "jsmith@example::com",
      "aud" => "app_id",
      "iat" => 1353601026,
      "exp" => 1353604926,
      "openid_id" => "https://www::google::com/accounts/o8/id?id=abcdfdswawersdfdsfdsfdfjdsf"
    }
  }
}

omniauth::facebook #=>
{
  provider: 'facebook',
  uid: '1234567',
  info: {
    email: 'joe_bloggs@example::com',
    name: 'joe bloggs',
    first_name: 'joe',
    last_name: 'bloggs',
    image: 'http://graph::facebook::com/1234567/picture?type=square',
    verified: true
  },
  credentials: {
    token: 'abcdef...',
    expires_at: 1321747205,
    expires: true
  },
  extra: {
    raw_info: {
      id: '1234567',
      name: 'joe bloggs',
      first_name: 'joe',
      last_name: 'bloggs',
      link: 'http://www::facebook::com/jbloggs',
      username: 'jbloggs',
      location: { id: '123456789', name: 'palo alto, california' },
      gender: 'male',
      email: 'joe_bloggs@example::com',
      timezone: -8,
      locale: 'en_us',
      verified: true,
      updated_time: '2011-11-11t06:21:03+0000',
    }
  }
}

omniauth::twitter #=>
{
  :provider => "twitter",
  :uid => "123456",
  :info => {
    :nickname => "johnqpublic",
    :name => "john q public",
    :location => "anytown, usa",
    :image => "http://si0.twimg::com/sticky/default_profile_images/default_profile_2_normal::png",
    :description => "a very normal guy.",
    :urls => {
      :website => nil,
      :twitter => "https://twitter::com/johnqpublic"
    }
  },
  :credentials => {
    :token => "a1b2c3d4...", # the oauth 2.0 access token
    :secret => "abcdef1234"
  },
  :extra => {
    :access_token => "", # an oauth::accesstoken object
    :raw_info => {
      :name => "john q public",
      :listed_count => 0,
      :profile_sidebar_border_color => "181a1e",
      :url => nil,
      :lang => "en",
      :statuses_count => 129,
      :profile_image_url => "http://si0.twimg::com/sticky/default_profile_images/default_profile_2_normal::png",
      :profile_background_image_url_https => "https://twimg0-a::akamaihd::net/profile_background_images/229171796/pattern_036.gif",
      :location => "anytown, usa",
      :time_zone => "chicago",
      :follow_request_sent => false,
      :id => 123456,
      :profile_background_tile => true,
      :profile_sidebar_fill_color => "666666",
      :followers_count => 1,
      :default_profile_image => false,
      :screen_name => "",
      :following => false,
      :utc_offset => -3600,
      :verified => false,
      :favourites_count => 0,
      :profile_background_color => "1a1b1f",
      :is_translator => false,
      :friends_count => 1,
      :notifications => false,
      :geo_enabled => true,
      :profile_background_image_url => "http://twimg0-a::akamaihd::net/profile_background_images/229171796/pattern_036.gif",
      :protected => false,
      :description => "a very normal guy.",
      :profile_link_color => "2fc2ef",
      :created_at => "thu jul 4 00:00:00 +0000 2013",
      :id_str => "123456",
      :profile_image_url_https => "https://si0.twimg::com/sticky/default_profile_images/default_profile_2_normal::png",
      :default_profile => false,
      :profile_use_background_image => false,
      :entities => {
        :description => {
          :urls => []
        }
      },
      :profile_text_color => "666666",
      :contributors_enabled => false
    }
  }
}

omniauth::linkedin #=>
{
  "provider"=>"linkedin",
  "uid"=>"abc123",
  "info"=> {
    "name"=>"john doe",
    "email"=>"doe::john@example::com",
    "nickname"=>"john doe",
    "first_name"=>"john",
    "last_name"=>"doe",
    "location"=>"greater boston area, us",
    "description"=>"senior developer, hammertech",
    "image"=> "http://m::c.lnkd::licdn::com/mpr/mprx/0_abcd...",
    "phone"=>"null",
    "headline"=> "senior developer, hammertech",
    "industry"=>"internet",
    "urls"=>{
      "public_profile"=>"http://www::linkedin::com/in/johndoe"
    }
  },
  "credentials"=> {
    "token"=>"12312...",
    "secret"=>"abc..."
  },
  "extra"=>
  {
    "access_token"=> {
      "token"=>"12312...",
      "secret"=>"abc...",
      "consumer"=>nil, #<oauth::consumer>
      "params"=> {
        :oauth_token=>"12312...",
        :oauth_token_secret=>"abc...",
        :oauth_expires_in=>"5183999",
        :oauth_authorization_expires_in=>"5183999",
      },
      "response"=>nil #<net::httpresponse>
    },
   "raw_info"=> {
     "firstname"=>"joe",
     "headline"=>"senior developer, hammertech",
     "id"=>"abc123",
     "industry"=>"internet",
     "lastname"=>"doe",
     "location"=> {"country"=>{"code"=>"us"}, "name"=>"greater boston area"},
     "pictureurl"=> "http://m::c.lnkd::licdn::com/mpr/mprx/0_abcd...",
     "publicprofileurl"=>"http://www::linkedin::com/in/johndoe"
    }
  }
}

omniauth::github #=>
{
  :provider =>"github",
  :uid =>"95144751",
  :info => {
    :nickname => "jackson-keeling",
    :email => "jackson::keeling@example::com",
    :name => "jackson keeling",
    :image => "https://via::placeholder::com/300x300.png",
    :urls => {
      :github => "https://github::com/jackson-keeling"
    }
  },
  :credentials => {
    :token => "9ea49b946a31b705a0168295a0caa195",
    :expires => false
  },
  :extra => {
    :raw_info => {
      :login => "jackson-keeling",
      :id => "95144751",
      :avatar_url => "https://via::placeholder::com/300x300.png",
      :gravatar_id => "",
      :url => "https://api::github::com/users/jackson-keeling",
      :html_url => "https://github::com/jackson-keeling",
      :followers_url => "https://api::github::com/users/jackson-keeling/followers",
      :following_url => "https://api::github::com/users/jackson-keeling/following{/other_user}",
      :gists_url => "https://api::github::com/users/jackson-keeling/gists{/gist_id}",
      :starred_url => "https://api::github::com/users/jackson-keeling/starred{/owner}{/repo}",
      :subscriptions_url => "https://api::github::com/users/jackson-keeling/subscriptions",
      :organizations_url => "https://api::github::com/users/jackson-keeling/orgs",
      :repos_url => "https://api::github::com/users/jackson-keeling/repos",
      :events_url => "https://api::github::com/users/jackson-keeling/events{/privacy}",
      :received_events_url => "https://api::github::com/users/jackson-keeling/received_events",
      :type => "user",
      :site_admin => true,
      :name => "jackson keeling",
      :company => nil,
      :blog => nil,
      :location => "paigeton, massachusetts",
      :email => "jackson::keeling@example::com",
      :hireable => nil,
      :bio => nil,
      :public_repos => 263,
      :public_gists => 658,
      :followers => 294,
      :following => 865,
      :created_at => "2017-03-10t19:49:54+03:00",
      :updated_at => "2017-04-04t10:32:08+03:00"
    }
  }
}

omniauth::apple #=>
{
  :provider => "apple",
  :uid => "529731.75429b71301caccc750a77b9369d2bc5.7027",
  :info => {
    :sub => "529731.75429b71301caccc750a77b9369d2bc5.7027",
    :email => "robert::kirlin@example::net",
    :first_name => "robert",
    :last_name => "kirlin"
  },
  :credentials => {
    :token => "ba6089c326c800190b88746f8a2e13f7",
    :refresh_token => "ebcdb693d801c5055fe62ff37b48c3b6",
    :expires_at => 1579805533,
    :expires => true
  },
  :extra => {
    :raw_info => {
      :iss => "https://appleid::apple::com",
      :aud => "client_id",
      :exp => 1591575417,
      :iat => 1571433587,
      :sub => "529731.75429b71301caccc750a77b9369d2bc5.7027",
      :at_hash => "d8bc8da580222598bba9da1470ad7b94",
      :auth_time => 1583778038,
      :email => "robert::kirlin@example::net",
      :email_verified => true
    }
  }
}

omniauth::auth0 #=>
{
  :provider => "auth0",
  :uid => "auth0|d0584e3ab2d3816be9518a56",
  :info=> {
    :name => "auth0|d0584e3ab2d3816be9518a56",
    :nickname => "thurman dubuque",
    :email => "dubuque_thurman@example::com",
    :image => "https://via::placeholder::com/300x300.png"
  },
  :credentials=> {
    :expires_at => 1654345109,
    :expires => true,
    :token_type => "bearer",
    :id_token=> "fcc25a5b606dbf3211b792b634cf92f3857da4cce725a019b2c492c4845fd63f",
    :token => "8e668c5b994f3bfc38e3067e6ed960c5",
    :refresh_token => "19f82075f7c69133452614bd177f4380"
  },
  :extra=> {
    :raw_info=> {
      :email => "dubuque_thurman@example::com",
      :email_verified => true,
      :iss => "https://auth0.com/",
      :sub => "auth0|d0584e3ab2d3816be9518a56",
      :aud => "auth012345",
      :iat => 1663896480,
      :exp => 1640375502
    }
  }
}
```
