server {
    server_name zurichbookclub.com;

    location / {
        proxy_pass http://127.0.0.1:3000;
    }

    location /api {
        proxy_pass http://127.0.0.1:8080;
    }

    gzip on;
    gzip_proxied any;
    gzip_types application/json;

    listen [::]:443 ssl http2;
    listen 443 ssl http2;
    ssl_certificate /etc/letsencrypt/live/zurichbookclub.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/zurichbookclub.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;
}

server {
    if ($host = zurichbookclub.com) {
        return 301 https://$host$request_uri;
    }

    listen 80;
    listen [::]:80;

    server_name zurichbookclub.com;
    return 404;
}
