#!/bin/bash

export AWS_ACCESS_KEY_ID=ANTN35UAENTS5UIAEATD
export AWS_SECRET_ACCESS_KEY=TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur
export S3_ENDPOINT='http://localhost:9000'

./docker_test_run.py \
    --docker-image "ceph/demo" \
    --port=9000 \
    --run-opt=-p=9000:80 \
    --run-opt=--env=CEPH_DEMO_ACCESS_KEY=$AWS_ACCESS_KEY_ID \
    --run-opt=--env=CEPH_DEMO_SECRET_KEY=$AWS_SECRET_ACCESS_KEY \
    --run-opt=--env=CEPH_DEMO_UID=demo_uid \
    --run-opt=--env=MON_IP=127.0.0.1 \
    --run-opt=--env=CEPH_PUBLIC_NETWORK=0.0.0.0/0 \
    --run-opt=--hostname=localhost \
    -- cargo test --features s3,disable_ceph_unsupported
