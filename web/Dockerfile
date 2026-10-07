# ROSE account website (sign up, forgot password, and game sign-in). Built by the
# repository's compose.yaml; see web/README.md.

FROM node:22-alpine AS build
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:22-alpine
WORKDIR /app
ENV NODE_ENV=production PORT=3001 HOSTNAME=0.0.0.0 DATA_DIR=/data NEXT_TELEMETRY_DISABLED=1
COPY --from=build /app/.next/standalone ./
COPY --from=build /app/.next/static ./.next/static
# Accounts and the token signing key. Keep this volume private and backed up.
RUN mkdir -p /data && chown node:node /data
USER node
VOLUME /data
EXPOSE 3001
HEALTHCHECK --interval=30s --timeout=5s CMD wget -q -O /dev/null http://127.0.0.1:3001/.well-known/openid-configuration || exit 1
CMD ["node", "--disable-warning=ExperimentalWarning", "server.js"]
