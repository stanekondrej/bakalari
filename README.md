# bakalari - a crate for the Bakaláři school system

This crate wraps the Bakaláři API, as specified in [this repo](https://github.com/bakalari-api/bakalari-api-v3).

## testing

To run the tests, you need to specify three environment variables:

- `BAKALARI_USERNAME`: The username of your Bakaláři account
- `BAKALARI_PASSWORD`: The password of your Bakaláři account
- `BAKALARI_BASE_URL`: The base URL of your Bakaláři instance

So, for example, this is what this would look like in Powershell:
```powershell
$env:BAKALARI_USERNAME = "myusername"; $env:BAKALARI_PASSWORD = "mypassword"; $env:BAKALARI_BASE_URL = "https://bakalari.school.tld";
```
