a simple poise[^1] discord bot app for calculating stuff with libqalculate[^2]

### usage

the usage is simple, just add the [bot](https://discord.com/oauth2/authorize?client_id=1478034805236043828) into your discord apps and run it with the following command:

```
/calc 1 + 5
/calc 10 usd to brl
/calc 27 years to seconds
```

it will solve almost any formula, take a look at the library to know more about libqalculate[^2] features

### development

here we use knope[^3] for changeset control, every new feature must have a changeset[^4]

the deployment of the bot is made using [Github Actions](./.github/workflows/deploy.yaml)

[^1]: https://github.com/serenity-rs/poise
[^2]: https://github.com/Qalculate/libqalculate
[^3]: https://github.com/knope-dev/knope
[^4]: https://github.com/changesets/changesets
