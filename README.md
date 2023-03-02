# Todo
## Preset environment
  - cargo
  - rustup
  - maven cli
## Step
  - Clone repository.
    ```
    git clone https://github.com/UsQuake/rocket_coverage.git
    ```
  - Open repository directory.
    ```
    cd path/to/rocket/coverage/clone
    ```
  - Clone commons-collections repository under rocket coverage repository directory.
    ```
    git clone https://github.com/apache/commons-collections.git
    ```
  - Make a directory to store coverage reports.
    ```
    mkdir coverage_reports
    ```
  - File structure should be same with this screenshot.
    + ![](https://user-images.githubusercontent.com/24998577/222329780-a8610cf2-c2f3-42b4-bf95-6a1ce4e24123.png)
  - Run a script.
    ```
    cargo run commons-collections
    ```
  
