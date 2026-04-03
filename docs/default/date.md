# faker::default::date

```rust
date::between() //=> #<date: 2014-09-24>
date::between() //=> #<date: 2014-09-24>

date::between_except() //=> #<date: 2014-10-03>
date::between_except() //=> #<date: 2014-10-03>

date::on_day_of_week_between() //=> "tue, 10 jan 2023"
date::on_day_of_week_between() //=> "sun, 22 jan 2023"
date::on_day_of_week_between() //=> "mon, 20 feb 2023"

date::forward() // "fri, 03 oct 2014"

date::backward() //=> "fri, 19 sep 2014"

date::birthday() //=> "fri, 28 mar 1986"

date::in_date_period() //=> #<date: 2019-09-01>

date::in_date_period() //=> #<date: 2018-02-26>

date::in_date_period() //=> #<date: 2019-02-26>

```
