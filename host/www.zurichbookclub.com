server {
    server_name www.zurichbookclub.com;
    return 301 $scheme://zurichbookclub.com$request_uri;

    listen [::]:443 ssl http2;
    listen 443 ssl http2;
    ssl_certificate /etc/letsencrypt/live/zurichbookclub.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/zurichbookclub.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}

server {
    server_name www.zurichbookclub.com;
    return 301 $scheme://zurichbookclub.com$request_uri;

    listen 80;
    listen [::]:80;
}
