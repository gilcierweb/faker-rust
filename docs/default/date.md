# faker_rust::date

```rust
date::between("2014-09-01", "2014-09-30") //=> "2014-09-24"
date::between("2014-09-01", "2014-09-30") //=> "2014-09-24"

date::between_except("2014-09-01", "2014-09-30", "2014-09-15") //=> "2014-10-03"
date::between_except("2014-09-01", "2014-09-30", "2014-09-15") //=> "2014-10-03"

date::on_day_of_week_between("tue", "2023-01-01", "2023-12-31") //=> "tue, 10 jan 2023"
date::on_day_of_week_between("tue", "2023-01-01", "2023-12-31") //=> "sun, 22 jan 2023"
date::on_day_of_week_between("tue", "2023-01-01", "2023-12-31") //=> "mon, 20 feb 2023"

date::forward(None, None, None) // "fri, 03 oct 2014"

date::backward(None, None, None) //=> "fri, 19 sep 2014"

date::birthday(None, None) //=> "fri, 28 mar 1986"

date::in_date_period("2019-01-01", "2019-12-31", None) //=> "2019-09-01"

date::in_date_period("2019-01-01", "2019-12-31", None) //=> "2018-02-26"

date::in_date_period("2019-01-01", "2019-12-31", None) //=> "2019-02-26"

```
