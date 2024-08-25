# Axmax

Small app to poll an Axpert Max 2 inverter for data and upload to AWS Cloudwatch.

# Installation

Store AWS creds in local creds file.

```shell
cat ~/.aws/credentials
...
[default]
aws_access_key_id = AKIA...
aws_secret_access_key = ABC...
```

Install as crontab.

```shell
crontab -e
```

```shell
# m h  dom mon dow   command
* * * * * /path/to/axmax/target/release/axmax
```