# Use an official Go runtime as the base image
FROM golang:1.20-alpine

# Set the working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Go application
RUN go build -o poller

# Set the entry point of the container
ENTRYPOINT ["/app/poller"]