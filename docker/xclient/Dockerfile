FROM ubuntu

RUN apt update -y 
RUN apt install firefox x11-apps psmisc -y

ENV DISPLAY=xserver.lan.com:0

CMD [ "sleep", "40000" ]