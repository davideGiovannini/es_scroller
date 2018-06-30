#!/bin/sh

until curl http://elasticsearch:9200;
do
    echo "Waiting for ElasticSearch to be available"
    sleep 5s
done

echo "ElasticSearch is ready"

curl -X PUT "elasticsearch:9200/twitter" -H 'Content-Type: application/json' -d'
{
    "settings" : {
        "index" : {
            "number_of_shards" : 3,
            "number_of_replicas" : 2
        }
    }
}
'
for id in $(seq 1 1000);
do
    curl -X PUT "elasticsearch:9200/twitter/_doc/$id" -H 'Content-Type: application/json' -d"
{
    \"user\" : \"$(head /dev/urandom | tr -dc A-Za-z0-9 | head -c 13)\",
    \"post_date\" : \"2009-11-15T14:12:12\",
    \"message\" : \"trying out Elasticsearch\"
}"
done

