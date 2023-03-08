FROM ghcr.io/ctron/trunk:main as builder

RUN mkdir /usr/src/project
COPY . /usr/src/project

WORKDIR /usr/src/project

#     --mount=type=cache,target=/opt/rust/registry,z \
#     --mount=type=cache,target=/opt/rust/git,z \
#     --mount=type=cache,target=/usr/src/project/target,z \

RUN \
    true \
    && npm install \
    && rm rust-toolchain.toml \
    && ls \
    && trunk build \
    && tar cvf /dist.tar dist/ \
    && mv dist /

#RUN find dist

FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

RUN microdnf -y install nginx

COPY --from=builder /dist /public
COPY nginx/nginx.conf /etc/nginx/nginx.conf

EXPOSE 8080

CMD ["/usr/sbin/nginx", "-g", "daemon off;"]
