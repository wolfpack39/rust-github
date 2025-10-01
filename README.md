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

## :warning: Please read these instructions carefully and entirely first
* Clone this repository to your local machine.
* Use your IDE of choice to complete the assignment.
* When you have completed the assignment, you need to  push your code to this repository and [mark the assignment as completed by clicking here](https://app.snapcode.review/submission_links/850d33fb-68ac-4e3a-bc43-a486ee1bdb5a).
* Once you mark it as completed, your access to this repository will be revoked. Please make sure that you have completed the assignment and pushed all code from your local machine to this repository before you click the link.

## Operability Take-Home Exercise

Welcome to the start of our recruitment process for Operability Engineers. It was great to speak to you regarding an opportunity to join the Equal Experts network!

Please write code to deliver a solution to the problems outlined below.

We appreciate that your time is valuable and do not expect this exercise to **take more than 90 minutes**. If you think this exercise will take longer than that, I **strongly** encourage you to please get in touch to ask any clarifying questions.

### Submission guidelines
**Do**
- Provide a README file in text or markdown format that documents a concise way to set up and run the provided solution.
- Take the time to read any applicable API or service docs, it may save you significant effort.
- Make your solution simple and clear. We aren't looking for overly complex ways to solve the problem since in our experience, simple and clear solutions to problems are generally the most maintainable and extensible solutions.

**Don't**

Expect the reviewer to dedicate a machine to review the test by:

- Installing software globally that may conflict with system software
- Requiring changes to system-wide configurations
- Providing overly complex solutions that need to spin up a ton of unneeded supporting dependencies. We aspire to keep our dev experiences as simple as possible (but no simpler)!
- Include identifying information in your submission. We are endeavouring to make our review process anonymous to reduce bias.

### Exercise
If you have any questions on the below exercise, please do get in touch and we’ll answer as soon as possible.

#### Build an API, test it, and package it into a container
- Build a simple HTTP web server API in any general-purpose programming language[^1] that interacts with the GitHub API and responds to requests on `/<USER>` with a list of the user’s publicly available Gists[^2].
- Create an automated test to validate that your web server API works. An example user to use as test data is `octocat`.
- Package the web server API into a docker container that listens for requests on port `8080`. You do not need to publish the resulting container image in any container registry, but we are expecting the Dockerfile in the submission.
- The solution may optionally provide other functionality (e.g. pagination, caching) but the above **must** be implemented.

Best of luck,  
Equal Experts
__________________________________________
[^1]: For example Go, Python or Ruby but not Bash or Powershell.  
[^2]: https://docs.github.com/en/rest/gists/gists?apiVersion=2022-11-28




