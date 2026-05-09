# Static web image for the WASM build of the gravity simulation. The WASM
# artifact is built by GitHub Actions before this Dockerfile runs (so the
# Rust toolchain doesn't need to be inside the runtime image). The CI step
# produces ./web/index.html and ./web/pkg/ which we copy in here.

FROM nginx:1.27-alpine

# nginx by default serves /usr/share/nginx/html.
COPY web/ /usr/share/nginx/html/

# .wasm needs the right MIME type; nginx:alpine's default already maps it,
# but make it explicit.
RUN printf '%s\n' \
    'types {' \
    '  application/wasm wasm;' \
    '}' \
    > /etc/nginx/conf.d/wasm-mime.conf

EXPOSE 80
