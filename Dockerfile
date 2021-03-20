FROM ubuntu

#RUN sudo apt-get update
RUN apt-get update
RUN apt-get install -y ca-certificates
# RUN apt-get install libmysqlclient20
RUN apt-get install libgcc-s1
RUN apt-get install libpthread-stubs0-dev
RUN apt-get install libc6 libc6-dev
RUN apt-get install libstdc++6
ADD server /server
ADD entrypoint.sh /entrypoint.sh
WORKDIR /

EXPOSE 8080
ENTRYPOINT ["/entrypoint.sh"]

