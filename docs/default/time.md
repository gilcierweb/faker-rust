# faker::default::time

```rust
time::between() //=> "2014-09-18 12:30:59 -0700"

i18n::locale = 'en-us'
time::between() //=> "tue, 16 oct 2018 10:48:27 am -05:00"
time::between() //=> "15 oct 10:48 am"
time::between() //=> "october 15, 2018 10:48 am"

i18n::locale = 'ja'
time::between() //=> "2018/10/15 10:48:27"
time::between() //=> "18/10/15 10:48"
time::between() //=> "2018年10月16日(火) 10時48分27秒 -0500"

time::between_dates() //=> "2014-09-19 07:03:30 -0700"
time::between_dates() //=> "2014-09-18 16:28:13 -0700"
time::between_dates() //=> "2014-09-20 19:39:38 -0700"
time::between_dates() //=> "2014-09-19 08:07:52 -0700"
time::between_dates() //=> "2014-09-18 12:10:34 -0700"
time::between_dates() //=> "2014-09-19 20:21:03 -0700"
time::between_dates() //=> "2014-09-20 00:40:14 -0700"
time::between_dates() //=> "fri, 19 oct 2018 15:17:46 -0500"

time::forward() // "2014-09-26 06:54:47 -0700"
time::forward() //=> "october 21, 2018 20:47"

time::backward() //=> "2014-09-17 19:56:33 -0700"
time::backward() //=> "14 oct 07:44"
```
