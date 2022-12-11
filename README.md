# RRChat - Live chat and video #
## Built with React and Rust ##
--------------------------------
## Clone the Repo to your server machine ##
## Client ##
1. Open the "livelyChat" folder
2. install node `curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -`
3. run `sudo apt-get install nodejs -y`
4. install the modules `npm install`
5. open the .env file and change the IP address to be the ip address of your server (and leave port 8080 as it is)
6. run `npm run dev`
7. open the link provided to you on the browsers of your clients (laptop or mobile) and make sure it's on https
8. accept the warning in the browser

## Server ##
1. Open the "actix-websockets" folder
2. install rustup `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` and follow the instructions
4. update openssl `sudo apt install pkg-config libssl-dev`
3. run `cargo build` (this will take some time since there are a lot packages)
4. We need to run a temporary https page on the server's port (8080) and accept it from the broswer
5. To do this run `openssl s_server -key cert/key.pem -cert cert/cert.pem -accept 8080 -www`
6. In the browsers of your client devices visit <yourServerIp>:8080 and accept the warning : https://<yourServerIp>:8080
7. shut down the temp openssl server by hitting Ctrl + c
8. run the rust server `cargo run --release`
  
You may now enter a username and test the app
  
- Note : You have to allow the temp server from all client devices before testing.
- Note : The video will not work if the client device has no camera or mic (use a laptop or mobile not a vm)
