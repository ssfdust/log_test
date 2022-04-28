FROM rockylinux:8.5

ENV LOG_PATH=/log/main.log LIFE_TIME=60
COPY ./target/rocky/release/log_test /usr/bin/log_test
USER 1000

CMD ["/usr/bin/log_test"]
