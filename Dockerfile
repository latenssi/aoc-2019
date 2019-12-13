FROM rust:1

WORKDIR /usr/src/myapp

RUN groupadd --gid 1000 mygroup \
    && useradd --uid 1000 --gid mygroup --shell /bin/bash myuser

ENV USER=myuser

USER myuser

CMD ["/bin/bash"]