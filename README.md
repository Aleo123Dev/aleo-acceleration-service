# Aleo Wallet & Acceleration Service

## How to install

### 1.Install aleo-acceleration-service
- [Download](https://github.com/Aleo123Dev/aleo-acceleration-service/releases)  *I recommend selecting the latest released version and downloading the appropriate platform software*.

#### Mac
- you need to enable application from any source, drag the app to application folder, then run the following command:

```bash
sudo xattr -r -d com.apple.quarantine
sudo xattr -r -d com.apple.quarantine /Applications/aleo-acc-service.app
```

- Open aleo-acc-service.app <img width="110" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/796dceff-3638-4424-8ff6-0ec43986fc0b"> view:
<img width="800" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/84295aaf-1e8a-4abb-a063-b962bff95042">

#### Linux

1. Install
   
Install aleo-acc-service_x.x.x_amd64.AppImage Or aleo-acc-service_x.x.x_amd64.deb

2. Install from source code 

Install dependent library
`sudo apt updatesudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

```shell
git clone git@github.com:Aleo123Dev/aleo-acceleration-service.git
yarn
yarn tauri build
yarn tauri dev
```

#### Window

Install aleo-acc-service_0.0.x_x64_en-US.msi

### 2. Install Soter Wallet
- [Download](https://github.com/aleoweb123/soter_aleo_wallet/releases)  *I recommend selecting the latest released version*.

- Extract the v0.1.2.d.zip file, the dist/ directory is generated.
- Open chrome://extensions/
  <img width="1031" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/7b316dcd-3ca3-4a25-b4f3-267daccea056">
- Drag the dist/ folder onto the browser.

### 3. Configure the ACC service
- Default service address: http://127.0.0.1:18340
- You can customize acc service,like this:
  <img width="609" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/d7309727-200e-464a-a761-f34032a00269">

### 4. Now, when the method is executed, the local service will be called by default.

### 5. Test Result "Send"
  <img width="596" alt="image" src="https://github.com/Aleo123Dev/aleo-acceleration-service/assets/123852645/c1b255e4-1a65-44b6-bf70-8d680207176b">
