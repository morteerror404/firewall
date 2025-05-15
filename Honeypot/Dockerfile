FROM alpine:latest

# Instala serviços vulneráveis
RUN apk add --no-cache \
    openssh-server \
    vsftpd \
    telnetd

# Configuração do honeypot
COPY config/services.json /etc/honeypot/
COPY src/monitor /monitor

ENTRYPOINT ["/monitor"]
CMD ["--level", "high"]