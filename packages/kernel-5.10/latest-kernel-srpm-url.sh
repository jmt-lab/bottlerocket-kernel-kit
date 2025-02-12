#!/bin/sh
docker run --rm public.ecr.aws/amazonlinux/amazonlinux:2 sh -c 'amazon-linux-extras enable kernel-5.10 >/dev/null && yum install -q -y yum-utils && yumdownloader -q --source --urls kernel | grep ^http'
