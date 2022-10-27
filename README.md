# CS361 Microservice

A simple stat block generator REST microservice for CS361.

## Usage

To get a stat block, send a GET request to `/req` with a `difficulty` integer value.
The server will return a `JSON` object with the calculated stats.

### Example

The following example uses `curl` and the microservice hosted at [cs361.moritamcvey.net/gen](https://cs361.moritamcvey.net/gen).

```sh
curl "https://cs361.moritamcvey.net/gen?difficulty=1"
```

returns:

```json
{ "difficulty": 1, "name": "Test Name", "health": 16, "damage": 138 }
```
