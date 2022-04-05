# Methods example

__users create__

```
curl -X 'POST' \
  'http://localhost:8080/api/v1/users/' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "username": "user_b",
  "firstName": "string",
  "password": "user_b"
}'
```

Response:

```
{"status":"created"}
```

Current api doesnt error when there are duplicates

__users list__

```
curl 'http://localhost:8080/api/v1/users'
```

Response:

```
[{"username":"user_b","firstName":"string","lastName":null,"email":null,"phone":null}]
```

__Login__

```
curl -X 'POST' \
  'http://localhost:8080/api/v1/login/' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "username": "string",
  "password": "string"
}'

```

Reponse:

```
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyX2IiLCJyb2xlIjoidXNlciIsImV4cCI6MTY0OTE4NTUzwd0.JbQFpkIFMSjUoyAWhpONyMj4NA2oek_JdM_uknS6vjA"}%
```

Error response 

```
{"error":"Invalid credentials"}%
```

# Db custom migrations

Add users index

```
db.members.createIndex( { "user_id": 1 }, { unique: true } )
```
