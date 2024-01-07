# mysec

Перед использованием следует проконсультироваться со специалистом.


# postgres

```
openssl req -new -text -passout pass:abcd -subj /CN=localhost -out server.req -keyout privkey.pem
openssl rsa -in privkey.pem -passin pass:abcd -out server.key
openssl req -x509 -in server.req -text -key server.key -out server.crt
chmod 600 server.key
test $(uname -s) = Linux && chown 70 server.key
docker run -d --name postgres -e POSTGRES_HOST_AUTH_METHOD=trust -v "$(pwd)/server.crt:/var/lib/postgresql/server.crt:ro" -v "$(pwd)/server.key:/var/lib/postgresql/server.key:ro" postgres:12-alpine -c ssl=on -c ssl_cert_file=/var/lib/postgresql/server.crt -c ssl_key_file=/var/lib/postgresql/server.key
```

###

```
openssl req -new -text -subj /CN=localhost -out server.req -keyout privkey.pem
openssl rsa -in privkey.pem -out server.key
openssl req -x509 -in server.req -text -key server.key -out server.crt
```

###
# Создание самоподписанного сертификата

```
openssl genrsa -out myCA.key 2048
openssl req -x509 -new -key myCA.key -days 10000 -out myCA.crt
openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr
openssl x509 -req -in server.csr -CA ./myCA.crt -CAkey ./myCA.key -CAcreateserial -out server.crt -days 88000
openssl genrsa -out client.key 2048
openssl req -new -key client.key -out client.csr
openssl x509 -req -in client.csr -CA ./myCA.crt -CAkey ./myCA.key -CAcreateserial -out client.crt -days 88000
chmod 0600 ./server.*
chmod 0600 ./client.*
chmod 0600 ./myCA.*
chown postgres:postgres ./client.*
chown postgres:postgres ./server.*
chown postgres:postgres ./myCA.*

mv ./client.key ~/.postgresql/postgresql.key
mv ./client.csr ~/.postgresql/postgresql.csr
mv ./client.crt ~/.postgresql/postgresql.crt
cp ./myCA.crt ~/.postgresql/root.crt

psql -h localhost -U postgres mysec
```