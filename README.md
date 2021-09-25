# visits

Visiting a country on a tourist visa means you must obey some rules on how long
you can be there. For example, an American can be in the Schengen Zone for up
to 90 days in a 180 day period. These rules can vary as a Canadian can spend up
to 182 days in a 365 day period in the USA.

If you're making multiple trips and trying to figure out whether you're doing
it right this can be a bit of an anxiety-inducing task because getting in wrong
can have some pretty severe consequences.

I made this program to help with that and make it easier to answer some
questions that come up when planning repeated trips.

I also made it to learn more [Rust](https://www.rust-lang.org/) :sweat_smile:


## Setup

Currently you need to have Postgres setup and [`diesel_cli`]() installed.

```bash
# Setup the pg database
$ createdb <dbname>
$ echo 'DATABASE_URL=postgres://<username>:<password>@localhost/<dbname>' > .env

# Run the migrations to make the tables
$ diesel migration run

# Compile the program
$ cargo build --release

# After that it should now be living in ./target/release/visits
# You can move it somewhere else along with the .env file if you like!
$ ./target/release/visits --version
```

## Using it

There are a few subcommands you can use with the cli: `add`, `rm`, `ls`, and `next`.

All of these commands currently require a `-u` option for saying who you are.
There's no authentication yet, but I added this one here because I'd like to
turn this into a webapp in the future so many users can use it at once.

### `add`

```bash
# You can start adding visits using `add` like this (make sure to use yyyy-mm-dd as the format)
$ ./target/release/visits add -u liam 2019-3-22 2019-3-25

# You should get some nice output like this if it worked.
OK: Added!
+----+------------+------------+--------+
| Id | Entry      | Exit       | Length |
+====+============+============+========+
| 84 | 2019-03-22 | 2019-03-25 | 4      |
+----+------------+------------+--------+
```

### `rm`

```bash
# If you want to remove a visit you can use `rm` like this, where 84 is the id of the visit
$ ./target/release/visits rm -u liam 84
```

### `ls`

```
# To list out all the visits you have added you can use `ls` like this
$ ./target/release/visits ls -u liam

# When you do this you will get a nice table and some helpful information
# (By default it uses the 90 day max per 180 days rule)

OK: 6 visits found. (90/180 days per period)
+----+------------+------------+--------+---------------+
| Id | Entry      | Exit       | Length | Days leftover |
+====+============+============+========+===============+
| 80 | 2015-01-12 | 2015-02-12 | 32     | 58            |
+----+------------+------------+--------+---------------+
| 1  | 2018-01-01 | 2018-02-05 | 36     | 54            |
+----+------------+------------+--------+---------------+
| 6  | 2018-03-10 | 2018-03-15 | 6      | 48            |
+----+------------+------------+--------+---------------+
| 7  | 2018-04-24 | 2018-05-07 | 14     | 34            |
+----+------------+------------+--------+---------------+
| 8  | 2018-06-06 | 2018-06-22 | 17     | 17            |
+----+------------+------------+--------+---------------+
| 9  | 2018-07-18 | 2018-08-02 | 16     | 34            |
+----+------------+------------+--------+---------------+

# If you're interested in seeing the "Days leftover" for a different rule you
# can use the `--days` and `--period` options like this
$ ./target/release/visits ls -u liam --days 182 --period 365

OK: 6 visits found. (182/365 days per period)
+----+------------+------------+--------+---------------+
| Id | Entry      | Exit       | Length | Days leftover |
+====+============+============+========+===============+
| 80 | 2015-01-12 | 2015-02-12 | 32     | 150           |
+----+------------+------------+--------+---------------+
| 1  | 2018-01-01 | 2018-02-05 | 36     | 146           |
+----+------------+------------+--------+---------------+
| 6  | 2018-03-10 | 2018-03-15 | 6      | 140           |
+----+------------+------------+--------+---------------+
| 7  | 2018-04-24 | 2018-05-07 | 14     | 126           |
+----+------------+------------+--------+---------------+
| 8  | 2018-06-06 | 2018-06-22 | 17     | 109           |
+----+------------+------------+--------+---------------+
```

### `next`

```
# To answer the question, "when is the next time I can visit?" easily there is
# the `next` command. It uses the same `--days` and `--period` options as `ls`,
# but in addition to that it also takes a `--length` option if you want to visit 
# for a specific duration.
$ ./target/release/visits next -u liam --length 14

# Doing that you might get something like this after it finds the next possible visit
OK: Next possible visit found!
+------------+------------+--------+---------------+
| Entry      | Exit       | Length | Days from now |
+============+============+========+===============+
| 2019-04-13 | 2019-04-26 | 14     | 23            |
+------------+------------+--------+---------------+
```

## Contributing + Thanks :octocat:

PRs, issues, and code-reviews welcome, thanks!

I used a bunch of nice [crates](https://crates.io/) to make this, so props to them:
* [diesel](http://diesel.rs/)
* [structopt](https://github.com/TeXitoi/structopt)
* [chrono](https://github.com/chronotope/chrono)
* [prettytable-rs](https://github.com/phsym/prettytable-rs)
* + Others too in the `Cargo.toml` :heart:
