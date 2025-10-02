### Author 
* Name: Ndoda Kheswa
* Email: daniel.kheswa@gmail.com
* Role: EqualExperts Assessment Submission

### Project Title
A Rust REST API using the Axum web application framework 

### ---- Description ----
The project is strutured into different Rust modules consisting of model, application and api with a (handlers) sub-directory for processing network requests that are intercepted by the Axum framework.

Since Axum is a multi-threaded application, the AppState structure uses a smart pointer, Atomic Reference Counted (Arc) to manage ownership of data across multiple threads. It wraps the http client, which lives as long as the application runs and is responsible for making http calls to the GitHub API to retrieve user gists, which are non-blocking and asynchronous.

Axum leverages the Tokio runtime, which provides the necessary infrastructure for building concurrent and highly performant Rust applications. 

### ----- Instructions for running the application ----------------
### Build the docker image
`docker build -t gist-api-image .`

### run the docker image in detach mode and expose port 8080 on the host
`docker run -d -p 8080:8080 --name gist-api-container gist-api-image`

### ------- Running Tests -------

One integration test is included in tests/lib.rs, however, the final version of the image is built in release mode, meaning tests do not run as part of the image build for production. A test docker image can be build by adding the following line to run tests:

`RUN cargo test --verbose --workspace --all-features`

Once the docker container is running, the API can be tested by sending a get request as follows:
### replace the <USER> with a test user such as octocat

`curl -L -H "User-Agent: AxumRust" https://localhost:8080/<USER>`       




