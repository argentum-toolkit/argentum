FROM debian:stable
# FROM debian:stable-slim

WORKDIR /app

# Build-time argument - REQUIRED (no default)
ARG RELEASE_PATH

# Validate that RELEASE_PATH is set and the directory exists
RUN if [ -z "$RELEASE_PATH" ]; then \
    echo "Error: RELEASE_PATH is not set or does not exist: $RELEASE_PATH"; \
    exit 1; \
    fi

# Copy the application
COPY $RELEASE_PATH /app/

# # Runtime environment variables with defaults
# PORT an IP is used by Dioxus
ENV PORT=8080
ENV IP=0.0.0.0

# EXPOSE $PORT
CMD ["/app/server"]
