# Builder ---------------------------------------------------------------------
FROM node:17-slim AS builder
WORKDIR /usr/src/bookclub-client

# Stage 1: install NPM dependencies (only runs when dependencies changed)
COPY package.json .
COPY package-lock.json .
RUN npm install

# Stage 2: copy code & build (only runs when code changed)
COPY . .
RUN npm run build

# Final image -----------------------------------------------------------------
FROM node:17-slim
WORKDIR /usr/local/bin/bookclub-client
COPY --from=builder /usr/src/bookclub-client/build build
COPY --from=builder /usr/src/bookclub-client/node_modules node_modules
COPY --from=builder /usr/src/bookclub-client/package.json .
CMD ["node", "build/index.js"]
